package pkg

func (agg *Aggregator) SubscribeToNewTasks() error {
	err := agg.subscribeToNewTasks()
	if err != nil {
		return err
	}

	for {
		select {
		case err := <-agg.taskSubscriber:
			agg.AggregatorConfig.BaseConfig.Logger.Info("Failed to subscribe to new tasks", "err", err)

			err = agg.subscribeToNewTasks()
			if err != nil {
				return err
			}
		case newBatch := <-agg.NewBatchChan:
			agg.AddNewTask(newBatch.BatchMerkleRoot, newBatch.TaskCreatedBlock)
		}
	}
}
func (agg *Aggregator) SubscribeToNewTasksV2() error {
	err := agg.subscribeToNewTasks()
	if err != nil {
		return err
	}

	for {
		select {
		case err := <-agg.taskSubscriber:
			agg.AggregatorConfig.BaseConfig.Logger.Info("Failed to subscribe to new tasks", "err", err)

			err = agg.subscribeToNewTasks()
			if err != nil {
				return err
			}
		case newBatch := <-agg.NewBatchChanV2:
			agg.AddNewTaskV2(newBatch.BatchMerkleRoot, newBatch.SenderAddress, newBatch.TaskCreatedBlock)
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
