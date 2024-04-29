package pkg

import (
	"errors"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/event"
	"github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/core/types"
	"sync"
)

// Aggregator stores TaskResponse for a task here
type TaskResponsesWithStatus struct {
	taskResponses       []types.SignedTaskResponse
	submittedToEthereum bool
}

type Aggregator struct {
	AggregatorConfig   *config.AggregatorConfig
	NewTaskCreatedChan chan *contractAlignedLayerServiceManager.ContractAlignedLayerServiceManagerNewTaskCreated
	avsSubscriber      *chainio.AvsSubscriber
	avsWriter          *chainio.AvsWriter
	taskSubscriber     event.Subscription

	// Using map here instead of slice to allow for easy lookup of tasks, when aggregator is restarting,
	// its easier to get the task from the map instead of filling the slice again
	tasks map[uint64]contractAlignedLayerServiceManager.AlignedLayerServiceManagerTask
	// Mutex to protect the tasks map
	tasksMutex *sync.Mutex

	taskResponses map[uint64]*TaskResponsesWithStatus
	// Mutex to protect the taskResponses map
	taskResponsesMutex *sync.Mutex
}

func NewAggregator(aggregatorConfig config.AggregatorConfig) (*Aggregator, error) {
	newTaskCreatedChan := make(chan *contractAlignedLayerServiceManager.ContractAlignedLayerServiceManagerNewTaskCreated)

	avsSubscriber, err := chainio.NewAvsSubscriberFromConfig(aggregatorConfig.BaseConfig)
	if err != nil {
		return nil, err
	}

	avsWriter, err := chainio.NewAvsWriterFromConfig(aggregatorConfig.BaseConfig, aggregatorConfig.EcdsaConfig)
	if err != nil {
		return nil, err
	}

	// Subscriber to Ethereum serviceManager task events
	taskSubscriber, err := avsSubscriber.AvsContractBindings.ServiceManager.WatchNewTaskCreated(&bind.WatchOpts{},
		newTaskCreatedChan, nil)
	if err != nil {
		return nil, err
	}

	aggregatorConfig.BaseConfig.Logger.Info("Created subscriber for task creation events")

	tasks := make(map[uint64]contractAlignedLayerServiceManager.AlignedLayerServiceManagerTask)
	taskResponses := make(map[uint64]*TaskResponsesWithStatus, 0)

	aggregator := Aggregator{
		AggregatorConfig:   &aggregatorConfig,
		avsSubscriber:      avsSubscriber,
		avsWriter:          avsWriter,
		taskSubscriber:     taskSubscriber,
		NewTaskCreatedChan: newTaskCreatedChan,
		tasks:              tasks,
		tasksMutex:         &sync.Mutex{},
		taskResponses:      taskResponses,
		taskResponsesMutex: &sync.Mutex{},
	}

	// Return the Aggregator instance
	return &aggregator, nil
}

func (agg *Aggregator) RespondToTask(taskIndex uint64, proofIsCorrect bool) error {
	fullTask, ok := agg.tasks[taskIndex]
	if !ok {
		agg.AggregatorConfig.BaseConfig.Logger.Error("Task does not exist", "taskIndex", taskIndex)
		return nil
	}

	txOpts := agg.avsWriter.Signer.GetTxOpts()

	// Don't send the transaction, just estimate the gas
	txOpts.NoSend = true

	tx, err := agg.avsWriter.AvsContractBindings.ServiceManager.RespondToTask(
		txOpts, taskIndex, proofIsCorrect)
	if err != nil {
		agg.AggregatorConfig.BaseConfig.Logger.Error("Error in responding to task", "err", err)
		return err
	}

	if tx.Cost().Cmp(fullTask.Fee) > 0 {
		agg.AggregatorConfig.BaseConfig.Logger.Error("Gas estimate is higher than the task fee", "gas", tx.Cost(), "fee", fullTask.Fee)

		// return error
		return errors.New("gas estimate is higher than the task fee")
	}

	txOpts.NoSend = false
	txOpts.GasLimit = tx.Gas()
	txOpts.GasPrice = tx.GasPrice()

	_, err = agg.avsWriter.AvsContractBindings.ServiceManager.RespondToTask(
		txOpts, taskIndex, proofIsCorrect)
	if err != nil {
		return err
	}

	agg.AggregatorConfig.BaseConfig.Logger.Info("Submitted task response to contract", "taskIndex", taskIndex, "proofIsValid", proofIsCorrect)

	return nil
}
