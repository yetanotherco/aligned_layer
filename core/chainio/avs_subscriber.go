package chainio

import (
	"fmt"

	"github.com/ethereum/go-ethereum/common"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
)

// type AvsSubscriberer interface {
// 	SubscribeToNewTasks(newTaskCreatedChan chan *cstaskmanager.ContractAlignedLayerTaskManagerNewTaskCreated) event.Subscription
// 	SubscribeToTaskResponses(taskResponseLogs chan *cstaskmanager.ContractAlignedLayerTaskManagerTaskResponded) event.Subscription
// 	ParseTaskResponded(rawLog types.Log) (*cstaskmanager.ContractAlignedLayerTaskManagerTaskResponded, error)
// }

// Subscribers use a ws connection instead of http connection like Readers
// kind of stupid that the geth client doesn't have a unified interface for both...
// it takes a single url, so the bindings, even though they have watcher functions, those can't be used
// with the http connection... seems very very stupid. Am I missing something?
type AvsSubscriber struct {
	AvsContractBindings *AvsServiceBindings
	logger              sdklogging.Logger
}

// func NewAvsSubscriberFromConfig(config *config.Config) (*AvsSubscriber, error) {
func NewAvsSubscriberFromConfig() (*AvsSubscriber, error) {
	logger, err := sdklogging.NewZapLogger("development")
	if err != nil {
		fmt.Println("Could not initialize logger")
	}
	alignedLayerServiceManagerAddr := common.HexToAddress("0xc5a5C42992dECbae36851359345FE25997F5C42d")
	operatorStateRetrieverAddr := common.HexToAddress("0x9d4454B023096f34B160D6B654540c56A1F81688")

	ethWsClient, err := eth.NewClient("ws://localhost:8545")
	if err != nil {
		panic(err)
	}

	avsContractBindings, err := NewAvsServiceBindings(alignedLayerServiceManagerAddr, operatorStateRetrieverAddr, ethWsClient, logger)
	if err != nil {
		logger.Errorf("Failed to create contract bindings", "err", err)
		return nil, err
	}

	return &AvsSubscriber{
		AvsContractBindings: avsContractBindings,
		logger:              logger,
	}, nil
}

// func (s *AvsSubscriber) SubscribeToNewTasks(newTaskCreatedChan chan *cstaskmanager.ContractAlignedLayerTaskManagerNewTaskCreated) event.Subscription {
// 	sub, err := s.AvsContractBindings.TaskManager.WatchNewTaskCreated(
// 		&bind.WatchOpts{}, newTaskCreatedChan, nil,
// 	)
// 	if err != nil {
// 		s.logger.Error("Failed to subscribe to new TaskManager tasks", "err", err)
// 	}
// 	s.logger.Infof("Subscribed to new TaskManager tasks")
// 	return sub
// }

// func (s *AvsSubscriber) SubscribeToTaskResponses(taskResponseChan chan *cstaskmanager.ContractAlignedLayerTaskManagerTaskResponded) event.Subscription {
// 	sub, err := s.AvsContractBindings.TaskManager.WatchTaskResponded(
// 		&bind.WatchOpts{}, taskResponseChan,
// 	)
// 	if err != nil {
// 		s.logger.Error("Failed to subscribe to TaskResponded events", "err", err)
// 	}
// 	s.logger.Infof("Subscribed to TaskResponded events")
// 	return sub
// }

// func (s *AvsSubscriber) ParseTaskResponded(rawLog types.Log) (*cstaskmanager.ContractAlignedLayerTaskManagerTaskResponded, error) {
// 	return s.AvsContractBindings.TaskManager.ContractAlignedLayerTaskManagerFilterer.ParseTaskResponded(rawLog)
// }
