package pkg

import (
	"context"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/types"
	"log"
)

type TaskSender struct {
	avsWriter *chainio.AvsWriter
}

func NewTaskSender(avsWriter *chainio.AvsWriter) *TaskSender {
	return &TaskSender{
		avsWriter: avsWriter,
	}
}

func (ts *TaskSender) SendTask(task *types.Task) error {
	log.Println("Sending task...")
	_, index, err := ts.avsWriter.SendTask(
		context.Background(),
		task.ProvingSystem,
		task.Proof,
		task.PublicInput,
		task.Fee,
	)
	if err != nil {
		return err
	}
	log.Println("Task sent successfully. Task index:", index)
	return nil
}
