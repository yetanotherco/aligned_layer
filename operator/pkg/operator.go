package operator

import (
	"bytes"
	"context"
	"crypto/ecdsa"
	"encoding/hex"
	"fmt"
	"log"
	"time"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	"github.com/Layr-Labs/eigensdk-go/logging"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/plonk"
	"github.com/consensys/gnark/backend/witness"
	"github.com/ethereum/go-ethereum/accounts/abi"
	ethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/event"
	"github.com/yetanotherco/aligned_layer/common"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/types"
	"golang.org/x/crypto/sha3"

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
	NewTaskCreatedChan chan *servicemanager.ContractAlignedLayerServiceManagerNewTaskCreated
	Logger             logging.Logger
	aggRpcClient       AggregatorRpcClient
	//Socket  string
	//Timeout time.Duration
	//OperatorId         eigentypes.OperatorId
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
	newTaskCreatedChan := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewTaskCreated)

	// FIXME(marian): We should not hardcode the aggregator IP:PORT address
	rpcClient, err := NewAggregatorRpcClient("localhost:8090", logger)
	if err != nil {
		return nil, fmt.Errorf("Could not create RPC client: %s. Is aggregator running?", err)
	}

	address := configuration.Operator.Address
	operator := &Operator{
		Config:             configuration,
		Logger:             logger,
		avsSubscriber:      *avsSubscriber,
		Address:            address,
		NewTaskCreatedChan: newTaskCreatedChan,
		aggRpcClient:       *rpcClient,
		// Timeout
		// OperatorId
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
		case newTaskCreatedLog := <-o.NewTaskCreatedChan:
			o.Logger.Infof("Received task with index: %d\n", newTaskCreatedLog.TaskIndex)
			taskResponse := o.ProcessNewTaskCreatedLog(newTaskCreatedLog)
			responseSignature, err := o.SignTaskResponse(taskResponse)
			if err != nil {
				o.Logger.Errorf("Could not sign task response", "err", err)
			}

			signedTaskResponse := types.SignedTaskResponse{
				TaskResponse: *taskResponse,
				BlsSignature: *responseSignature,
				// FIXME(marian): Dummy Operator ID, we should get the correct one.
				OperatorId: eigentypes.Bytes32(make([]byte, 32)),
			}

			o.Logger.Infof("Signed hash: %+v", *responseSignature)
			go o.aggRpcClient.SendSignedTaskResponseToAggregator(&signedTaskResponse)
		}
	}
}

// Takes a NewTaskCreatedLog struct as input and returns a TaskResponseHeader struct.
// The TaskResponseHeader struct is the struct that is signed and sent to the contract as a task response.
func (o *Operator) ProcessNewTaskCreatedLog(newTaskCreatedLog *servicemanager.ContractAlignedLayerServiceManagerNewTaskCreated) *servicemanager.AlignedLayerServiceManagerTaskResponse {
	proof := newTaskCreatedLog.Task.Proof
	proofLen := (uint)(len(proof))

	pubInput := newTaskCreatedLog.Task.PubInput
	// pubInputLen := (uint)(len(pubInput))

	provingSystemId := newTaskCreatedLog.Task.ProvingSystemId

	o.Logger.Info("Received new task with proof to verify",
		"proof length", proofLen,
		"proof first bytes", "0x"+hex.EncodeToString(proof[0:8]),
		"proof last bytes", "0x"+hex.EncodeToString(proof[proofLen-8:proofLen]),
		"task index", newTaskCreatedLog.TaskIndex,
		"task created block", newTaskCreatedLog.Task.TaskCreatedBlock,
		// "quorumNumbers", newTaskCreatedLog.Task.QuorumNumbers,
		"QuorumThresholdPercentage", newTaskCreatedLog.Task.QuorumThresholdPercentage,
	)

	switch provingSystemId {
	case uint16(common.GnarkPlonkBls12_381):
		verificationKey := newTaskCreatedLog.Task.VerificationKey
		VerificationResult := o.VerifyPlonkProofBLS12_381(proof, pubInput, verificationKey)

		o.Logger.Infof("PLONK BLS12_381 proof verification result: %t", VerificationResult)
		taskResponse := &servicemanager.AlignedLayerServiceManagerTaskResponse{
			TaskIndex:      newTaskCreatedLog.TaskIndex,
			ProofIsCorrect: VerificationResult,
		}
		return taskResponse

	case uint16(common.GnarkPlonkBn254):
		verificationKey := newTaskCreatedLog.Task.VerificationKey
		VerificationResult := o.VerifyPlonkProofBN254(proof, pubInput, verificationKey)

		o.Logger.Infof("PLONK BN254 proof verification result: %t", VerificationResult)
		taskResponse := &servicemanager.AlignedLayerServiceManagerTaskResponse{
			TaskIndex:      newTaskCreatedLog.TaskIndex,
			ProofIsCorrect: VerificationResult,
		}
		return taskResponse

	default:
		o.Logger.Error("Unrecognized proving system ID")
		return nil
	}
}

// VerifyPlonkProofBLS12_381 verifies a PLONK proof using BLS12-381 curve.
func (o *Operator) VerifyPlonkProofBLS12_381(proofBytes []byte, pubInputBytes []byte, verificationKeyBytes []byte) bool {
	return o.verifyPlonkProof(proofBytes, pubInputBytes, verificationKeyBytes, ecc.BLS12_381)
}

// VerifyPlonkProofBN254 verifies a PLONK proof using BN254 curve.
func (o *Operator) VerifyPlonkProofBN254(proofBytes []byte, pubInputBytes []byte, verificationKeyBytes []byte) bool {
	return o.verifyPlonkProof(proofBytes, pubInputBytes, verificationKeyBytes, ecc.BN254)
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

func AbiEncodeTaskResponse(taskResponse servicemanager.AlignedLayerServiceManagerTaskResponse) ([]byte, error) {
	// The order here has to match the field ordering of servicemanager.AlignedLayerServiceManagerTaskResponse

	/* TODO: Solve this in a more generic way so it's less prone for errors. Name and types can be obtained with reflection
	for i := 0; i < reflectedType.NumField(); i++ {
		name := reflectedType.Field(i).Name
		thisType := reflectedType.Field(i).Type
	}
	*/

	/*
		This matches:

		struct TaskResponse {
			uint64 taskIndex;
			bool proofIsCorrect;
		}
	*/
	taskResponseType, err := abi.NewType("tuple", "", []abi.ArgumentMarshaling{
		{
			Name: "taskIndex",
			Type: "uint64",
		},
		{
			Name: "proofIsCorrect",
			Type: "bool",
		},
	})
	if err != nil {
		return nil, err
	}
	arguments := abi.Arguments{
		{
			Type: taskResponseType,
		},
	}

	bytes, err := arguments.Pack(taskResponse)
	if err != nil {
		return nil, err
	}

	return bytes, nil
}

func (o *Operator) SignTaskResponse(taskResponse *servicemanager.AlignedLayerServiceManagerTaskResponse) (*bls.Signature, error) {
	encodedResponseBytes, err := AbiEncodeTaskResponse(*taskResponse)
	if err != nil {
		return nil, err
	}

	var taskResponseDigest [32]byte
	hasher := sha3.NewLegacyKeccak256()
	hasher.Write(encodedResponseBytes)
	copy(taskResponseDigest[:], hasher.Sum(nil)[:32])

	responseSignature := *o.Config.BlsConfig.KeyPair.SignMessage(taskResponseDigest)
	return &responseSignature, nil
}
