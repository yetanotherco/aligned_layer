package operator

import (
	"bytes"
	"context"
	"crypto/ecdsa"
	"fmt"
	"log"
	"time"

	"github.com/celestiaorg/celestia-node/api/rpc/client"
	"github.com/yetanotherco/aligned_layer/operator/sp1"

	"github.com/Layr-Labs/eigenda/api/grpc/disperser"
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
	disperser          disperser.DisperserClient
	celestiaClient     *client.Client
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
		log.Fatalf("Operator is not registered with AlignedLayer AVS")
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
	operator := &Operator{
		Config:             configuration,
		Logger:             logger,
		avsSubscriber:      *avsSubscriber,
		Address:            address,
		NewTaskCreatedChan: newTaskCreatedChan,
		aggRpcClient:       *rpcClient,
		OperatorId:         operatorId,
		disperser:          configuration.EigenDADisperserConfig.Disperser,
		celestiaClient:     configuration.CelestiaConfig.Client,
		// Timeout
		// Socket
	}

	return operator, nil
}

func (o *Operator) SubscribeToNewTasks() event.Subscription {
	sub := o.avsSubscriber.SubscribeToNewTasks(o.NewTaskCreatedChan)
	return sub
}

func (o *Operator) Start(ctx context.Context) error {
	sub := o.SubscribeToNewTasks()
	for {
		select {
		case <-context.Background().Done():
			o.Logger.Info("Operator shutting down...")
			return nil
		case err := <-sub.Err():
			o.Logger.Infof("Error in websocket subscription", "err", err)
			sub.Unsubscribe()
			sub = o.SubscribeToNewTasks()
		case newBatchLog := <-o.NewTaskCreatedChan:
			// o.Logger.Infof("Received task with index: %d\n", newTaskCreatedLog.TaskIndex)
			err := o.ProcessNewBatchLog(newBatchLog)
			if err != nil {
				o.Logger.Errorf("Proof in batch did not verify", "err", err)
				// FIXME(marian): This is not how we should handle this error. Just doing this for fast iteration and debug
				panic("Proof did not verify")
			}
			responseSignature := o.SignTaskResponse(newBatchLog.BatchMerkleRoot)

			signedTaskResponse := types.SignedTaskResponse{
				BatchMerkleRoot:  newBatchLog.BatchMerkleRoot,
				TaskCreatedBlock: newBatchLog.TaskCreatedBlock,
				BlsSignature:     *responseSignature,
				OperatorId:       o.OperatorId,
			}

			o.Logger.Infof("Signed hash: %+v", *responseSignature)
			go o.aggRpcClient.SendSignedTaskResponseToAggregator(&signedTaskResponse)
		}
	}
}

// Takes a NewTaskCreatedLog struct as input and returns a TaskResponseHeader struct.
// The TaskResponseHeader struct is the struct that is signed and sent to the contract as a task response.
func (o *Operator) ProcessNewBatchLog(newBatchLog *servicemanager.ContractAlignedLayerServiceManagerNewBatch) error {

	o.Logger.Info("Received new batch with proofs to verify",
		"batch merkle root", newBatchLog.BatchMerkleRoot,
	)

	verificationDataBatch, err := o.getBatchFromS3(newBatchLog.BatchDataPointer)
	if err != nil {
		o.Logger.Errorf("Could not get proofs from S3 bucket: %v", err)
		return nil
	}

	for _, verificationData := range verificationDataBatch {
		if !o.verify(verificationData) {
			return fmt.Errorf("Proof did not verify")
		}
	}

	return nil
}

func (o *Operator) verify(verificationData VerificationData) bool {
	switch verificationData.ProvingSystemId {
	case common.GnarkPlonkBls12_381:
		verificationResult := o.verifyPlonkProofBLS12_381(verificationData.Proof, verificationData.PubInput, verificationData.VerificationKey)
		o.Logger.Infof("PLONK BLS12-381 proof verification result: %t", verificationResult)

		return verificationResult

	case common.GnarkPlonkBn254:
		verificationResult := o.verifyPlonkProofBN254(verificationData.Proof, verificationData.PubInput, verificationData.VerificationKey)
		o.Logger.Infof("PLONK BN254 proof verification result: %t", verificationResult)

		return verificationResult

	case common.Groth16Bn254:
		verificationResult := o.verifyGroth16ProofBN254(verificationData.Proof, verificationData.PubInput, verificationData.VerificationKey)

		o.Logger.Infof("GROTH16 BN254 proof verification result: %t", verificationResult)
		return verificationResult

	case common.SP1:
		proofBytes := make([]byte, sp1.MaxProofSize)
		copy(proofBytes, verificationData.Proof)
		proofLen := (uint)(len(verificationData.Proof))

		elf := verificationData.VmProgramCode
		elfBytes := make([]byte, sp1.MaxElfBufferSize)
		copy(elfBytes, elf)
		elfLen := (uint)(len(elf))

		verificationResult := sp1.VerifySp1Proof(([sp1.MaxProofSize]byte)(proofBytes), proofLen, ([sp1.MaxElfBufferSize]byte)(elfBytes), elfLen)
		o.Logger.Infof("SP1 proof verification result: %t", verificationResult)

		return verificationResult

	default:
		o.Logger.Error("Unrecognized proving system ID")
		return false
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
		o.Logger.Errorf("Could not deserialize proof: %v", err)
		return false
	}

	pubInputReader := bytes.NewReader(pubInputBytes)
	pubInput, err := witness.New(curve.ScalarField())
	if err != nil {
		o.Logger.Errorf("Error instantiating witness: %v", err)
		return false
	}
	if _, err = pubInput.ReadFrom(pubInputReader); err != nil {
		o.Logger.Errorf("Could not read PLONK public input: %v", err)
		return false
	}

	verificationKeyReader := bytes.NewReader(verificationKeyBytes)
	verificationKey := plonk.NewVerifyingKey(curve)
	if _, err = verificationKey.ReadFrom(verificationKeyReader); err != nil {
		o.Logger.Errorf("Could not read PLONK verifying key from bytes: %v", err)
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
		o.Logger.Errorf("Could not deserialize proof: %v", err)
		return false
	}

	pubInputReader := bytes.NewReader(pubInputBytes)
	pubInput, err := witness.New(curve.ScalarField())
	if err != nil {
		o.Logger.Errorf("Error instantiating witness: %v", err)
		return false
	}
	if _, err = pubInput.ReadFrom(pubInputReader); err != nil {
		o.Logger.Errorf("Could not read Groth16 public input: %v", err)
		return false
	}

	verificationKeyReader := bytes.NewReader(verificationKeyBytes)
	verificationKey := groth16.NewVerifyingKey(curve)
	if _, err = verificationKey.ReadFrom(verificationKeyReader); err != nil {
		o.Logger.Errorf("Could not read Groth16 verifying key from bytes: %v", err)
		return false
	}

	err = groth16.Verify(proof, verificationKey, pubInput)
	return err == nil
}

func (o *Operator) SignTaskResponse(batchMerkleRoot [32]byte) *bls.Signature {
	responseSignature := *o.Config.BlsConfig.KeyPair.SignMessage(batchMerkleRoot)
	return &responseSignature
}
