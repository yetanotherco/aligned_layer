package pkg

import (
	"errors"
	"fmt"
	"time"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
)

const (
	MaxRetries    = 100
	RetryInterval = 1 * time.Second
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
			return err
		case newBatch := <-agg.NewBatchChan:
			agg.AddNewTask(newBatch.BatchMerkleRoot, newBatch.TaskCreatedBlock)
		}
	}
}

func (agg *Aggregator) tryCreateTaskSubscriber() error {
	var err error

	agg.AggregatorConfig.BaseConfig.Logger.Info("Subscribing to Ethereum serviceManager task events")
	agg.taskSubscriber, err = agg.avsSubscriber.AvsContractBindings.ServiceManager.WatchNewBatch(&bind.WatchOpts{},
		agg.NewBatchChan, nil)

	if err != nil {
		agg.AggregatorConfig.BaseConfig.Logger.Info("Failed to create task subscriber", "err", err)
	}
	return err
}
