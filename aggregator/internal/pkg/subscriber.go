package pkg

import (
	"fmt"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/yetanotherco/aligned_layer/core/types"
	"time"
)

const (
	MaxRetries    = 5
	RetryInterval = 10 * time.Second
)

func (agg *Aggregator) SubscribeToNewTasks() error {

	var err error

	for attempt := 0; attempt < MaxRetries; attempt++ {
		if err != nil {
			err = tryCreateTaskSubscriber(agg)
		} else {
			err = agg.subscribeToNewTasks()
		}
	}

	return err
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

func tryCreateTaskSubscriber(agg *Aggregator) error {
	var err error

	for attempt := 0; attempt < MaxRetries; attempt++ {
		agg.AggregatorConfig.BaseConfig.Logger.Info("Subscribing to Ethereum serviceManager task events")
		agg.taskSubscriber, err = agg.avsSubscriber.AvsContractBindings.ServiceManager.WatchNewTaskCreated(&bind.WatchOpts{},
			agg.NewTaskCreatedChan, nil)

		if err != nil {
			message := fmt.Sprintf("Failed to create task subscriber, waiting %d seconds before retrying", RetryInterval/time.Second)
			agg.AggregatorConfig.BaseConfig.Logger.Info(message, "err", err)
			time.Sleep(RetryInterval)
		} else {
			break
		}
	}

	return err
}
