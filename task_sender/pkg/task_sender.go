package pkg

import (
	"context"
	"log"
	"time"

	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
)

type Task struct {
	BatchMerkleRoot  [32]byte
	batchDataPointer string
}

func NewTask(batchMerkleRoot [32]byte, batchDataPointer string) *Task {
	return &Task{
		BatchMerkleRoot:  batchMerkleRoot,
		batchDataPointer: batchDataPointer,
	}
}

type TaskSender struct {
	avsWriter      *chainio.AvsWriter
	eigenDAConfig  *config.EigenDADisperserConfig
	celestiaConfig *config.CelestiaConfig
}

const RetryInterval = 1 * time.Second

func NewTaskSender(config *config.TaskSenderConfig, avsWriter *chainio.AvsWriter) *TaskSender {
	return &TaskSender{
		avsWriter:      avsWriter,
		eigenDAConfig:  config.EigenDADisperserConfig,
		celestiaConfig: config.CelestiaConfig,
	}
}

func (ts *TaskSender) SendTask(task *Task) error {
	log.Println("Sending task...")
	err := ts.avsWriter.SendTask(
		context.Background(),
		task.BatchMerkleRoot,
		task.batchDataPointer,
	)
	if err != nil {
		return err
	}
	log.Println("Task sent successfully. Batch Merkle Root:", task.BatchMerkleRoot)
	return nil
}
