package pkg

import "github.com/yetanotherco/aligned_layer/core/chainio"

func (agg *Aggregator) SubscribeToNewTasks() *chainio.ErrorPair {
	errorPair := agg.subscribeToNewTasks()
	if errorPair != nil {
		return errorPair
	}

	for {
		select {
		case err := <-agg.taskSubscriber:
			agg.AggregatorConfig.BaseConfig.Logger.Info("Failed to subscribe to new tasks", "err", err)
			errorPair = agg.subscribeToNewTasks()
			if errorPair != nil {
				return errorPair
			}
		case newBatch := <-agg.NewBatchChan:
			agg.AggregatorConfig.BaseConfig.Logger.Info("Adding new task")
			agg.AddNewTask(newBatch.BatchMerkleRoot, newBatch.SenderAddress, newBatch.TaskCreatedBlock)
		}
	}
}

func (agg *Aggregator) subscribeToNewTasks() *chainio.ErrorPair {
	errorPair := agg.avsSubscriber.SubscribeToNewTasksV3(agg.NewBatchChan, agg.taskSubscriber)

	if errorPair != nil {
		agg.AggregatorConfig.BaseConfig.Logger.Info("Failed to create task subscriber", "err", errorPair)
	}

	return errorPair
}
