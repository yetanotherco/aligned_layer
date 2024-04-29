package pkg

import (
	"context"
	"fmt"
	"log"

	"github.com/yetanotherco/aligned_layer/common"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/tests/mocks"
)

type Task struct {
	verificationSystem common.ProvingSystemId
	proof              []byte
	publicInput        []byte
	verificationKey    []byte
}

func NewTask(verificationSystem common.ProvingSystemId, proof []byte, publicInput []byte, verificationKey []byte) *Task {
	return &Task{
		verificationSystem: verificationSystem,
		proof:              proof,
		publicInput:        publicInput,
		verificationKey:    verificationKey,
	}
}

func SendTask(task *Task) error {
	log.Println("Sending task...")
	avsWriter, err := chainio.NewAvsWriterFromConfig(mocks.NewMockConfig())
	if err != nil {
		return err
	}

	_, index, err := avsWriter.SendTask(
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
func GetVerificationSystem(system string) (common.ProvingSystemId, error) {
	var unknownValue common.ProvingSystemId
	switch system {
	case "plonk":
		return common.GnarkPlonkBls12_381, nil
	default:
		return unknownValue, fmt.Errorf("unsupported proof system: %s", system)
	}
}
