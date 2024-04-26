package pkg

import (
	"context"
	"fmt"
	"github.com/yetanotherco/aligned_layer/common"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"log"
)

type TaskSender struct {
	avsWriter *chainio.AvsWriter
}

type Task struct {
	verificationSystem common.SystemVerificationId
	proof              []byte
	publicInput        []byte
	verificationKey    []byte
}

func NewTaskSender(avsWriter *chainio.AvsWriter) *TaskSender {
	return &TaskSender{
		avsWriter: avsWriter,
	}
}

func NewTask(verificationSystem common.SystemVerificationId, proof []byte, publicInput []byte, verificationKey []byte) *Task {
	return &Task{
		verificationSystem: verificationSystem,
		proof:              proof,
		publicInput:        publicInput,
		verificationKey:    verificationKey,
	}
}

func (ts *TaskSender) SendTask(task *Task) error {
	log.Println("Sending task...")
	_, index, err := ts.avsWriter.SendTask(
		context.Background(),
		task.verificationSystem,
		task.proof,
		task.publicInput,
	)
	if err != nil {
		return err
	}
	log.Println("Task sent successfully. Task index:", index)
	return nil
}

// TODO Set corrects verification systems
func GetVerificationSystem(system string) (common.SystemVerificationId, error) {
	var unknownValue common.SystemVerificationId
	switch system {
	case "plonk":
		return common.GnarkPlonkBls12_381, nil
	default:
		return unknownValue, fmt.Errorf("unsupported proof system: %s", system)
	}
}
