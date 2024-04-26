package pkg

import "github.com/yetanotherco/aligned_layer/core/types"

func (agg *Aggregator) SubscribeToNewTasks() error {
	for {
		select {
		case err := <-agg.taskSubscriber.Err():
			// TODO: Retry subscription
			agg.AggregatorConfig.BaseConfig.Logger.Error("Error in subscription", "err", err)
			return err
		case task := <-agg.NewTaskCreatedChan:
			agg.AggregatorConfig.BaseConfig.Logger.Info("New task created", "taskIndex", task.TaskIndex,
				"task", task.Task)

			agg.tasksMutex.Lock()
			agg.tasks[task.TaskIndex] = task.Task
			agg.tasksMutex.Unlock()

			agg.taskResponsesMutex.Lock()
			agg.taskResponses[task.TaskIndex] = &TaskResponses{
				taskResponses: make([]types.SignedTaskResponse, 0),
				responded:     false,
			}
			agg.taskResponsesMutex.Unlock()
		}
	}
}
