package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"path/filepath"

	"github.com/urfave/cli/v2"
)

func verify(system, proofPath, publicInputPath, verificationKeyPath string) error {
	proof, err := readFile(proofPath)
	if err != nil {
		return fmt.Errorf("error reading proof file: %v", err)
	}
	publicInput, err := readFile(publicInputPath)
	if err != nil {
		return fmt.Errorf("error reading public input file: %v", err)
	}
	verificationKey, err := readFile(verificationKeyPath)
	if err != nil {
		return fmt.Errorf("error reading verification key file: %v", err)
	}

	fmt.Printf("Verifying using %s...\n", system)
	fmt.Println("Proof:", string(proof))
	fmt.Println("Public Input:", string(publicInput))
	fmt.Println("Verification Key:", string(verificationKey))

	return nil
}

func readFile(filePath string) ([]byte, error) {
	data, err := ioutil.ReadFile(filePath)
	if err != nil {
		return nil, err
	}
	return data, nil
}

func runCLI() {
	app := &cli.App{
		Name:      "Aligned Layer Verifier",
		UsageText: "\n" + "send-task plonk proof.txt public_input_file " + "verification_key_file" + "\n" + "send-task groth16 proof.txt public_input_file " + "verification_key_file",
		Commands: []*cli.Command{
			{
				Name:  "send-task",
				Usage: "verify a proof using a specified proof system",
				Action: func(c *cli.Context) error {
					if c.NArg() < 4 {
						fmt.Println("Error: Insufficient arguments")
						cli.ShowCommandHelp(c, "send-task")
						return nil
					}

					system := c.Args().Get(0)
					executableDir, err := os.Executable()
					if err != nil {
						return fmt.Errorf("error getting executable path: %v", err)
					}
					validSystems := map[string]bool{"plonk": true, "groth16": true}
					if !validSystems[system] {
						fmt.Printf("Error: '%s' is not a supported proof system. Supported systems are plonk and groth16\n", system)
						return nil
					}
					proofPath := filepath.Join(filepath.Dir(executableDir), c.Args().Get(1))
					publicInputPath := filepath.Join(filepath.Dir(executableDir), c.Args().Get(2))
					verificationKeyPath := filepath.Join(filepath.Dir(executableDir), c.Args().Get(3))

					return verify(system, proofPath, publicInputPath, verificationKeyPath)
				},
				UsageText:   "send-task plonk proof_file public_input_file verification_key",
				Description: "Verifies a proof using a specified proof system. Provide the name of the proof system (e.g., plonk or groth16) along with the paths to the proof file, public input file, and verification key file.",
			},
		},
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}
