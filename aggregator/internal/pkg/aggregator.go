package pkg

import (
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/event"
	"github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/core/types"
)

type Aggregator struct {
	AggregatorConfig   *config.AggregatorConfig
	NewTaskCreatedChan chan *contractAlignedLayerServiceManager.ContractAlignedLayerServiceManagerNewTaskCreated
	avsSubscriber      *chainio.AvsSubscriber
	taskSubscriber     event.Subscription

	// Using map here instead of slice to allow for easy lookup of tasks, when aggregator is restarting,
	// its easier to get the task from the map instead of filling the slice again
	tasks map[uint64]contractAlignedLayerServiceManager.AlignedLayerServiceManagerTask

	taskResponses map[uint64][]types.SignedTaskResponse
}

func NewAggregator(aggregatorConfig config.AggregatorConfig) (*Aggregator, error) {
	newTaskCreatedChan := make(chan *contractAlignedLayerServiceManager.ContractAlignedLayerServiceManagerNewTaskCreated)
	avsSubscriber, err := chainio.NewAvsSubscriberFromConfig(aggregatorConfig.BaseConfig)
	if err != nil {
		return nil, err
	}

	taskSubscriber, err := avsSubscriber.AvsContractBindings.ServiceManager.WatchNewTaskCreated(&bind.WatchOpts{},
		newTaskCreatedChan, nil)
	if err != nil {
		return nil, err
	}

	tasks := make(map[uint64]contractAlignedLayerServiceManager.AlignedLayerServiceManagerTask)
	taskResponses := make(map[uint64][]types.SignedTaskResponse)

	aggregator := Aggregator{
		AggregatorConfig:   &aggregatorConfig,
		avsSubscriber:      avsSubscriber,
		taskSubscriber:     taskSubscriber,
		NewTaskCreatedChan: newTaskCreatedChan,
		tasks:              tasks,
		taskResponses:      taskResponses,
	}

	// Return the Aggregator instance
	return &aggregator, nil
}

func (aggregator *Aggregator) ListenForTasks() error {
	for {
		select {
		case err := <-aggregator.taskSubscriber.Err():
			aggregator.AggregatorConfig.BaseConfig.Logger.Error("Error in subscription", "err", err)
			return err
		case task := <-aggregator.NewTaskCreatedChan:
			aggregator.AggregatorConfig.BaseConfig.Logger.Info("New task created", "taskIndex", task.TaskIndex,
				"task", task.Task)
			aggregator.tasks[task.TaskIndex] = task.Task
			aggregator.taskResponses[task.TaskIndex] = make([]types.SignedTaskResponse, 0)
		}
	}
}
