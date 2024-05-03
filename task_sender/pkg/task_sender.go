package pkg

import (
	"context"
	"log"

	"github.com/Layr-Labs/eigensdk-go/types"
	"github.com/yetanotherco/aligned_layer/common"
	"github.com/yetanotherco/aligned_layer/core/chainio"
)

type Task struct {
	ProvingSystem              common.ProvingSystemId
	Proof                      []byte
	PublicInput                []byte
	VerificationKey            []byte
	QuorumNumbers              types.QuorumNums
	QuorumThresholdPercentages types.QuorumThresholdPercentages
}

func NewTask(provingSystemId common.ProvingSystemId, proof []byte, publicInput []byte, verificationKey []byte, quorumNumbers types.QuorumNums, quorumThresholdPercentages types.QuorumThresholdPercentages) *Task {
	return &Task{
		ProvingSystem:              provingSystemId,
		Proof:                      proof,
		PublicInput:                publicInput,
		VerificationKey:            verificationKey,
		QuorumNumbers:              quorumNumbers,
		QuorumThresholdPercentages: quorumThresholdPercentages,
	}
}

type TaskSender struct {
	avsWriter *chainio.AvsWriter
}

func NewTaskSender(avsWriter *chainio.AvsWriter) *TaskSender {
	return &TaskSender{
		avsWriter: avsWriter,
	}
}

func (ts *TaskSender) SendTask(task *Task) error {
	log.Println("Sending task...")
	_, index, err := ts.avsWriter.SendTask(
		context.Background(),
		task.ProvingSystem,
		task.Proof,
		task.PublicInput,
		task.VerificationKey,
		task.QuorumNumbers,
		task.QuorumThresholdPercentages,
	)
	if err != nil {
		return err
	}
	log.Println("Task sent successfully. Task index:", index)
	return nil
}
