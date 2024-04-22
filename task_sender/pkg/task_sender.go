package pkg

import (
	"fmt"
	"log"
)

type Task struct {
	verificationSystem string
	proof              []byte
	publicInput        []byte
	verificationKey    []byte
}

func NewTask(verificationSystem string, proof []byte, publicInput []byte, verificationKey []byte) *Task {
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

// TODO Change to VerificationSystem instead of String
func GetVerificationSystem(system string) (string, error) {
	switch system {
	case "plonk":
		return "plonk", nil
	case "groth16":
		return "groth16", nil
	default:
		return "", fmt.Errorf("unsupported proof system: %s", system)
	}
}
