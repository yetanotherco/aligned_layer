package operator

import (
	"bytes"
	"context"
	"crypto/ecdsa"
	"encoding/hex"
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
	//Socket  string
	//Timeout time.Duration
	//OperatorId         eigentypes.OperatorId
}

func NewOperatorFromConfig(configuration config.OperatorConfig) (*Operator, error) {
	logger := configuration.BaseConfig.Logger

	avsSubscriber, err := chainio.NewAvsSubscriberFromConfig(configuration.BaseConfig)
	if err != nil {
		log.Fatalf("Could not create AVS subscriber")
	}
	newTaskCreatedChan := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewTaskCreated)

	address := configuration.Operator.Address
	operator := &Operator{
		Config:             configuration,
		Logger:             logger,
		avsSubscriber:      *avsSubscriber,
		Address:            address,
		NewTaskCreatedChan: newTaskCreatedChan,
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

			o.Logger.Infof("Signed hash: %+v", *responseSignature)
			// go o.aggregatorRpcClient.SendSignedTaskResponseToAggregator(signedTaskResponse)
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
		VerificationResult := o.VerifyPlonkProof(proof, pubInput, verificationKey)

		o.Logger.Infof("PLONK proof verification result: %t", VerificationResult)
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

func (o *Operator) VerifyPlonkProof(proofBytes []byte, pubInputBytes []byte, verificationKeyBytes []byte) bool {
	proofReader := bytes.NewReader(proofBytes)
	proof := plonk.NewProof(ecc.BLS12_381)
	_, err := proof.ReadFrom(proofReader)

	// If the proof can't be deserialized from the bytes then it doesn't verifies
	if err != nil {
		return false
	}

	pubInputReader := bytes.NewReader(pubInputBytes)
	pubInput, err := witness.New(ecc.BLS12_381.ScalarField())
	if err != nil {
		panic("Error instantiating witness")
	}
	_, err = pubInput.ReadFrom(pubInputReader)
	if err != nil {
		panic("Could not read PLONK public input")
	}
	verificationKeyReader := bytes.NewReader(verificationKeyBytes)
	verificationKey := plonk.NewVerifyingKey(ecc.BLS12_381)
	_, err = verificationKey.ReadFrom(verificationKeyReader)
	if err != nil {
		panic("Could not read PLONK verifying key from bytes")
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
