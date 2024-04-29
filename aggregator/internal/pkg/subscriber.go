package pkg

import (
	"fmt"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/yetanotherco/aligned_layer/core/types"
	"time"
)

const (
	RetryInterval = 10 * time.Second
)

func (agg *Aggregator) SubscribeToNewTasks() error {
	var createErr error
	createErr = agg.tryCreateTaskSubscriber()
	if createErr == nil {
		_ = agg.subscribeToNewTasks() // This will block until an error occurs
	}
	time.Sleep(RetryInterval)
	return agg.SubscribeToNewTasks()
}

func (agg *Aggregator) subscribeToNewTasks() error {
	for {
		select {
		case err := <-agg.taskSubscriber.Err():
			agg.AggregatorConfig.BaseConfig.Logger.Error("Error in subscription", "err", err)
			return err
		case task := <-agg.NewTaskCreatedChan:
			agg.AggregatorConfig.BaseConfig.Logger.Info("New task created", "taskIndex", task.TaskIndex,
				"task", task.Task)

			agg.tasksMutex.Lock()
			agg.tasks[task.TaskIndex] = task.Task
			agg.tasksMutex.Unlock()

			agg.taskResponsesMutex.Lock()
			agg.taskResponses[task.TaskIndex] = &TaskResponsesWithStatus{
				taskResponses:       make([]types.SignedTaskResponse, 0),
				submittedToEthereum: false,
			}
			agg.taskResponsesMutex.Unlock()
		}
	}
}

func (agg *Aggregator) tryCreateTaskSubscriber() error {
	var err error

	agg.AggregatorConfig.BaseConfig.Logger.Info("Subscribing to Ethereum serviceManager task events")
	agg.taskSubscriber, err = agg.avsSubscriber.AvsContractBindings.ServiceManager.WatchNewTaskCreated(&bind.WatchOpts{},
		agg.NewTaskCreatedChan, nil)

	if err != nil {
		message := fmt.Sprintf("Failed to create task subscriber")
		agg.AggregatorConfig.BaseConfig.Logger.Info(message, "err", err)
	}
	return err
}
