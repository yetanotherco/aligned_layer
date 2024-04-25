package chainio

import (
	"github.com/yetanotherco/aligned_layer/core/config"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
)

// NOTE(marian): Leaving this commented code here as it may be useful in the short term.
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

func NewAvsSubscriberFromConfig(c *config.BaseConfig) (*AvsSubscriber, error) {
	avsContractBindings, err := NewAvsServiceBindings(c.AlignedLayerDeploymentConfig.AlignedLayerServiceManagerAddr, c.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr, c.EthWsClient, c.Logger)

	if err != nil {
		c.Logger.Errorf("Failed to create contract bindings", "err", err)
		return nil, err
	}

	return &AvsSubscriber{
		AvsContractBindings: avsContractBindings,
		logger:              c.Logger,
	}, nil
}

// NOTE(marian): Leaving this commented code here as it may be useful in the short term.
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
