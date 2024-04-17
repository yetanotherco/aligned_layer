package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"path/filepath"

	"github.com/urfave/cli/v2"
)

var (
	systemFlag = &cli.StringFlag{
		Name:     "system",
		Aliases:  []string{"s"},
		Required: true,
		Usage:    "the proof system to use (e.g., plonk, groth16)",
	}
	proofFlag = &cli.PathFlag{
		Name:     "proof",
		Aliases:  []string{"p"},
		Required: true,
		Usage:    "path to the proof file",
	}
	publicInputFlag = &cli.PathFlag{
		Name:     "public-input",
		Aliases:  []string{"pi"},
		Required: true,
		Usage:    "path to the public input file",
	}
	verificationKeyFlag = &cli.PathFlag{
		Name:     "verification-key",
		Aliases:  []string{"vk"},
		Required: true,
		Usage:    "path to the verification key file",
	}
)

var flags = []cli.Flag{
	systemFlag,
	proofFlag,
	publicInputFlag,
	verificationKeyFlag,
}

func verifierAction(c *cli.Context) error {
	system := c.String("system")
	executableDir, err := os.Executable()
	if err != nil {
		return fmt.Errorf("error getting executable path: %v", err)
	}
	proofPath := filepath.Join(filepath.Dir(executableDir), c.String("proof"))
	publicInputPath := filepath.Join(filepath.Dir(executableDir), c.String("public-input"))
	verificationKeyPath := filepath.Join(filepath.Dir(executableDir), c.String("verification-key"))

	if err := validateSystem(system); err != nil {
		return err
	}

	return verify(system, proofPath, publicInputPath, verificationKeyPath)
}

func validateSystem(system string) error {
	validSystems := map[string]bool{"plonk": true, "groth16": true}
	if !validSystems[system] {
		return fmt.Errorf("error: '%s' is not a supported proof system. Supported systems are plonk and groth16", system)
	}
	return nil
}

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
		Name:        "Aligned Layer Verifier",
		Usage:       "A CLI to verify cryptographic proofs using specified proof systems",
		Description: "Verifies cryptographic proofs, ensuring they are valid under the specified proof systems.",
		Flags:       flags,
		Action:      verifierAction,
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}
