package operator

import (
	"bytes"
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"log"
	"math/big"
	"net/http"
	"os"
	"path/filepath"
	"sync"
	"time"

	"github.com/ethereum/go-ethereum/crypto"
	"github.com/urfave/cli/v2"
	"github.com/yetanotherco/aligned_layer/operator/risc_zero"
	"github.com/yetanotherco/aligned_layer/operator/risc_zero_old"
	"golang.org/x/crypto/sha3"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/yetanotherco/aligned_layer/metrics"

	"github.com/yetanotherco/aligned_layer/operator/sp1"
	"github.com/yetanotherco/aligned_layer/operator/sp1_old"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	"github.com/Layr-Labs/eigensdk-go/logging"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/backend/plonk"
	"github.com/consensys/gnark/backend/witness"
	ethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/yetanotherco/aligned_layer/common"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/types"

	"github.com/yetanotherco/aligned_layer/core/config"
)

type Operator struct {
	Config                    config.OperatorConfig
	Address                   ethcommon.Address
	Socket                    string
	Timeout                   time.Duration
	KeyPair                   *bls.KeyPair
	OperatorId                eigentypes.OperatorId
	avsSubscriber             chainio.AvsSubscriber
	avsReader                 chainio.AvsReader
	NewTaskCreatedChanV2      chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2
	NewTaskCreatedChanV3      chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3
	Logger                    logging.Logger
	aggRpcClient              AggregatorRpcClient
	metricsReg                *prometheus.Registry
	metrics                   *metrics.Metrics
	lastProcessedBatch        OperatorLastProcessedBatch
	lastProcessedBatchLogFile string
	//Socket  string
	//Timeout time.Duration
}

const (
	BatchDownloadTimeout    = 1 * time.Minute
	BatchDownloadMaxRetries = 3
	BatchDownloadRetryDelay = 5 * time.Second
	UnverifiedBatchOffset   = 100
)

func NewOperatorFromConfig(configuration config.OperatorConfig) (*Operator, error) {
	logger := configuration.BaseConfig.Logger

	avsReader, err := chainio.NewAvsReaderFromConfig(configuration.BaseConfig)
	if err != nil {
		log.Fatalf("Could not create AVS reader")
	}

	registered, err := avsReader.IsOperatorRegistered(configuration.Operator.Address)
	if err != nil {
		log.Fatalf("Could not check if operator is registered")
	}
	if !registered {
		log.Fatal("Operator not registered")
	}

	avsSubscriber, err := chainio.NewAvsSubscriberFromConfig(configuration.BaseConfig)
	if err != nil {
		log.Fatalf("Could not create AVS subscriber")
	}
	newTaskCreatedChanV2 := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2)
	newTaskCreatedChanV3 := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3)

	rpcClient, err := NewAggregatorRpcClient(configuration.Operator.AggregatorServerIpPortAddress, logger)
	if err != nil {
		return nil, fmt.Errorf("could not create RPC client: %s. Is aggregator running?", err)
	}

	operatorId := eigentypes.OperatorIdFromKeyPair(configuration.BlsConfig.KeyPair)
	address := configuration.Operator.Address
	lastProcessedBatchLogFile := configuration.Operator.LastProcessedBatchFilePath

	if lastProcessedBatchLogFile == "" {
		logger.Fatalf("Config file field: `last_processed_batch_filepath` not provided.")
	}

	// Metrics
	reg := prometheus.NewRegistry()
	operatorMetrics := metrics.NewMetrics(configuration.Operator.MetricsIpPortAddress, reg, logger)

	operator := &Operator{
		Config:                    configuration,
		Logger:                    logger,
		avsSubscriber:             *avsSubscriber,
		avsReader:                 *avsReader,
		Address:                   address,
		NewTaskCreatedChanV2:      newTaskCreatedChanV2,
		NewTaskCreatedChanV3:      newTaskCreatedChanV3,
		aggRpcClient:              *rpcClient,
		OperatorId:                operatorId,
		metricsReg:                reg,
		metrics:                   operatorMetrics,
		lastProcessedBatchLogFile: lastProcessedBatchLogFile,
		lastProcessedBatch: OperatorLastProcessedBatch{
			BlockNumber:        0,
			batchProcessedChan: make(chan uint32),
		},

		// Timeout
		// Socket
	}

	err = operator.LoadLastProcessedBatch()
	if err != nil {
		logger.Fatalf("Error while loading last process batch: %v. This is probably related to the `last_processed_batch_filepath` field passed in the config file", err)
	}

	return operator, nil
}

func (o *Operator) SubscribeToNewTasksV2() (chan error, error) {
	return o.avsSubscriber.SubscribeToNewTasksV2(o.NewTaskCreatedChanV2)
}

func (o *Operator) SubscribeToNewTasksV3() (chan error, error) {
	return o.avsSubscriber.SubscribeToNewTasksV3(o.NewTaskCreatedChanV3)
}

type OperatorLastProcessedBatch struct {
	BlockNumber        uint32      `json:"block_number"`
	batchProcessedChan chan uint32 `json:"-"`
}

func (o *Operator) LoadLastProcessedBatch() error {
	// check if the directory exist
	folderPath := filepath.Dir(o.lastProcessedBatchLogFile)
	_, err := os.Stat(folderPath)

	if os.IsNotExist(err) {
		return err
	}

	file, err := os.ReadFile(o.lastProcessedBatchLogFile)

	// if the file does not exist, we don't return an err, as it will get created later
	// that is why we check of the directory exist in the first place
	if err != nil {
		return nil
	}

	err = json.Unmarshal(file, &o.lastProcessedBatch)

	if err != nil {
		return err
	}

	return nil
}

func (o *Operator) UpdateLastProcessBatch(blockNumber uint32) error {
	// we want to store the latest block number
	if blockNumber < o.lastProcessedBatch.BlockNumber {
		return nil
	}

	o.lastProcessedBatch.BlockNumber = blockNumber

	// write to a file so it can be recovered in case of operator outage
	json, err := json.Marshal(o.lastProcessedBatch)

	if err != nil {
		return fmt.Errorf("failed to marshal batch: %v", err)
	}

	err = os.WriteFile(o.lastProcessedBatchLogFile, json, os.ModePerm)
	if err != nil {
		return fmt.Errorf("failed to write to file: %v", err)
	}

	o.Logger.Infof("Updated latest block json file, new block: %v", blockNumber)

	return nil
}

func (o *Operator) Start(ctx context.Context) error {
	subV2, err := o.SubscribeToNewTasksV2()
	if err != nil {
		log.Fatal("Could not subscribe to new tasks")
	}

	subV3, err := o.SubscribeToNewTasksV3()
	if err != nil {
		log.Fatal("Could not subscribe to new tasks")
	}

	var metricsErrChan <-chan error
	if o.Config.Operator.EnableMetrics {
		metricsErrChan = o.metrics.Start(ctx, o.metricsReg)
	} else {
		metricsErrChan = make(chan error, 1)
	}

	go o.ProcessMissedBatchesWhileOffline()

	for {
		select {
		case <-context.Background().Done():
			o.Logger.Info("Operator shutting down...")
			return nil
		case err := <-metricsErrChan:
			o.Logger.Errorf("Metrics server failed", "err", err)
		case err := <-subV2:
			o.Logger.Infof("Error in websocket subscription", "err", err)
			subV2, err = o.SubscribeToNewTasksV2()
			if err != nil {
				o.Logger.Fatal("Could not subscribe to new tasks V2")
			}
		case err := <-subV3:
			o.Logger.Infof("Error in websocket subscription", "err", err)
			subV3, err = o.SubscribeToNewTasksV3()
			if err != nil {
				o.Logger.Fatal("Could not subscribe to new tasks V3")
			}
		case newBatchLogV2 := <-o.NewTaskCreatedChanV2:
			go o.handleNewBatchLogV2(newBatchLogV2)
		case newBatchLogV3 := <-o.NewTaskCreatedChanV3:
			go o.handleNewBatchLogV3(newBatchLogV3)
		case blockNumber := <-o.lastProcessedBatch.batchProcessedChan:
			err = o.UpdateLastProcessBatch(blockNumber)
			if err != nil {
				o.Logger.Errorf("Error while updating last process batch", "err", err)
			}
		}
	}
}

// Here we query all the batches that have not yet been verified starting from
// the latest verified batch by the operator. We also read from the previous
// `UnverifiedBatchOffset` blocks, because as batches are processed in parallel, there could be
// unverified batches slightly before the latest verified batch
func (o *Operator) ProcessMissedBatchesWhileOffline() {
	// this is the default value
	// and it means there was no file so no batches have been verified
	if o.lastProcessedBatch.BlockNumber == 0 {
		o.Logger.Info("Not continuing with missed batch processing, as operator hasn't verified anything yet...")
		return
	}

	o.Logger.Info("Getting missed tasks")

	// this check is necessary for overflows as go does not do saturating arithmetic
	var fromBlock uint64
	if o.lastProcessedBatch.BlockNumber < UnverifiedBatchOffset {
		fromBlock = 0
	} else {
		fromBlock = uint64(o.lastProcessedBatch.BlockNumber - UnverifiedBatchOffset)
	}

	logs, err := o.avsReader.GetNotRespondedTasksFrom(fromBlock)
	if err != nil {
		return
	}
	o.Logger.Infof(fmt.Sprintf("Missed tasks retrieved, total tasks to process: %v", len(logs)))

	if len(logs) == 0 {
		return
	}

	o.Logger.Infof("Starting to verify missed batches while offline")
	for _, logEntry := range logs {
		go o.handleNewBatchLogV3(&logEntry)
	}
	o.Logger.Info("Finished verifying all batches missed while offline")
}

// Currently, Operator can handle NewBatchV2 and NewBatchV3 events.

// The difference between these events do not affect the operator
// So if you read below, handleNewBatchLogV2 and handleNewBatchLogV3
// are identical.

// This structure may help for future upgrades. Having different logics under
// different events enables the smooth operator upgradeability

// Process of handling batches from V2 events:
func (o *Operator) handleNewBatchLogV2(newBatchLog *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2) {
	var err error
	defer func() { o.afterHandlingBatchV2(newBatchLog, err == nil) }()

	o.Logger.Info("Received new batch log V2")
	err = o.ProcessNewBatchLogV2(newBatchLog)
	if err != nil {
		o.Logger.Infof("batch %x did not verify. Err: %v", newBatchLog.BatchMerkleRoot, err)
		return
	}

	batchIdentifier := append(newBatchLog.BatchMerkleRoot[:], newBatchLog.SenderAddress[:]...)
	var batchIdentifierHash = *(*[32]byte)(crypto.Keccak256(batchIdentifier))
	responseSignature := o.SignTaskResponse(batchIdentifierHash)
	o.Logger.Debugf("responseSignature about to send: %x", responseSignature)

	signedTaskResponse := types.SignedTaskResponse{
		BatchIdentifierHash: batchIdentifierHash,
		BatchMerkleRoot:     newBatchLog.BatchMerkleRoot,
		SenderAddress:       newBatchLog.SenderAddress,
		BlsSignature:        *responseSignature,
		OperatorId:          o.OperatorId,
	}
	o.Logger.Infof("Signed Task Response to send: BatchIdentifierHash=%s, BatchMerkleRoot=%s, SenderAddress=%s",
		hex.EncodeToString(signedTaskResponse.BatchIdentifierHash[:]),
		hex.EncodeToString(signedTaskResponse.BatchMerkleRoot[:]),
		hex.EncodeToString(signedTaskResponse.SenderAddress[:]),
	)

	o.aggRpcClient.SendSignedTaskResponseToAggregator(&signedTaskResponse)
}
func (o *Operator) ProcessNewBatchLogV2(newBatchLog *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2) error {

	o.Logger.Info("Received new batch with proofs to verify",
		"batch merkle root", "0x"+hex.EncodeToString(newBatchLog.BatchMerkleRoot[:]),
		"sender address", "0x"+hex.EncodeToString(newBatchLog.SenderAddress[:]),
	)

	ctx, cancel := context.WithTimeout(context.Background(), BatchDownloadTimeout)
	defer cancel()

	verificationDataBatch, err := o.getBatchFromDataService(ctx, newBatchLog.BatchDataPointer, newBatchLog.BatchMerkleRoot, BatchDownloadMaxRetries, BatchDownloadRetryDelay)
	if err != nil {
		o.Logger.Errorf("Could not get proofs from S3 bucket: %v", err)
		return err
	}

	verificationDataBatchLen := len(verificationDataBatch)
	results := make(chan bool, verificationDataBatchLen)
	var wg sync.WaitGroup
	wg.Add(verificationDataBatchLen)

	disabledVerifiersBitmap, err := o.avsReader.DisabledVerifiers()
	if err != nil {
		o.Logger.Errorf("Could not check verifiers status: %s", err)
		results <- false
		return err
	}

	for _, verificationData := range verificationDataBatch {
		go func(data VerificationData) {
			defer wg.Done()
			o.verify(data, disabledVerifiersBitmap, results)
			o.metrics.IncOperatorTaskResponses()
		}(verificationData)
	}

	go func() {
		wg.Wait()
		close(results)
	}()

	for result := range results {
		if !result {
			return fmt.Errorf("invalid proof")
		}
	}

	return nil
}

// Process of handling batches from V3 events:
func (o *Operator) handleNewBatchLogV3(newBatchLog *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3) {
	var err error
	defer func() { o.afterHandlingBatchV3(newBatchLog, err == nil) }()
	o.Logger.Infof("Received new batch log V3")
	err = o.ProcessNewBatchLogV3(newBatchLog)
	if err != nil {
		o.Logger.Infof("batch %x did not verify. Err: %v", newBatchLog.BatchMerkleRoot, err)
		return
	}

	batchIdentifier := append(newBatchLog.BatchMerkleRoot[:], newBatchLog.SenderAddress[:]...)
	var batchIdentifierHash = *(*[32]byte)(crypto.Keccak256(batchIdentifier))
	responseSignature := o.SignTaskResponse(batchIdentifierHash)
	o.Logger.Debugf("responseSignature about to send: %x", responseSignature)

	signedTaskResponse := types.SignedTaskResponse{
		BatchIdentifierHash: batchIdentifierHash,
		BatchMerkleRoot:     newBatchLog.BatchMerkleRoot,
		SenderAddress:       newBatchLog.SenderAddress,
		BlsSignature:        *responseSignature,
		OperatorId:          o.OperatorId,
	}
	o.Logger.Infof("Signed Task Response to send: BatchIdentifierHash=%s, BatchMerkleRoot=%s, SenderAddress=%s",
		hex.EncodeToString(signedTaskResponse.BatchIdentifierHash[:]),
		hex.EncodeToString(signedTaskResponse.BatchMerkleRoot[:]),
		hex.EncodeToString(signedTaskResponse.SenderAddress[:]),
	)

	o.aggRpcClient.SendSignedTaskResponseToAggregator(&signedTaskResponse)
}
func (o *Operator) ProcessNewBatchLogV3(newBatchLog *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3) error {

	o.Logger.Info("Received new batch with proofs to verify",
		"batch merkle root", "0x"+hex.EncodeToString(newBatchLog.BatchMerkleRoot[:]),
		"sender address", "0x"+hex.EncodeToString(newBatchLog.SenderAddress[:]),
	)

	ctx, cancel := context.WithTimeout(context.Background(), BatchDownloadTimeout)
	defer cancel()

	verificationDataBatch, err := o.getBatchFromDataService(ctx, newBatchLog.BatchDataPointer, newBatchLog.BatchMerkleRoot, BatchDownloadMaxRetries, BatchDownloadRetryDelay)
	if err != nil {
		o.Logger.Errorf("Could not get proofs from S3 bucket: %v", err)
		return err
	}

	verificationDataBatchLen := len(verificationDataBatch)
	results := make(chan bool, verificationDataBatchLen)
	var wg sync.WaitGroup
	wg.Add(verificationDataBatchLen)
	disabledVerifiersBitmap, err := o.avsReader.DisabledVerifiers()
	if err != nil {
		o.Logger.Errorf("Could not check verifiers status: %s", err)
		results <- false
		return err
	}
	for _, verificationData := range verificationDataBatch {
		go func(data VerificationData) {
			defer wg.Done()
			o.verify(data, disabledVerifiersBitmap, results)
			o.metrics.IncOperatorTaskResponses()
		}(verificationData)
	}

	go func() {
		wg.Wait()
		close(results)
	}()

	for result := range results {
		if !result {
			return fmt.Errorf("invalid proof")
		}
	}

	return nil
}

func (o *Operator) afterHandlingBatchV2(log *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2, succeeded bool) {
	if succeeded {
		o.lastProcessedBatch.batchProcessedChan <- uint32(log.Raw.BlockNumber)
	}
}

func (o *Operator) afterHandlingBatchV3(log *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3, succeeded bool) {
	if succeeded {
		o.lastProcessedBatch.batchProcessedChan <- uint32(log.Raw.BlockNumber)
	}
}

func (o *Operator) verify(verificationData VerificationData, disabledVerifiersBitmap *big.Int, results chan bool) {
	IsVerifierDisabled := IsVerifierDisabled(disabledVerifiersBitmap, verificationData.ProvingSystemId)
	if IsVerifierDisabled {
		o.Logger.Infof("Verifier %s is disabled. Returning false", verificationData.ProvingSystemId.String())
		results <- false
		return
	}
	switch verificationData.ProvingSystemId {
	case common.GnarkPlonkBls12_381:
		verificationResult := o.verifyPlonkProofBLS12_381(verificationData.Proof, verificationData.PubInput, verificationData.VerificationKey)
		o.Logger.Infof("PLONK BLS12-381 proof verification result: %t", verificationResult)

		results <- verificationResult

	case common.GnarkPlonkBn254:
		verificationResult := o.verifyPlonkProofBN254(verificationData.Proof, verificationData.PubInput, verificationData.VerificationKey)
		o.Logger.Infof("PLONK BN254 proof verification result: %t", verificationResult)

		results <- verificationResult

	case common.Groth16Bn254:
		verificationResult := o.verifyGroth16ProofBN254(verificationData.Proof, verificationData.PubInput, verificationData.VerificationKey)
		o.Logger.Infof("GROTH16 BN254 proof verification result: %t", verificationResult)

		results <- verificationResult

	case common.SP1:
		verificationResult, err := sp1.VerifySp1Proof(verificationData.Proof, verificationData.VmProgramCode)
		if !verificationResult {
			o.Logger.Infof("SP1 proof verification failed. Trying old SP1 version...")
			verificationResult, err = sp1_old.VerifySp1ProofOld(verificationData.Proof, verificationData.VmProgramCode)
			if !verificationResult {
				o.Logger.Errorf("Old SP1 proof verification failed")
			}
		}
		o.Logger.Infof("SP1 proof verification result: %t", verificationResult)
		o.handleVerificationResult(results, verificationResult, err, "SP1 proof verification")

	case common.Risc0:
		verificationResult, err := risc_zero.VerifyRiscZeroReceipt(verificationData.Proof,
			verificationData.VmProgramCode, verificationData.PubInput)
		if !verificationResult {
			o.Logger.Infof("Risc0 proof verification failed. Trying old Risc0 version...")
			verificationResult, err = risc_zero_old.VerifyRiscZeroReceiptOld(verificationData.Proof, verificationData.VmProgramCode, verificationData.PubInput)
			if !verificationResult {
				o.Logger.Errorf("Old Risc0 proof verification failed")
			}
		}
		o.Logger.Infof("Risc0 proof verification result: %t", verificationResult)
		o.handleVerificationResult(results, verificationResult, err, "Risc0 proof verification")
	default:
		o.Logger.Error("Unrecognized proving system ID")
		results <- false
	}
}

func (o *Operator) handleVerificationResult(results chan bool, isVerified bool, err error, name string) {
	if err != nil {
		o.Logger.Errorf("%v failed %v", name, err)
		results <- false
	} else {
		o.Logger.Infof("%v result: %t", name, isVerified)
		results <- isVerified
	}
}

// VerifyPlonkProofBLS12_381 verifies a PLONK proof using BLS12-381 curve.
func (o *Operator) verifyPlonkProofBLS12_381(proofBytes []byte, pubInputBytes []byte, verificationKeyBytes []byte) bool {
	return o.verifyPlonkProof(proofBytes, pubInputBytes, verificationKeyBytes, ecc.BLS12_381)
}

// VerifyPlonkProofBN254 verifies a PLONK proof using BN254 curve.
func (o *Operator) verifyPlonkProofBN254(proofBytes []byte, pubInputBytes []byte, verificationKeyBytes []byte) bool {
	return o.verifyPlonkProof(proofBytes, pubInputBytes, verificationKeyBytes, ecc.BN254)
}

// VerifyGroth16ProofBN254 verifies a GROTH16 proof using BN254 curve.
func (o *Operator) verifyGroth16ProofBN254(proofBytes []byte, pubInputBytes []byte, verificationKeyBytes []byte) bool {
	return o.verifyGroth16Proof(proofBytes, pubInputBytes, verificationKeyBytes, ecc.BN254)
}

// verifyPlonkProof contains the common proof verification logic.
func (o *Operator) verifyPlonkProof(proofBytes []byte, pubInputBytes []byte, verificationKeyBytes []byte, curve ecc.ID) bool {
	proofReader := bytes.NewReader(proofBytes)
	proof := plonk.NewProof(curve)
	if _, err := proof.ReadFrom(proofReader); err != nil {
		o.Logger.Infof("Could not deserialize proof: %v", err)
		return false
	}

	pubInputReader := bytes.NewReader(pubInputBytes)
	pubInput, err := witness.New(curve.ScalarField())
	if err != nil {
		o.Logger.Infof("Error instantiating witness: %v", err)
		return false
	}
	if _, err = pubInput.ReadFrom(pubInputReader); err != nil {
		o.Logger.Infof("Could not read PLONK public input: %v", err)
		return false
	}

	verificationKeyReader := bytes.NewReader(verificationKeyBytes)
	verificationKey := plonk.NewVerifyingKey(curve)
	if _, err = verificationKey.ReadFrom(verificationKeyReader); err != nil {
		o.Logger.Infof("Could not read PLONK verifying key from bytes: %v", err)
		return false
	}

	err = plonk.Verify(proof, verificationKey, pubInput)
	return err == nil
}

// verifyGroth16Proof contains the common proof verification logic.
func (o *Operator) verifyGroth16Proof(proofBytes []byte, pubInputBytes []byte, verificationKeyBytes []byte, curve ecc.ID) bool {
	proofReader := bytes.NewReader(proofBytes)
	proof := groth16.NewProof(curve)
	if _, err := proof.ReadFrom(proofReader); err != nil {
		o.Logger.Infof("Could not deserialize proof: %v", err)
		return false
	}

	pubInputReader := bytes.NewReader(pubInputBytes)
	pubInput, err := witness.New(curve.ScalarField())
	if err != nil {
		o.Logger.Infof("Error instantiating witness: %v", err)
		return false
	}
	if _, err = pubInput.ReadFrom(pubInputReader); err != nil {
		o.Logger.Infof("Could not read Groth16 public input: %v", err)
		return false
	}

	verificationKeyReader := bytes.NewReader(verificationKeyBytes)
	verificationKey := groth16.NewVerifyingKey(curve)
	if _, err = verificationKey.ReadFrom(verificationKeyReader); err != nil {
		o.Logger.Infof("Could not read Groth16 verifying key from bytes: %v", err)
		return false
	}

	err = groth16.Verify(proof, verificationKey, pubInput)
	return err == nil
}

func (o *Operator) SignTaskResponse(batchIdentifierHash [32]byte) *bls.Signature {
	responseSignature := *o.Config.BlsConfig.KeyPair.SignMessage(batchIdentifierHash)
	return &responseSignature
}

func (o *Operator) SendTelemetryData(ctx *cli.Context) error {
	// hash version
	hash := sha3.NewLegacyKeccak256()
	hash.Write([]byte(ctx.App.Version))

	// get hash
	var version [32]byte // All zeroed initially
	copy(version[:], hash.Sum(nil))

	// sign version
	signature := o.Config.BlsConfig.KeyPair.SignMessage(version)
	public_key_g2 := o.Config.BlsConfig.KeyPair.GetPubKeyG2()
	ethRpcUrl, err := BaseUrlOnly(o.Config.BaseConfig.EthRpcUrl)
	if err != nil {
		return err
	}
	ethRpcUrlFallback, err := BaseUrlOnly(o.Config.BaseConfig.EthRpcUrlFallback)
	if err != nil {
		return err
	}
	ethWsUrl, err := BaseUrlOnly(o.Config.BaseConfig.EthWsUrl)
	if err != nil {
		return err
	}
	ethWsUrlFallback, err := BaseUrlOnly(o.Config.BaseConfig.EthWsUrlFallback)
	if err != nil {
		return err
	}

	body := map[string]interface{}{
		"eth_rpc_url":          ethRpcUrl,
		"eth_rpc_url_fallback": ethRpcUrlFallback,
		"eth_ws_url":           ethWsUrl,
		"eth_ws_url_fallback":  ethWsUrlFallback,
		"address":              o.Address,
		"version":              ctx.App.Version,
		"signature":            signature.Bytes(),
		"pub_key_g2":           public_key_g2.Bytes(),
	}

	bodyBuffer := new(bytes.Buffer)

	bodyReader := json.NewEncoder(bodyBuffer)
	err = bodyReader.Encode(body)
	if err != nil {
		return err
	}

	// send version to operator tracker server
	endpoint := o.Config.Operator.OperatorTrackerIpPortAddress + "/versions"
	o.Logger.Info("Sending version to operator tracker server: ", "endpoint", endpoint)

	res, err := http.Post(endpoint, "application/json", bodyBuffer)
	if err != nil {
		// Dont prevent operator from starting if operator tracker server is down
		o.Logger.Warn("Error sending version to metrics server: ", "err", err)
	} else if res.StatusCode != http.StatusCreated && res.StatusCode != http.StatusNoContent {
		o.Logger.Warn("Error sending version to operator tracker server: ", "status_code", res.StatusCode)
	}

	return nil
}
