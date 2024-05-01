package pkg

import (
	"errors"
	"fmt"
	"time"

	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
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
		case newTask := <-agg.NewTaskCreatedChan:
			agg.AggregatorConfig.BaseConfig.Logger.Info("New task created", "taskIndex", newTask.TaskIndex)

			agg.tasksMutex.Lock()
			agg.tasks[newTask.TaskIndex] = newTask.Task
			agg.tasksMutex.Unlock()

			agg.taskResponsesMutex.Lock()
			agg.OperatorTaskResponses[newTask.TaskIndex] = &TaskResponsesWithStatus{
				taskResponses:       make([]types.SignedTaskResponse, 0),
				submittedToEthereum: false,
			}
			agg.taskResponsesMutex.Unlock()

			// DUMMY VALUES TO MAKE IT WORK
			q := make([]eigentypes.QuorumNum, 1)
			q[0] = 0
			qPorcentage := make([]eigentypes.QuorumThresholdPercentage, 1)
			qPorcentage[0] = 100
			// ----------------------------

			// agg.blsAggregationService.InitializeNewTask(taskIndex, newTask.TaskCreatedBlock, newTask.Task.QuorumNumbers, quorumThresholdPercentages, taskTimeToExpiry)
			agg.blsAggregationService.InitializeNewTask(newTask.TaskIndex, newTask.Task.TaskCreatedBlock, eigentypes.QuorumNums(q), qPorcentage, 100*time.Second)
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
