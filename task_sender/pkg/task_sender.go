package pkg

import (
	"context"
	"fmt"
	"github.com/yetanotherco/aligned_layer/common"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/core/tests/mocks"
	"log"
)

type Task struct {
	verificationSystem common.SystemVerificationId
	proof              []byte
	publicInput        []byte
	verificationKey    []byte
}

func NewTask(verificationSystem common.SystemVerificationId, proof []byte, publicInput []byte, verificationKey []byte) *Task {
	return &Task{
		verificationSystem: verificationSystem,
		proof:              proof,
		publicInput:        publicInput,
		verificationKey:    verificationKey,
	}
}

func SendTask(task *Task) error {
	log.Println("Sending task...")
	avsWriter, err := chainio.NewAvsWriterFromConfig(newDummyConfig())
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
func GetVerificationSystem(system string) (common.SystemVerificationId, error) {
	var unknownValue common.SystemVerificationId
	switch system {
	case "plonk":
		return common.GnarkPlonkBls12_381, nil
	default:
		return unknownValue, fmt.Errorf("unsupported proof system: %s", system)
	}
}

func newDummyConfig() *config.Config {
	ecdsaPrivateKey := "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
	operatorStateRetrieverAddr := "0x9d4454b023096f34b160d6b654540c56a1f81688"
	serviceManagerAddr := "0xc5a5c42992decbae36851359345fe25997f5c42d"
	mockConfig := mocks.NewMockConfig(ecdsaPrivateKey, operatorStateRetrieverAddr, serviceManagerAddr)
	return mockConfig
}
