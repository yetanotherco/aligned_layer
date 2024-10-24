package pkg

import (
	"context"
	"encoding/hex"
	"fmt"
	"sync"
	"time"

	gethtypes "github.com/ethereum/go-ethereum/core/types"

	"github.com/prometheus/client_golang/prometheus"
	connection "github.com/yetanotherco/aligned_layer/core"
	"github.com/yetanotherco/aligned_layer/metrics"

	sdkclients "github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/services/avsregistry"
	blsagg "github.com/Layr-Labs/eigensdk-go/services/bls_aggregation"
	oppubkeysserv "github.com/Layr-Labs/eigensdk-go/services/operatorsinfo"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/ethereum/go-ethereum/crypto"
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

// BatchData stores the data of a batch, for use in map BatchIdentifierHash -> BatchData
type BatchData struct {
	BatchMerkleRoot [32]byte
	SenderAddress   [20]byte
}

type Aggregator struct {
	AggregatorConfig      *config.AggregatorConfig
	NewBatchChan          chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3
	avsReader             *chainio.AvsReader
	avsSubscriber         *chainio.AvsSubscriber
	avsWriter             *chainio.AvsWriter
	taskSubscriber        chan error
	blsAggregationService blsagg.BlsAggregationService

	// BLS Signature Service returns an Index
	// Since our ID is not an idx, we build this cache
	// Note: In case of a reboot, this doesn't need to be loaded,
	// and can start from zero
	batchesIdentifierHashByIdx map[uint32][32]byte

	// This is the counterpart,
	// to use when we have the batch but not the index
	// Note: In case of a reboot, this doesn't need to be loaded,
	// and can start from zero
	batchesIdxByIdentifierHash map[[32]byte]uint32

	// Stores the taskCreatedBlock for each batch bt batch index
	batchCreatedBlockByIdx map[uint32]uint64

	// Stores the TaskResponse for each batch by batchIdentifierHash
	batchDataByIdentifierHash map[[32]byte]BatchData

	// This task index is to communicate with the local BLS
	// Service.
	// Note: In case of a reboot it can start from 0 again
	nextBatchIndex uint32

	// Mutex to protect batchesIdentifierHashByIdx, batchesIdxByIdentifierHash and nextBatchIndex
	taskMutex *sync.Mutex

	// Mutex to protect ethereum wallet
	walletMutex *sync.Mutex

	logger logging.Logger

	// Metrics
	metricsReg *prometheus.Registry
	metrics    *metrics.Metrics

	// Telemetry
	telemetry *Telemetry
}

func NewAggregator(aggregatorConfig config.AggregatorConfig) (*Aggregator, error) {
	newBatchChan := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3)

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

	batchesIdentifierHashByIdx := make(map[uint32][32]byte)
	batchesIdxByIdentifierHash := make(map[[32]byte]uint32)
	batchDataByIdentifierHash := make(map[[32]byte]BatchData)
	batchCreatedBlockByIdx := make(map[uint32]uint64)

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
	clients, err := sdkclients.BuildAll(chainioConfig, aggregatorPrivateKey, logger)
	if err != nil {
		logger.Errorf("Cannot create sdk clients", "err", err)
		return nil, err
	}

	// This is a dummy "hash function" made to fulfill the BLS aggregator service API requirements.
	// When operators respond to a task, a call to `ProcessNewSignature` is made. In `v0.1.6` of the eigensdk,
	// this function required an argument `TaskResponseDigest`, which has changed to just `TaskResponse` in v0.1.9.
	// The digest we used in v0.1.6 was just the batch merkle root. To continue with the same idea, the hashing
	// function is set as the following one, which does nothing more than output the input it receives, which in
	// our case will be the batch merkle root. If wanted, we could define a real hash function here but there should
	// not be any need to re-hash the batch merkle root.
	hashFunction := func(taskResponse eigentypes.TaskResponse) (eigentypes.TaskResponseDigest, error) {
		taskResponseDigest, ok := taskResponse.([32]byte)
		if !ok {
			return eigentypes.TaskResponseDigest{}, fmt.Errorf("TaskResponse is not a 32-byte value")
		}
		return taskResponseDigest, nil
	}

	operatorPubkeysService := oppubkeysserv.NewOperatorsInfoServiceInMemory(context.Background(), clients.AvsRegistryChainSubscriber, clients.AvsRegistryChainReader, nil, oppubkeysserv.Opts{}, logger)
	avsRegistryService := avsregistry.NewAvsRegistryServiceChainCaller(avsReader.ChainReader, operatorPubkeysService, logger)
	blsAggregationService := blsagg.NewBlsAggregatorService(avsRegistryService, hashFunction, logger)

	// Metrics
	reg := prometheus.NewRegistry()
	aggregatorMetrics := metrics.NewMetrics(aggregatorConfig.Aggregator.MetricsIpPortAddress, reg, logger)

	// Telemetry
	aggregatorTelemetry := NewTelemetry(aggregatorConfig.Aggregator.TelemetryIpPortAddress, logger)

	nextBatchIndex := uint32(0)

	aggregator := Aggregator{
		AggregatorConfig: &aggregatorConfig,
		avsReader:        avsReader,
		avsSubscriber:    avsSubscriber,
		avsWriter:        avsWriter,
		NewBatchChan:     newBatchChan,

		batchesIdentifierHashByIdx: batchesIdentifierHashByIdx,
		batchesIdxByIdentifierHash: batchesIdxByIdentifierHash,
		batchDataByIdentifierHash:  batchDataByIdentifierHash,
		batchCreatedBlockByIdx:     batchCreatedBlockByIdx,
		nextBatchIndex:             nextBatchIndex,
		taskMutex:                  &sync.Mutex{},
		walletMutex:                &sync.Mutex{},

		blsAggregationService: blsAggregationService,
		logger:                logger,
		metricsReg:            reg,
		metrics:               aggregatorMetrics,
		telemetry:             aggregatorTelemetry,
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
	agg.taskMutex.Lock()
	agg.AggregatorConfig.BaseConfig.Logger.Info("- Locked Resources: Fetching task data")
	batchIdentifierHash := agg.batchesIdentifierHashByIdx[blsAggServiceResp.TaskIndex]
	batchData := agg.batchDataByIdentifierHash[batchIdentifierHash]
	taskCreatedBlock := agg.batchCreatedBlockByIdx[blsAggServiceResp.TaskIndex]
	agg.taskMutex.Unlock()
	agg.AggregatorConfig.BaseConfig.Logger.Info("- Unlocked Resources: Fetching task data")

	// Finish task trace once the task is processed (either successfully or not)
	defer agg.telemetry.FinishTrace(batchData.BatchMerkleRoot)

	if blsAggServiceResp.Err != nil {
		agg.telemetry.LogTaskError(batchData.BatchMerkleRoot, blsAggServiceResp.Err)
		agg.logger.Error("BlsAggregationServiceResponse contains an error", "err", blsAggServiceResp.Err, "batchIdentifierHash", hex.EncodeToString(batchIdentifierHash[:]))
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

	agg.telemetry.LogQuorumReached(batchData.BatchMerkleRoot)

	agg.logger.Info("Threshold reached", "taskIndex", blsAggServiceResp.TaskIndex,
		"batchIdentifierHash", "0x"+hex.EncodeToString(batchIdentifierHash[:]))

	agg.logger.Info("Maybe waiting one block to send aggregated response onchain",
		"taskIndex", blsAggServiceResp.TaskIndex,
		"batchIdentifierHash", "0x"+hex.EncodeToString(batchIdentifierHash[:]),
		"taskCreatedBlock", taskCreatedBlock)

	err := agg.avsSubscriber.WaitForOneBlock(taskCreatedBlock)
	if err != nil {
		agg.logger.Error("Error waiting for one block, sending anyway", "err", err)
	}

	agg.logger.Info("Sending aggregated response onchain", "taskIndex", blsAggServiceResp.TaskIndex,
		"batchIdentifierHash", "0x"+hex.EncodeToString(batchIdentifierHash[:]))

	_, err = agg.sendAggregatedResponse(batchIdentifierHash, batchData.BatchMerkleRoot, batchData.SenderAddress, nonSignerStakesAndSignature)
	if err == nil {
		agg.logger.Info("Aggregator successfully responded to task",
			"taskIndex", blsAggServiceResp.TaskIndex,
			"batchIdentifierHash", "0x"+hex.EncodeToString(batchIdentifierHash[:]))
		return
	}

	agg.logger.Error("Aggregator failed to respond to task, this batch will be lost",
		"err", err,
		"taskIndex", blsAggServiceResp.TaskIndex,
		"merkleRoot", "0x"+hex.EncodeToString(batchData.BatchMerkleRoot[:]),
		"senderAddress", "0x"+hex.EncodeToString(batchData.SenderAddress[:]),
		"batchIdentifierHash", "0x"+hex.EncodeToString(batchIdentifierHash[:]))
	agg.telemetry.LogTaskError(batchData.BatchMerkleRoot, err)
}

// TODO(pat): should we be retrying the entire Send -> Receive Receipt step? For bumping the gas fee this is needed.

// / Sends response to contract and waits for transaction receipt
// / Returns error if it fails to send tx or receipt is not found
func (agg *Aggregator) sendAggregatedResponse(batchIdentifierHash [32]byte, batchMerkleRoot [32]byte, senderAddress [20]byte, nonSignerStakesAndSignature servicemanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*gethtypes.Receipt, error) {

	agg.walletMutex.Lock()
	agg.logger.Infof("- Locked Wallet Resources: Sending aggregated response for batch",
		"merkleRoot", hex.EncodeToString(batchMerkleRoot[:]),
		"senderAddress", hex.EncodeToString(senderAddress[:]),
		"batchIdentifierHash", hex.EncodeToString(batchIdentifierHash[:]))

	// Retry
	txHash, err := agg.avsWriter.SendAggregatedResponse(batchIdentifierHash, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature)
	if err != nil {
		agg.walletMutex.Unlock()
		agg.logger.Infof("- Unlocked Wallet Resources: Error sending aggregated response for batch %s. Error: %s", hex.EncodeToString(batchIdentifierHash[:]), err)
		agg.telemetry.LogTaskError(batchMerkleRoot, err)
		return nil, err
	}

	agg.walletMutex.Unlock()
	agg.logger.Infof("- Unlocked Wallet Resources: Sending aggregated response for batch %s", hex.EncodeToString(batchIdentifierHash[:]))

	// Retry
	receipt, err := utils.WaitForTransactionReceiptRetryable(
		agg.AggregatorConfig.BaseConfig.EthRpcClient, context.Background(), *txHash)
	if err != nil {
		agg.telemetry.LogTaskError(batchMerkleRoot, err)
		return nil, err
	}

	agg.metrics.IncAggregatedResponses()

	return receipt, nil
}

func (agg *Aggregator) AddNewTask(batchMerkleRoot [32]byte, senderAddress [20]byte, taskCreatedBlock uint32) {
	agg.telemetry.InitNewTrace(batchMerkleRoot)
	batchIdentifier := append(batchMerkleRoot[:], senderAddress[:]...)
	var batchIdentifierHash = *(*[32]byte)(crypto.Keccak256(batchIdentifier))

	agg.AggregatorConfig.BaseConfig.Logger.Info("Adding new task",
		"Batch merkle root", "0x"+hex.EncodeToString(batchMerkleRoot[:]),
		"Sender Address", "0x"+hex.EncodeToString(senderAddress[:]),
		"batchIdentifierHash", "0x"+hex.EncodeToString(batchIdentifierHash[:]))

	agg.taskMutex.Lock()
	agg.AggregatorConfig.BaseConfig.Logger.Info("- Locked Resources: Adding new task")

	// --- UPDATE BATCH - INDEX CACHES ---
	batchIndex := agg.nextBatchIndex
	if _, ok := agg.batchesIdxByIdentifierHash[batchIdentifierHash]; ok {
		agg.logger.Warn("Batch already exists", "batchIndex", batchIndex, "batchIdentifierHash", batchIdentifierHash)
		agg.taskMutex.Unlock()
		agg.AggregatorConfig.BaseConfig.Logger.Info("- Unlocked Resources: Adding new task")
		return
	}

	// This shouldn't happen, since both maps are updated together
	if _, ok := agg.batchesIdentifierHashByIdx[batchIndex]; ok {
		agg.logger.Warn("Batch already exists", "batchIndex", batchIndex, "batchIdentifierHash", batchIdentifierHash)
		agg.taskMutex.Unlock()
		agg.AggregatorConfig.BaseConfig.Logger.Info("- Unlocked Resources: Adding new task")
		return
	}

	agg.batchesIdxByIdentifierHash[batchIdentifierHash] = batchIndex
	agg.batchCreatedBlockByIdx[batchIndex] = uint64(taskCreatedBlock)
	agg.batchesIdentifierHashByIdx[batchIndex] = batchIdentifierHash
	agg.batchDataByIdentifierHash[batchIdentifierHash] = BatchData{
		BatchMerkleRoot: batchMerkleRoot,
		SenderAddress:   senderAddress,
	}
	agg.nextBatchIndex += 1

	quorumNums := eigentypes.QuorumNums{eigentypes.QuorumNum(QUORUM_NUMBER)}
	quorumThresholdPercentages := eigentypes.QuorumThresholdPercentages{eigentypes.QuorumThresholdPercentage(QUORUM_THRESHOLD)}

	// Retry
	err := agg.InitializeNewTaskRetryable(batchIndex, taskCreatedBlock, quorumNums, quorumThresholdPercentages, 100*time.Second)
	// FIXME(marian): When this errors, should we retry initializing new task? Logging fatal for now.
	if err != nil {
		agg.logger.Fatalf("BLS aggregation service error when initializing new task: %s", err)
	}

	agg.metrics.IncAggregatorReceivedTasks()
	agg.taskMutex.Unlock()
	agg.AggregatorConfig.BaseConfig.Logger.Info("- Unlocked Resources: Adding new task")
	agg.logger.Info("New task added", "batchIndex", batchIndex, "batchIdentifierHash", "0x"+hex.EncodeToString(batchIdentifierHash[:]))
}

// |---RETRYABLE---|

func (agg *Aggregator) InitializeNewTaskRetryable(batchIndex uint32, taskCreatedBlock uint32, quorumNums eigentypes.QuorumNums, quorumThresholdPercentages eigentypes.QuorumThresholdPercentages, timeToExpiry time.Duration) error {
	initilizeNewTask_func := func() error {
		// Returns an error if the
		return agg.blsAggregationService.InitializeNewTask(batchIndex, taskCreatedBlock, quorumNums, quorumThresholdPercentages, 100*time.Second)
	}
	return connection.Retry(initilizeNewTask_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
}
