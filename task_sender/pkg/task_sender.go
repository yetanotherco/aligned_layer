package pkg

import (
	"fmt"
	"log"
)

type Task struct {
	proof           []byte
	publicInput     []byte
	verificationKey []byte
}

func NewTask(proof []byte, publicInput []byte, verificationKey []byte) *Task {
	return &Task{
		proof:           proof,
		publicInput:     publicInput,
		verificationKey: verificationKey,
	}
}

func SendTask(task *Task) error {
	log.Println("Sending task...")
	log.Println("Proof:", task.proof)
	log.Println("Public input:", task.publicInput)
	log.Println("Verification key:", task.verificationKey)
	return nil
}

// TODO Change to VerificationSystem instead of String
func getVerificationSystem(system string) (string, error) {
	switch system {
	case "plonk":
		return "plonk", nil
	case "groth16":
		return "groth16", nil
	default:
		return "", fmt.Errorf("unsupported proof system: %s", system)
	}
}

func validateVerificationSystem(system string) error {
	// TODO
	return nil
}

/*
NOTE may be useful
func validateSystem(system string) error {
	validSystems := map[string]bool{"plonk": true, "groth16": true}
	if !validSystems[system] {
		return fmt.Errorf("error: '%s' is not a supported proof system. Supported systems are plonk and groth16", system)
	}
	return nil
}
*/
