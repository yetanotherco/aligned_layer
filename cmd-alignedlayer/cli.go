package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"

	"github.com/urfave/cli/v2"
)

func runCLI() {
	app := &cli.App{
		Name:      "Aligned Layer Verifier",
		UsageText: "\n" + "verify plonk proof.txt public_input_file.txt " + "verification_key.txt" + "\n" + "verify groth16 proof.txt public_input_file.txt " + "verification_key.txt",
		Commands: []*cli.Command{
			{
				Name:  "verify",
				Usage: "verify a proof using a specified proof system",
				Action: func(c *cli.Context) error {
					if c.NArg() < 4 {
						fmt.Println("Insufficient arguments")
						cli.ShowCommandHelp(c, "verify")
						return nil
					}

					system := c.Args().Get(0)
					proofPath := c.Args().Get(1)
					publicInputPath := c.Args().Get(2)
					verificationKeyPath := c.Args().Get(3)

					return verify(system, proofPath, publicInputPath, verificationKeyPath)
				},
			},
		},
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
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
