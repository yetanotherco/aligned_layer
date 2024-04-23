package pkg

import (
	"fmt"
	"github.com/yetanotherco/aligned_layer/common"
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
	log.Println("Verification system:", task.verificationSystem)
	log.Println("Proof:", task.proof)
	log.Println("Public input:", task.publicInput)
	log.Println("Verification key:", task.verificationKey)
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
