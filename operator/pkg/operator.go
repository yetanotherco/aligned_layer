package operator

import (
	"context"
	"log"

	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/event"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"golang.org/x/crypto/sha3"

	"github.com/yetanotherco/aligned_layer/core/config"
)

type Operator struct {
	Config             config.OperatorConfig
	Address            common.Address
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
			log.Println("Operator shutting down...")
			return nil
		case err := <-sub.Err():
			log.Println("Error in websocket subscription", "err", err)
			sub.Unsubscribe()
			sub = o.SubscribeToNewTasks()
		case newTaskCreatedLog := <-o.NewTaskCreatedChan:

			/* --------- OPERATOR MAIN LOGIC --------- */
			log.Printf("The received task's index is: %d\n", newTaskCreatedLog.TaskIndex)

			// Here we should process a task, here we will pretend the proof is always true until adding that
			taskResponse := servicemanager.AlignedLayerServiceManagerTaskResponse{TaskIndex: newTaskCreatedLog.TaskIndex, ProofIsCorrect: true}
			encodedResponseBytes, _ := AbiEncodeTaskResponse(taskResponse)
			log.Println("Task response:", taskResponse)
			log.Println("ABI Encoded bytes:\n", encodedResponseBytes)

			var taskResponseDigest [32]byte
			hasher := sha3.NewLegacyKeccak256()
			hasher.Write(encodedResponseBytes)
			copy(taskResponseDigest[:], hasher.Sum(nil)[:32])
			log.Println("Encoded response hash:", taskResponseDigest)
			log.Println("Encoded response hash len:", len(taskResponseDigest))
			responseSignature := *o.Config.BlsConfig.KeyPair.SignMessage(taskResponseDigest)
			log.Println("Signed hash:", responseSignature)
		}
	}
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
