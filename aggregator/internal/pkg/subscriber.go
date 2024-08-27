package pkg

func (agg *Aggregator) SubscribeToNewTasks() error {
	err := agg.subscribeToNewTasks()
	if err != nil {
		return err
	}
	err = agg.subscribeToNewTasksV2()
	if err != nil {
		return err
	}

	V2 := false

	for {
		select {
		case err := <-agg.taskSubscriber:
			agg.AggregatorConfig.BaseConfig.Logger.Info("Failed to subscribe to new tasks", "err", err)

			if !V2 {
				err = agg.subscribeToNewTasks()
				if err != nil {
					return err
				}
			} else {
				err = agg.subscribeToNewTasksV2()
				if err != nil {
					return err
				}
			}
		case newBatch := <-agg.NewBatchChan:
			if !V2 {
				agg.AggregatorConfig.BaseConfig.Logger.Info("Adding new task, V1")
				agg.AddNewTask(newBatch.BatchMerkleRoot, newBatch.TaskCreatedBlock)
			}
		case newBatchV2 := <-agg.NewBatchChanV2:
			if V2 {
				agg.AggregatorConfig.BaseConfig.Logger.Info("Adding new task, V2")
				agg.AddNewTaskV2(newBatchV2.BatchMerkleRoot, newBatchV2.SenderAddress, newBatchV2.TaskCreatedBlock)
			}
		}
	}
}

func (agg *Aggregator) subscribeToNewTasks() error {
	var err error

	agg.taskSubscriber, err = agg.avsSubscriber.SubscribeToNewTasks(agg.NewBatchChan)

	if err != nil {
		agg.AggregatorConfig.BaseConfig.Logger.Info("Failed to create task subscriber", "err", err)
	}

	return err
}
func (agg *Aggregator) subscribeToNewTasksV2() error {
	var err error

	agg.taskSubscriber, err = agg.avsSubscriber.SubscribeToNewTasksV2(agg.NewBatchChanV2)

	if err != nil {
		agg.AggregatorConfig.BaseConfig.Logger.Info("Failed to create task subscriber", "err", err)
	}

	return err
}
