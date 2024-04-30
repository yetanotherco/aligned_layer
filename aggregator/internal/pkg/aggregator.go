package pkg

import (
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
	avsReader          *chainio.AvsReader
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

	avsReader, err := chainio.NewAvsReaderFromConfig(aggregatorConfig.BaseConfig, aggregatorConfig.EcdsaConfig)
	if err != nil {
		return nil, err
	}

	avsSubscriber, err := chainio.NewAvsSubscriberFromConfig(aggregatorConfig.BaseConfig)
	if err != nil {
		return nil, err
	}

	avsWriter, err := chainio.NewAvsWriterFromConfig(aggregatorConfig.BaseConfig, aggregatorConfig.EcdsaConfig)
	if err != nil {
		return nil, err
	}

	tasks := make(map[uint64]contractAlignedLayerServiceManager.AlignedLayerServiceManagerTask)
	taskResponses := make(map[uint64]*TaskResponsesWithStatus, 0)

	aggregator := Aggregator{
		AggregatorConfig:   &aggregatorConfig,
		avsReader:          avsReader,
		avsSubscriber:      avsSubscriber,
		avsWriter:          avsWriter,
		NewTaskCreatedChan: newTaskCreatedChan,
		tasks:              tasks,
		tasksMutex:         &sync.Mutex{},
		taskResponses:      taskResponses,
		taskResponsesMutex: &sync.Mutex{},
	}

	// Return the Aggregator instance
	return &aggregator, nil
}

func (agg *Aggregator) AddNewTask(index uint64, task contractAlignedLayerServiceManager.AlignedLayerServiceManagerTask) {
	agg.AggregatorConfig.BaseConfig.Logger.Info("Adding new task", "taskIndex", index, "task", task)
	agg.tasksMutex.Lock()
	agg.tasks[index] = task
	agg.tasksMutex.Unlock()
	agg.taskResponsesMutex.Lock()
	agg.taskResponses[index] = &TaskResponsesWithStatus{
		taskResponses:       make([]types.SignedTaskResponse, 0),
		submittedToEthereum: false,
	}
	agg.taskResponsesMutex.Unlock()
}
