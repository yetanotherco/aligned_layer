package operator

import (
	"context"
	"crypto/ecdsa"
	"log"
	"time"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	"github.com/Layr-Labs/eigensdk-go/logging"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/event"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
)

type Operator struct {
	Config             config.OperatorConfig
	Address            common.Address
	Socket             string
	Timeout            time.Duration
	PrivKey            *ecdsa.PrivateKey
	KeyPair            *bls.KeyPair
	OperatorId         eigentypes.OperatorId
	avsSubscriber      chainio.AvsSubscriber
	NewTaskCreatedChan chan *servicemanager.ContractAlignedLayerServiceManagerNewTaskCreated
	Logger             logging.Logger
}

func NewOperatorFromConfig(config config.OperatorConfig) (*Operator, error) {
	logger := config.BaseConfig.Logger

	deploymentConfig := config.AlignedLayerDeploymentConfig

	serviceManagerAddr := deploymentConfig.AlignedLayerServiceManagerAddr
	operatorStateRetrieverAddr := deploymentConfig.AlignedLayerOperatorStateRetrieverAddr
	ethWsUrl := config.BaseConfig.EthWsUrl
	ethWsClient, err := eth.NewClient(ethWsUrl)
	if err != nil {
		log.Fatalln("Cannot create websocket ethereum client", "err", err)
		return nil, err
	}

	avsSubscriber, err := chainio.NewAvsSubscriberFromConfig(serviceManagerAddr, operatorStateRetrieverAddr, ethWsClient, logger)
	if err != nil {
		log.Fatalf("Could not create AVS subscriber")
	}
	newTaskCreatedChan := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewTaskCreated)

	address := config.Operator.Address
	operator := &Operator{
		Config:             config,
		Logger:             logger,
		avsSubscriber:      *avsSubscriber,
		Address:            address,
		NewTaskCreatedChan: newTaskCreatedChan,
		// KeyPair
		// PrivKey
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
			// taskResponse := o.ProcessNewTaskCreatedLog(newTaskCreatedLog)
			// signedTaskResponse, err := o.SignTaskResponse(taskResponse)
			// if err != nil {
			// 	continue
			// }
			// go o.aggregatorRpcClient.SendSignedTaskResponseToAggregator(signedTaskResponse)

			log.Printf("The received task's index is: %d\n", newTaskCreatedLog.TaskIndex)
		}
	}
}
