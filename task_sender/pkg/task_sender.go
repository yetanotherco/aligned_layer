package pkg

import (
	"fmt"
	"github.com/urfave/cli/v2"
	"log"
)

func SendTask(c *cli.Context) error {
	log.Println("Sending task...")

	// TODO get system
	proof, err := loadProofFile(c.String("proof"))
	if err != nil {
		return fmt.Errorf("error loading proof file: %v", err)
	}
	log.Println("Proof:", string(proof))

	publicInput, err := loadPublicInputFile(c.String("public-input"))
	if err != nil {
		return fmt.Errorf("error loading public input file: %v", err)
	}
	log.Println("Public Input:", string(publicInput))

	if len(c.String("verification-key")) > 0 {
		verificationKey, err := loadVerificationKeyFile(c.String("verification-key"))
		if err != nil {
			return fmt.Errorf("error loading verification key file: %v", err)
		}
		log.Println("Verification Key:", string(verificationKey))
	}

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

func loadProofFile(proofPath string) ([]byte, error) {
	return readFile(proofPath)
}

func loadPublicInputFile(publicInputPath string) ([]byte, error) {
	return readFile(publicInputPath)
}

func loadVerificationKeyFile(verificationKeyPath string) ([]byte, error) {
	return readFile(verificationKeyPath)
}
