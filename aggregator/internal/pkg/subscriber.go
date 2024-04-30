package pkg

import (
	"errors"
	"fmt"
	"time"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/yetanotherco/aligned_layer/core/types"
)

const (
	MaxRetries    = 20
	RetryInterval = 10 * time.Second
)

func (agg *Aggregator) SubscribeToNewTasks() error {
	for retries := 0; retries < MaxRetries; retries++ {
		err := agg.tryCreateTaskSubscriber()
		if err == nil {
			_ = agg.subscribeToNewTasks() // This will block until an error occurs
		}

		message := fmt.Sprintf("Failed to subscribe to new tasks. Retrying in %v", RetryInterval)
		agg.AggregatorConfig.BaseConfig.Logger.Info(message)
		time.Sleep(RetryInterval)
	}

	return errors.New("failed to subscribe to new tasks after max retries")
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
			agg.OperatorTaskResponses[task.TaskIndex] = &TaskResponsesWithStatus{
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
		agg.AggregatorConfig.BaseConfig.Logger.Info("Failed to create task subscriber", "err", err)
	}
	return err
}
