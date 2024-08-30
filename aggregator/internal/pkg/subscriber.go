package pkg

func (agg *Aggregator) SubscribeToNewTasks() error {
	err := agg.subscribeToNewTasksV2()
	if err != nil {
		return err
	}

	for {
		select {
		case err := <-agg.taskSubscriber:
			agg.AggregatorConfig.BaseConfig.Logger.Info("Failed to subscribe to new tasks", "err", err)
			err = agg.subscribeToNewTasksV2()
			if err != nil {
				return err
			}
		case newBatchV2 := <-agg.NewBatchChanV2:
			agg.AggregatorConfig.BaseConfig.Logger.Info("Adding new task, V2")
			agg.AddNewTaskV2(newBatchV2.BatchMerkleRoot, newBatchV2.SenderAddress, newBatchV2.TaskCreatedBlock)
		}
	}
}

func (agg *Aggregator) subscribeToNewTasksV2() error {
	var err error

	agg.taskSubscriber, err = agg.avsSubscriber.SubscribeToNewTasksV2(agg.NewBatchChanV2)

	if err != nil {
		agg.AggregatorConfig.BaseConfig.Logger.Info("Failed to create task subscriber", "err", err)
	}

	return err
}
