package operator

import (
	"bytes"
	"context"
	"crypto/ecdsa"
	"encoding/binary"
	"fmt"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"log"
	"math/big"
	"strings"
	"sync"
	"time"

	"github.com/ethereum/go-ethereum/crypto"
	"github.com/yetanotherco/aligned_layer/operator/risc_zero"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/yetanotherco/aligned_layer/metrics"

	"github.com/yetanotherco/aligned_layer/operator/halo2ipa"
	"github.com/yetanotherco/aligned_layer/operator/halo2kzg"
	"github.com/yetanotherco/aligned_layer/operator/sp1"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	"github.com/Layr-Labs/eigensdk-go/logging"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/backend/plonk"
	"github.com/consensys/gnark/backend/witness"
	ethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/event"
	"github.com/yetanotherco/aligned_layer/common"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/core/types"
)

const (
	blockInterval           uint64 = 50000
	pollLatestBatchInterval        = 5 * time.Second
)

type Operator struct {
	Config                config.OperatorConfig
	Address               ethcommon.Address
	Socket                string
	Timeout               time.Duration
	PrivKey               *ecdsa.PrivateKey
	KeyPair               *bls.KeyPair
	OperatorId            eigentypes.OperatorId
	avsSubscriber         chainio.AvsSubscriber
	NewTaskCreatedChan    chan *servicemanager.ContractAlignedLayerServiceManagerNewBatch
	Logger                logging.Logger
	aggRpcClient          AggregatorRpcClient
	metricsReg            *prometheus.Registry
	metrics               *metrics.Metrics
	processedBatches      map[[32]byte]struct{}
	processedBatchesMutex *sync.Mutex
	latestProcessedBatch  *servicemanager.ContractAlignedLayerServiceManagerNewBatch
	//Socket  string
	//Timeout time.Duration
}

func NewOperatorFromConfig(configuration config.OperatorConfig) (*Operator, error) {
	logger := configuration.BaseConfig.Logger

	avsReader, err := chainio.NewAvsReaderFromConfig(configuration.BaseConfig, configuration.EcdsaConfig)
	if err != nil {
		log.Fatalf("Could not create AVS reader")
	}

	registered, err := avsReader.IsOperatorRegistered(configuration.Operator.Address)
	if err != nil {
		log.Fatalf("Could not check if operator is registered")
	}

	if !registered {
		log.Println("Operator is not registered with AlignedLayer AVS, registering...")
		quorumNumbers := []byte{0}

		// Generate salt and expiry
		privateKeyBytes := []byte(configuration.BlsConfig.KeyPair.PrivKey.String())
		salt := [32]byte{}

		copy(salt[:], crypto.Keccak256([]byte("churn"), []byte(time.Now().String()), quorumNumbers, privateKeyBytes))

		err = RegisterOperator(context.Background(), &configuration, salt)
		if err != nil {
			log.Fatalf("Could not register operator")
		}
	}

	avsSubscriber, err := chainio.NewAvsSubscriberFromConfig(configuration.BaseConfig)
	if err != nil {
		log.Fatalf("Could not create AVS subscriber")
	}
	newTaskCreatedChan := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewBatch)

	rpcClient, err := NewAggregatorRpcClient(configuration.Operator.AggregatorServerIpPortAddress, logger)
	if err != nil {
		return nil, fmt.Errorf("Could not create RPC client: %s. Is aggregator running?", err)
	}

	operatorId := eigentypes.OperatorIdFromKeyPair(configuration.BlsConfig.KeyPair)
	address := configuration.Operator.Address

	// Metrics
	reg := prometheus.NewRegistry()
	operatorMetrics := metrics.NewMetrics(configuration.Operator.MetricsIpPortAddress, reg, logger)

	operator := &Operator{
		Config:             configuration,
		Logger:             logger,
		avsSubscriber:      *avsSubscriber,
		Address:            address,
		NewTaskCreatedChan: newTaskCreatedChan,
		aggRpcClient:       *rpcClient,
		OperatorId:         operatorId,
		metricsReg:         reg,
		metrics:            operatorMetrics,
		// The cheapest way to make a set in Go is to use a map with empty structs as values
		processedBatches:      make(map[[32]byte]struct{}),
		processedBatchesMutex: &sync.Mutex{},
		latestProcessedBatch:  nil,
		// Timeout
		// Socket
	}

	return operator, nil
}

func (o *Operator) SubscribeToNewTasks() (event.Subscription, error) {
	return o.avsSubscriber.SubscribeToNewTasks(o.NewTaskCreatedChan)
}

func (o *Operator) Start(ctx context.Context) error {
	sub, err := o.SubscribeToNewTasks()
	if err != nil {
		log.Fatal("Could not subscribe to new tasks")
	}

	var metricsErrChan <-chan error
	if o.Config.Operator.EnableMetrics {
		metricsErrChan = o.metrics.Start(ctx, o.metricsReg)
	} else {
		metricsErrChan = make(chan error, 1)
	}

	pollLatestBatchTicker := time.NewTicker(pollLatestBatchInterval)
	defer pollLatestBatchTicker.Stop()

	for {
		select {
		case <-context.Background().Done():
			o.Logger.Info("Operator shutting down...")
			return nil
		case err := <-metricsErrChan:
			o.Logger.Fatal("Metrics server failed", "err", err)
		case err := <-sub.Err():
			o.Logger.Infof("Error in websocket subscription", "err", err)
			sub.Unsubscribe()
			sub, err = o.SubscribeToNewTasks()
			if err != nil {
				o.Logger.Fatal("Could not subscribe to new tasks")
			}
		case newBatchLog := <-o.NewTaskCreatedChan:
			o.processedBatchesMutex.Lock()
			err := o.ProcessNewBatchLog(newBatchLog)
			if err != nil {
				o.Logger.Infof("batch %x did not verify. Err: %v", newBatchLog.BatchMerkleRoot, err)
				o.processedBatchesMutex.Unlock()
				continue
			}
			responseSignature := o.SignTaskResponse(newBatchLog.BatchMerkleRoot)

			signedTaskResponse := types.SignedTaskResponse{
				BatchMerkleRoot: newBatchLog.BatchMerkleRoot,
				BlsSignature:    *responseSignature,
				OperatorId:      o.OperatorId,
			}

			o.processedBatches[newBatchLog.BatchMerkleRoot] = struct{}{}
			o.latestProcessedBatch = newBatchLog

			o.processedBatchesMutex.Unlock()

			o.Logger.Infof("Signed hash: %+v", *responseSignature)
			go o.aggRpcClient.SendSignedTaskResponseToAggregator(&signedTaskResponse)
		case <-pollLatestBatchTicker.C:
			err := o.checkForMissedBatches()
			if err != nil {
				o.Logger.Infof("Could not process latest task: %v", err)
				continue
			}
		}
	}
}

// Takes a NewTaskCreatedLog struct as input and returns a TaskResponseHeader struct.
// The TaskResponseHeader struct is the struct that is signed and sent to the contract as a task response.
func (o *Operator) ProcessNewBatchLog(newBatchLog *servicemanager.ContractAlignedLayerServiceManagerNewBatch) error {

	o.Logger.Info("Received new batch with proofs to verify",
		"batch merkle root", newBatchLog.BatchMerkleRoot,
	)

	verificationDataBatch, err := o.getBatchFromS3(newBatchLog.BatchDataPointer, newBatchLog.BatchMerkleRoot)
	if err != nil {
		o.Logger.Errorf("Could not get proofs from S3 bucket: %v", err)
		return err
	}

	verificationDataBatchLen := len(verificationDataBatch)
	results := make(chan bool, verificationDataBatchLen)
	var wg sync.WaitGroup
	wg.Add(verificationDataBatchLen)
	for _, verificationData := range verificationDataBatch {
		go func(data VerificationData) {
			defer wg.Done()
			o.verify(data, results)
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

func (o *Operator) processNewBatch(newBatchLog *servicemanager.ContractAlignedLayerServiceManagerNewBatch) error {

	err := o.ProcessNewBatchLog(newBatchLog)
	if err != nil {
		return fmt.Errorf("batch %v did not verify: %w", newBatchLog.BatchMerkleRoot, err)
	}

	responseSignature := o.SignTaskResponse(newBatchLog.BatchMerkleRoot)

	signedTaskResponse := types.SignedTaskResponse{
		BatchMerkleRoot: newBatchLog.BatchMerkleRoot,
		BlsSignature:    *responseSignature,
		OperatorId:      o.OperatorId,
	}

	o.Logger.Infof("Signed hash: %+v", *responseSignature)

	go o.aggRpcClient.SendSignedTaskResponseToAggregator(&signedTaskResponse)

	return nil

}

func (o *Operator) checkForMissedBatches() error {
	latestBatch, err := o.getLatestTaskFromBlockchain()
	if err != nil {
		// This is not necessarily an error, it just means that there are no new batches
		return nil
	}

	o.processedBatchesMutex.Lock()
	defer o.processedBatchesMutex.Unlock()

	if o.latestProcessedBatch != nil && latestBatch.BatchMerkleRoot == o.latestProcessedBatch.BatchMerkleRoot {
		// Latest batch has already been processed
		return nil
	}

	if _, processed := o.processedBatches[latestBatch.BatchMerkleRoot]; !processed {
		o.Logger.Infof("Found missed batch %x, processing...", latestBatch.BatchMerkleRoot)
		err := o.processNewBatch(latestBatch)
		if err != nil {
			return err
		}
		o.processedBatches[latestBatch.BatchMerkleRoot] = struct{}{}
		o.latestProcessedBatch = latestBatch
	}

	return nil
}

// getLatestTaskFromBlockchain queries the blockchain for the latest task using the FilterLogs method.
// The alternative to this is using the FilterNewBatch method from the contract's filterer, but it requires
// to iterate over all the logs, which is not efficient and not needed since we only need the latest task.
func (o *Operator) getLatestTaskFromBlockchain() (*servicemanager.ContractAlignedLayerServiceManagerNewBatch, error) {
	latestBlock, err := o.Config.BaseConfig.EthRpcClient.BlockNumber(context.Background())

	if err != nil {
		return nil, fmt.Errorf("failed to get latest block number: %w", err)
	}

	var fromBlock uint64

	if latestBlock < blockInterval {
		fromBlock = 0
	} else {
		fromBlock = latestBlock - blockInterval
	}

	alignedLayerServiceManagerABI, err := abi.JSON(strings.NewReader(servicemanager.ContractAlignedLayerServiceManagerMetaData.ABI))
	if err != nil {
		return nil, fmt.Errorf("failed to parse ABI: %w", err)
	}

	// We just care about the createNewTask event
	newBatchEvent := alignedLayerServiceManagerABI.Events["NewBatch"]
	if newBatchEvent.ID == (ethcommon.Hash{}) {
		return nil, fmt.Errorf("createNewTask event not found in ABI")
	}

	query := ethereum.FilterQuery{
		FromBlock: big.NewInt(int64(fromBlock)),
		ToBlock:   big.NewInt(int64(latestBlock)),
		Addresses: []ethcommon.Address{o.Config.AlignedLayerDeploymentConfig.AlignedLayerServiceManagerAddr},
		Topics:    [][]ethcommon.Hash{{newBatchEvent.ID, {}}},
	}

	logs, err := o.Config.BaseConfig.EthRpcClient.FilterLogs(context.Background(), query)
	if err != nil {
		return nil, fmt.Errorf("failed to get logs: %w", err)
	}

	if len(logs) == 0 {
		return nil, fmt.Errorf("no logs found")
	}

	lastLog := logs[len(logs)-1]

	var latestTask servicemanager.ContractAlignedLayerServiceManagerNewBatch
	err = alignedLayerServiceManagerABI.UnpackIntoInterface(&latestTask, "NewBatch", lastLog.Data)
	if err != nil {
		return nil, fmt.Errorf("failed to unpack log data: %w", err)
	}

	// The second topic is the batch merkle root, as it is an indexed variable in the contract
	latestTask.BatchMerkleRoot = lastLog.Topics[1]

	return &latestTask, nil

}

func (o *Operator) verify(verificationData VerificationData, results chan bool) {
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
		proofLen := (uint32)(len(verificationData.Proof))
		elfLen := (uint32)(len(verificationData.VmProgramCode))

		verificationResult := sp1.VerifySp1Proof(verificationData.Proof, proofLen, verificationData.VmProgramCode, elfLen)
		o.Logger.Infof("SP1 proof verification result: %t", verificationResult)
		results <- verificationResult
	case common.Halo2IPA:
		// Extract Proof Bytes
		proofBytes := make([]byte, halo2ipa.MaxProofSize)
		copy(proofBytes, verificationData.Proof)
		proofLen := (uint32)(len(verificationData.Proof))

		// Extract Verification Key Bytes
		paramsBytes := verificationData.VerificationKey

		// Deserialize csLen
		csLenBuffer := make([]byte, 4)
		copy(csLenBuffer, paramsBytes[:4])
		csLen := (uint32)(binary.LittleEndian.Uint32(csLenBuffer))

		// Deserialize vkLen
		vkLenBuffer := make([]byte, 4)
		copy(vkLenBuffer, paramsBytes[4:8])
		vkLen := (uint32)(binary.LittleEndian.Uint32(vkLenBuffer))

		// Deserialize ipaParamLen
		IpaParamsLenBuffer := make([]byte, 4)
		copy(IpaParamsLenBuffer, paramsBytes[8:12])
		IpaParamsLen := (uint32)(binary.LittleEndian.Uint32(IpaParamsLenBuffer))

		// Extract Constraint System Bytes
		csBytes := make([]byte, halo2ipa.MaxConstraintSystemSize)
		csOffset := uint32(12)
		copy(csBytes, paramsBytes[csOffset:(csOffset+csLen)])

		// Extract Verification Key Bytes
		vkBytes := make([]byte, halo2ipa.MaxVerifierKeySize)
		vkOffset := csOffset + csLen
		copy(vkBytes, paramsBytes[vkOffset:(vkOffset+vkLen)])

		// Extract ipa Parameter Bytes
		IpaParamsBytes := make([]byte, (halo2ipa.MaxIpaParamsSize))
		IpaParamsOffset := vkOffset + vkLen
		copy(IpaParamsBytes, paramsBytes[IpaParamsOffset:])

		// Extract Public Input Bytes
		publicInput := verificationData.PubInput
		publicInputBytes := make([]byte, halo2ipa.MaxPublicInputSize)
		copy(publicInputBytes, publicInput)
		publicInputLen := (uint32)(len(publicInput))

		verificationResult := halo2ipa.VerifyHalo2IpaProof(
			([halo2ipa.MaxProofSize]byte)(proofBytes), proofLen,
			([halo2ipa.MaxConstraintSystemSize]byte)(csBytes), csLen,
			([halo2ipa.MaxVerifierKeySize]byte)(vkBytes), vkLen,
			([halo2ipa.MaxIpaParamsSize]byte)(IpaParamsBytes), IpaParamsLen,
			([halo2ipa.MaxPublicInputSize]byte)(publicInputBytes), publicInputLen)

		o.Logger.Infof("Halo2-IPA proof verification result: %t", verificationResult)
		results <- verificationResult
	case common.Halo2KZG:
		// Extract Proof Bytes
		proofBytes := make([]byte, halo2kzg.MaxProofSize)
		copy(proofBytes, verificationData.Proof)
		proofLen := (uint32)(len(verificationData.Proof))

		// Extract Verification Key Bytes
		paramsBytes := verificationData.VerificationKey

		// Deserialize csLen
		csLenBuffer := make([]byte, 4)
		copy(csLenBuffer, paramsBytes[:4])
		csLen := (uint32)(binary.LittleEndian.Uint32(csLenBuffer))

		// Deserialize vkLen
		vkLenBuffer := make([]byte, 4)
		copy(vkLenBuffer, paramsBytes[4:8])
		vkLen := (uint32)(binary.LittleEndian.Uint32(vkLenBuffer))

		// Deserialize kzgParamLen
		kzgParamsLenBuffer := make([]byte, 4)
		copy(kzgParamsLenBuffer, paramsBytes[8:12])
		kzgParamsLen := (uint32)(binary.LittleEndian.Uint32(kzgParamsLenBuffer))

		// Extract Constraint System Bytes
		csBytes := make([]byte, halo2kzg.MaxConstraintSystemSize)
		csOffset := uint32(12)
		copy(csBytes, paramsBytes[csOffset:(csOffset+csLen)])

		// Extract Verification Key Bytes
		vkBytes := make([]byte, halo2kzg.MaxVerifierKeySize)
		vkOffset := csOffset + csLen
		copy(vkBytes, paramsBytes[vkOffset:(vkOffset+vkLen)])

		// Extract Kzg Parameter Bytes
		kzgParamsBytes := make([]byte, (halo2kzg.MaxKzgParamsSize))
		kzgParamsOffset := vkOffset + vkLen
		copy(kzgParamsBytes, paramsBytes[kzgParamsOffset:])

		// Extract Public Input Bytes
		publicInput := verificationData.PubInput
		publicInputBytes := make([]byte, halo2kzg.MaxPublicInputSize)
		copy(publicInputBytes, publicInput)
		publicInputLen := (uint32)(len(publicInput))

		verificationResult := halo2kzg.VerifyHalo2KzgProof(
			([halo2kzg.MaxProofSize]byte)(proofBytes), proofLen,
			([halo2kzg.MaxConstraintSystemSize]byte)(csBytes), csLen,
			([halo2kzg.MaxVerifierKeySize]byte)(vkBytes), vkLen,
			([halo2kzg.MaxKzgParamsSize]byte)(kzgParamsBytes), kzgParamsLen,
			([halo2kzg.MaxPublicInputSize]byte)(publicInputBytes), publicInputLen)

		o.Logger.Infof("Halo2-KZG proof verification result: %t", verificationResult)
		results <- verificationResult
	case common.Risc0:
		proofLen := (uint32)(len(verificationData.Proof))
		imageIdLen := (uint32)(len(verificationData.VmProgramCode))
		pubInputLen := (uint32)(len(verificationData.PubInput))

		verificationResult := risc_zero.VerifyRiscZeroReceipt(verificationData.Proof, proofLen,
			verificationData.VmProgramCode, imageIdLen, verificationData.PubInput, pubInputLen)

		o.Logger.Infof("Risc0 proof verification result: %t", verificationResult)
		results <- verificationResult
	default:
		o.Logger.Error("Unrecognized proving system ID")
		results <- false
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

func (o *Operator) SignTaskResponse(batchMerkleRoot [32]byte) *bls.Signature {
	responseSignature := *o.Config.BlsConfig.KeyPair.SignMessage(batchMerkleRoot)
	return &responseSignature
}
