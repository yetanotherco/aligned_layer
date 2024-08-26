package operator

import (
	"bytes"
	"context"
	"crypto/ecdsa"
	"encoding/hex"
	"fmt"
	"log"
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
	"github.com/yetanotherco/aligned_layer/common"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/types"

	"github.com/yetanotherco/aligned_layer/core/config"
)

type Operator struct {
	Config             config.OperatorConfig
	Address            ethcommon.Address
	Socket             string
	Timeout            time.Duration
	PrivKey            *ecdsa.PrivateKey
	KeyPair            *bls.KeyPair
	OperatorId         eigentypes.OperatorId
	avsSubscriber      chainio.AvsSubscriber
	NewTaskCreatedChan chan *servicemanager.ContractAlignedLayerServiceManagerNewBatch
	Logger             logging.Logger
	aggRpcClient       AggregatorRpcClient
	metricsReg         *prometheus.Registry
	metrics            *metrics.Metrics
	//Socket  string
	//Timeout time.Duration
}

const (
	BatchDownloadTimeout = 1 * time.Minute
)

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
		// Timeout
		// Socket
	}

	return operator, nil
}

func (o *Operator) SubscribeToNewTasks() (chan error, error) {
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

	for {
		select {
		case <-context.Background().Done():
			o.Logger.Info("Operator shutting down...")
			return nil
		case err := <-metricsErrChan:
			o.Logger.Fatal("Metrics server failed", "err", err)
		case err := <-sub:
			o.Logger.Infof("Error in websocket subscription", "err", err)
			sub, err = o.SubscribeToNewTasks()
			if err != nil {
				o.Logger.Fatal("Could not subscribe to new tasks")
			}
		case newBatchLog := <-o.NewTaskCreatedChan:
			err := o.ProcessNewBatchLog(newBatchLog)
			// err := o.ProcessNewBatchLogV2(newBatchLog)
			if err != nil {
				o.Logger.Infof("batch %x did not verify. Err: %v", newBatchLog.BatchMerkleRoot, err)
				continue
			}

			// V2
			// batchIdentifier := append(newBatchLog.BatchMerkleRoot[:], newBatchLog.SenderAddress[:]...)
			// var batchIdentifierHash = *(*[32]byte)(crypto.Keccak256(batchIdentifier))
			// responseSignature := o.SignTaskResponse(batchIdentifierHash)

			// V2
			// signedTaskResponse := types.SignedTaskResponse{
			// 	BatchIdentifierHash: batchIdentifierHash,
			// 	BatchMerkleRoot: newBatchLog.BatchMerkleRoot,
			// 	SenderAddress:   newBatchLog.SenderAddress,
			// 	BlsSignature:    *responseSignature,
			// 	OperatorId:      o.OperatorId,
			// }
			// o.Logger.Infof("Signed Task Response to send: BatchIdentifierHash=%s, BatchMerkleRoot=%s, SenderAddress=%s",
			// 	hex.EncodeToString(signedTaskResponse.BatchIdentifierHash[:]),
			// 	hex.EncodeToString(signedTaskResponse.BatchMerkleRoot[:]),
			// 	hex.EncodeToString(signedTaskResponse.SenderAddress[:]),
			// )


			responseSignature := o.SignTaskResponse(newBatchLog.BatchMerkleRoot)
			o.Logger.Debugf("responseSignature about to send: %x", responseSignature)

			signedTaskResponse := types.SignedTaskResponse{
				BatchMerkleRoot: newBatchLog.BatchMerkleRoot,
				BlsSignature:    *responseSignature,
				OperatorId:      o.OperatorId,
			}

			o.Logger.Infof("Signed Task Response to send: BatchIdentifierHash=%s, BatchMerkleRoot=%s, SenderAddress=%s",
				hex.EncodeToString(signedTaskResponse.BatchMerkleRoot[:]),
			)
			go o.aggRpcClient.SendSignedTaskResponseToAggregator(&signedTaskResponse)
		}
	}
}

// Takes a NewTaskCreatedLog struct as input and returns a TaskResponseHeader struct.
// The TaskResponseHeader struct is the struct that is signed and sent to the contract as a task response.
func (o *Operator) ProcessNewBatchLog(newBatchLog *servicemanager.ContractAlignedLayerServiceManagerNewBatch) error {

	o.Logger.Info("Received new batch with proofs to verify",
		"batch merkle root", "0x"+hex.EncodeToString(newBatchLog.BatchMerkleRoot[:]),
	)

	ctx, cancel := context.WithTimeout(context.Background(), BatchDownloadTimeout)
	defer cancel()

	verificationDataBatch, err := o.getBatchFromS3(ctx, newBatchLog.BatchDataPointer, newBatchLog.BatchMerkleRoot)
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
func (o *Operator) ProcessNewBatchLogV2(newBatchLog *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2) error {

	o.Logger.Info("Received new batch with proofs to verify",
		"batch merkle root", "0x"+hex.EncodeToString(newBatchLog.BatchMerkleRoot[:]),
		"sender address", "0x"+hex.EncodeToString(newBatchLog.SenderAddress[:]),
	)

	ctx, cancel := context.WithTimeout(context.Background(), BatchDownloadTimeout)
	defer cancel()

	verificationDataBatch, err := o.getBatchFromS3(ctx, newBatchLog.BatchDataPointer, newBatchLog.BatchMerkleRoot)
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
		proofLen := (uint32)(len(verificationData.Proof))
		paramsLen := (uint32)(len(verificationData.VerificationKey))
		publicInputLen := (uint32)(len(verificationData.PubInput))

		verificationResult := halo2ipa.VerifyHalo2IpaProof(
			verificationData.Proof, proofLen,
			verificationData.VerificationKey, paramsLen,
			verificationData.PubInput, publicInputLen)

		o.Logger.Infof("Halo2-IPA proof verification result: %t", verificationResult)
		results <- verificationResult
	case common.Halo2KZG:
		proofLen := (uint32)(len(verificationData.Proof))
		paramsLen := (uint32)(len(verificationData.VerificationKey))
		publicInputLen := (uint32)(len(verificationData.PubInput))

		verificationResult := halo2kzg.VerifyHalo2KzgProof(
			verificationData.Proof, proofLen,
			verificationData.VerificationKey, paramsLen,
			verificationData.PubInput, publicInputLen)

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

func (o *Operator) SignTaskResponse(batchIdentifierHash [32]byte) *bls.Signature {
	responseSignature := *o.Config.BlsConfig.KeyPair.SignMessage(batchIdentifierHash)
	return &responseSignature
}
