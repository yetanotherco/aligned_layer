package pkg

import (
	"context"
	"encoding/hex"
	gethtypes "github.com/ethereum/go-ethereum/core/types"
	"sync"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/yetanotherco/aligned_layer/metrics"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	sdkclients "github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/services/avsregistry"
	blsagg "github.com/Layr-Labs/eigensdk-go/services/bls_aggregation"
	oppubkeysserv "github.com/Layr-Labs/eigensdk-go/services/operatorpubkeys"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/ethereum/go-ethereum/event"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/core/types"
	"github.com/yetanotherco/aligned_layer/core/utils"
)

// FIXME(marian): Read this from Aligned contract directly
const QUORUM_NUMBER = byte(0)
const QUORUM_THRESHOLD = byte(67)

// Aggregator stores TaskResponse for a task here
type TaskResponses = []types.SignedTaskResponse

type Aggregator struct {
	AggregatorConfig      *config.AggregatorConfig
	NewBatchChan          chan *servicemanager.ContractAlignedLayerServiceManagerNewBatch
	avsReader             *chainio.AvsReader
	avsSubscriber         *chainio.AvsSubscriber
	avsWriter             *chainio.AvsWriter
	taskSubscriber        event.Subscription
	blsAggregationService blsagg.BlsAggregationService

	// BLS Signature Service returns an Index
	// Since our ID is not an idx, we build this cache
	// Note: In case of a reboot, this doesn't need to be loaded,
	// and can start from zero
	batchesRootByIdx map[uint32][32]byte

	// This is the counterpart,
	// to use when we have the batch but not the index
	// Note: In case of a reboot, this doesn't need to be loaded,
	// and can start from zero
	batchesIdxByRoot map[[32]byte]uint32

	// This task index is to communicate with the local BLS
	// Service.
	// Note: In case of a reboot it can start from 0 again
	nextBatchIndex uint32

	// Mutex to protect batchesRootByIdx, batchesIdxByRoot and nextBatchIndex
	taskMutex *sync.Mutex

	// Mutex to protect ethereum wallet
	walletMutex *sync.Mutex

	logger logging.Logger

	metricsReg *prometheus.Registry
	metrics    *metrics.Metrics
}

func NewAggregator(aggregatorConfig config.AggregatorConfig) (*Aggregator, error) {
	newBatchChan := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewBatch)

	avsReader, err := chainio.NewAvsReaderFromConfig(aggregatorConfig.BaseConfig, aggregatorConfig.EcdsaConfig)
	if err != nil {
		return nil, err
	}

	avsSubscriber, err := chainio.NewAvsSubscriberFromConfig(aggregatorConfig.BaseConfig)
	if err != nil {
		return nil, err
	}

	avsWriter, err := chainio.NewAvsWriterFromConfig(aggregatorConfig.BaseConfig, aggregatorConfig.EcdsaConfig)
	if err != nil {
		return nil, err
	}

	batchesRootByIdx := make(map[uint32][32]byte)
	batchesIdxByRoot := make(map[[32]byte]uint32)

	chainioConfig := sdkclients.BuildAllConfig{
		EthHttpUrl:                 aggregatorConfig.BaseConfig.EthRpcUrl,
		EthWsUrl:                   aggregatorConfig.BaseConfig.EthWsUrl,
		RegistryCoordinatorAddr:    aggregatorConfig.BaseConfig.AlignedLayerDeploymentConfig.AlignedLayerRegistryCoordinatorAddr.Hex(),
		OperatorStateRetrieverAddr: aggregatorConfig.BaseConfig.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr.Hex(),
		AvsName:                    "AlignedLayer",
		PromMetricsIpPortAddress:   ":9090",
	}

	aggregatorPrivateKey := aggregatorConfig.EcdsaConfig.PrivateKey

	logger := aggregatorConfig.BaseConfig.Logger
	clients, err := clients.BuildAll(chainioConfig, aggregatorPrivateKey, logger)
	if err != nil {
		logger.Errorf("Cannot create sdk clients", "err", err)
		return nil, err
	}

	operatorPubkeysService := oppubkeysserv.NewOperatorPubkeysServiceInMemory(context.Background(), clients.AvsRegistryChainSubscriber, clients.AvsRegistryChainReader, logger)
	avsRegistryService := avsregistry.NewAvsRegistryServiceChainCaller(avsReader.AvsRegistryReader, operatorPubkeysService, logger)
	blsAggregationService := blsagg.NewBlsAggregatorService(avsRegistryService, logger)

	// Metrics
	reg := prometheus.NewRegistry()
	aggregatorMetrics := metrics.NewMetrics(aggregatorConfig.Aggregator.MetricsIpPortAddress, reg, logger)

	nextBatchIndex := uint32(0)

	aggregator := Aggregator{
		AggregatorConfig: &aggregatorConfig,
		avsReader:        avsReader,
		avsSubscriber:    avsSubscriber,
		avsWriter:        avsWriter,
		NewBatchChan:     newBatchChan,

		batchesRootByIdx: batchesRootByIdx,
		batchesIdxByRoot: batchesIdxByRoot,
		nextBatchIndex:   nextBatchIndex,
		taskMutex:        &sync.Mutex{},
		walletMutex:      &sync.Mutex{},

		blsAggregationService: blsAggregationService,
		logger:                logger,
		metricsReg:            reg,
		metrics:               aggregatorMetrics,
	}

	return &aggregator, nil
}

func (agg *Aggregator) Start(ctx context.Context) error {
	agg.logger.Infof("Starting aggregator...")

	go func() {
		err := agg.ServeOperators()
		if err != nil {
			agg.logger.Fatal("Error listening for tasks", "err", err)
		}
	}()

	var metricsErrChan <-chan error
	if agg.AggregatorConfig.Aggregator.EnableMetrics {
		metricsErrChan = agg.metrics.Start(ctx, agg.metricsReg)
	} else {
		metricsErrChan = make(chan error, 1)
	}

	for {
		select {
		case <-ctx.Done():
			return nil
		case err := <-metricsErrChan:
			agg.logger.Fatal("Metrics server failed", "err", err)
		case blsAggServiceResp := <-agg.blsAggregationService.GetResponseChannel():
			agg.logger.Info("Received response from BLS aggregation service",
				"taskIndex", blsAggServiceResp.TaskIndex)

			go agg.handleBlsAggServiceResponse(blsAggServiceResp)
		}
	}
}

const MaxSentTxRetries = 5

func (agg *Aggregator) handleBlsAggServiceResponse(blsAggServiceResp blsagg.BlsAggregationServiceResponse) {
	if blsAggServiceResp.Err != nil {
		agg.logger.Warn("BlsAggregationServiceResponse contains an error", "err", blsAggServiceResp.Err)
		return
	}

	nonSignerPubkeys := []servicemanager.BN254G1Point{}
	for _, nonSignerPubkey := range blsAggServiceResp.NonSignersPubkeysG1 {
		nonSignerPubkeys = append(nonSignerPubkeys, utils.ConvertToBN254G1Point(nonSignerPubkey))
	}
	quorumApks := []servicemanager.BN254G1Point{}
	for _, quorumApk := range blsAggServiceResp.QuorumApksG1 {
		quorumApks = append(quorumApks, utils.ConvertToBN254G1Point(quorumApk))
	}

	nonSignerStakesAndSignature := servicemanager.IBLSSignatureCheckerNonSignerStakesAndSignature{
		NonSignerPubkeys:             nonSignerPubkeys,
		QuorumApks:                   quorumApks,
		ApkG2:                        utils.ConvertToBN254G2Point(blsAggServiceResp.SignersApkG2),
		Sigma:                        utils.ConvertToBN254G1Point(blsAggServiceResp.SignersAggSigG1.G1Point),
		NonSignerQuorumBitmapIndices: blsAggServiceResp.NonSignerQuorumBitmapIndices,
		QuorumApkIndices:             blsAggServiceResp.QuorumApkIndices,
		TotalStakeIndices:            blsAggServiceResp.TotalStakeIndices,
		NonSignerStakeIndices:        blsAggServiceResp.NonSignerStakeIndices,
	}

	agg.taskMutex.Lock()
	agg.AggregatorConfig.BaseConfig.Logger.Info("- Locked Resources: Fetching merkle root")
	batchMerkleRoot := agg.batchesRootByIdx[blsAggServiceResp.TaskIndex]
	agg.AggregatorConfig.BaseConfig.Logger.Info("- Unlocked Resources: Fetching merkle root")
	agg.taskMutex.Unlock()

	agg.logger.Info("Threshold reached. Sending aggregated response onchain.",
		"taskIndex", blsAggServiceResp.TaskIndex,
		"merkleRoot", hex.EncodeToString(batchMerkleRoot[:]))

	var err error

	for i := 0; i < MaxSentTxRetries; i++ {
		_, err = agg.sendAggregatedResponse(batchMerkleRoot, nonSignerStakesAndSignature)
		if err == nil {
			agg.logger.Info("Aggregator successfully responded to task",
				"taskIndex", blsAggServiceResp.TaskIndex,
				"merkleRoot", hex.EncodeToString(batchMerkleRoot[:]))

			return
		}

		// Sleep for a bit before retrying
		time.Sleep(2 * time.Second)
	}

	agg.logger.Error("Aggregator failed to respond to task, this batch will be lost",
		"err", err,
		"taskIndex", blsAggServiceResp.TaskIndex,
		"merkleRoot", hex.EncodeToString(batchMerkleRoot[:]))
}



/// Sends response to contract and waits for transaction receipt
/// Returns error if it fails to send tx or receipt is not found
func (agg *Aggregator) sendAggregatedResponse(batchMerkleRoot [32]byte, nonSignerStakesAndSignature servicemanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*gethtypes.Receipt, error) {
	agg.walletMutex.Lock()
	agg.logger.Infof("- Locked Wallet Resources: Sending aggregated response for batch %s", hex.EncodeToString(batchMerkleRoot[:]))

	txHash, err := agg.avsWriter.SendAggregatedResponse(batchMerkleRoot, nonSignerStakesAndSignature)
	if err != nil {
		agg.walletMutex.Unlock()
		agg.logger.Infof("- Unlocked Wallet Resources: Error sending aggregated response for batch %s. Error: %s", hex.EncodeToString(batchMerkleRoot[:]), err)
		return nil, err
	}

	agg.walletMutex.Unlock()
	agg.logger.Infof("- Unlocked Wallet Resources: Sending aggregated response for batch %s", hex.EncodeToString(batchMerkleRoot[:]))

	receipt, err := utils.WaitForTransactionReceipt(
		agg.AggregatorConfig.BaseConfig.EthRpcClient, context.Background(), *txHash)
	if err != nil {
		return nil, err
	}

	agg.metrics.IncAggregatedResponses()

	return receipt, nil
}


func (agg *Aggregator) AddNewTask(batchMerkleRoot [32]byte, taskCreatedBlock uint32) {
	agg.AggregatorConfig.BaseConfig.Logger.Info("Adding new task",
		"Batch merkle root", hex.EncodeToString(batchMerkleRoot[:]))

	agg.taskMutex.Lock()
	agg.AggregatorConfig.BaseConfig.Logger.Info("- Locked Resources: Adding new task")

	// --- UPDATE BATCH - INDEX CACHES ---
	batchIndex := agg.nextBatchIndex
	if _, ok := agg.batchesIdxByRoot[batchMerkleRoot]; ok {
		agg.logger.Warn("Batch already exists", "batchIndex", batchIndex, "batchRoot", batchMerkleRoot)
		agg.taskMutex.Unlock()
		agg.AggregatorConfig.BaseConfig.Logger.Info("- Unlocked Resources: Adding new task")
		return
	}

	// This shouldn't happen, since both maps are updated together
	if _, ok := agg.batchesRootByIdx[batchIndex]; ok {
		agg.logger.Warn("Batch already exists", "batchIndex", batchIndex, "batchRoot", batchMerkleRoot)
		agg.taskMutex.Unlock()
		agg.AggregatorConfig.BaseConfig.Logger.Info("- Unlocked Resources: Adding new task")
		return
	}

	agg.batchesIdxByRoot[batchMerkleRoot] = batchIndex
	agg.batchesRootByIdx[batchIndex] = batchMerkleRoot
	agg.nextBatchIndex += 1

	quorumNums := eigentypes.QuorumNums{eigentypes.QuorumNum(QUORUM_NUMBER)}
	quorumThresholdPercentages := eigentypes.QuorumThresholdPercentages{eigentypes.QuorumThresholdPercentage(QUORUM_THRESHOLD)}

	err := agg.blsAggregationService.InitializeNewTask(batchIndex, taskCreatedBlock, quorumNums, quorumThresholdPercentages, 100*time.Second)
	// FIXME(marian): When this errors, should we retry initializing new task? Logging fatal for now.
	if err != nil {
		agg.logger.Fatalf("BLS aggregation service error when initializing new task: %s", err)
	}

	agg.taskMutex.Unlock()
	agg.AggregatorConfig.BaseConfig.Logger.Info("- Unlocked Resources: Adding new task")
	agg.logger.Info("New task added", "batchIndex", batchIndex, "batchMerkleRoot", hex.EncodeToString(batchMerkleRoot[:]))
}
