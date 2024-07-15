package pkg

func (agg *Aggregator) SubscribeToNewTasks() error {
	err := agg.subscribeToNewTasks()
	if err != nil {
		return err
	}

	for {
		select {
		case err := <-agg.taskSubscriber.Err():
			agg.AggregatorConfig.BaseConfig.Logger.Info("Failed to subscribe to new tasks", "err", err)
			agg.taskSubscriber.Unsubscribe()
			err = agg.subscribeToNewTasks()
			if err != nil {
				return err
			}
		case newBatch := <-agg.NewBatchChan:
			agg.AddNewTask(newBatch.BatchMerkleRoot, newBatch.TaskCreatedBlock)
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
