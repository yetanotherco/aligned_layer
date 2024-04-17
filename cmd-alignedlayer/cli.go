package main

import (
	"fmt"
	"log"
	"os"

	"github.com/urfave/cli/v2"
)

func runCLI() {
	app := &cli.App{
		Name:  "verify",
		Usage: "verifies using different proof systems",
		Commands: []*cli.Command{
			{
				Name:    "verify",
				Aliases: []string{"v"},
				Usage:   "verify a ZK proof",
				Flags: []cli.Flag{
					&cli.StringFlag{
						Name:     "system",
						Usage:    "Proofing system to use (e.g., gnark-plonk)",
						Required: true,
					},
					&cli.StringFlag{
						Name:     "proof",
						Usage:    "Path to the proof file",
						Required: true,
					},
					&cli.StringFlag{
						Name:     "public-input",
						Aliases:  []string{"pinput"},
						Usage:    "Path to the public input file",
						Required: true,
					},
					&cli.StringFlag{
						Name:     "verification-key",
						Aliases:  []string{"vkey"},
						Usage:    "Path to the verification key file",
						Required: true,
					},
				},
				Action: func(c *cli.Context) error {
					if c.String("system") != "gnark-plonk" {
						fmt.Println("Unsupported proof system. Currently only 'gnark-plonk' is supported.")
						return nil
					}
					fmt.Println("Verifying using gnark-plonk...")
					proof, err := readProofFile(c.String("proof"))
					if err != nil {
						return err
					}
					publicInput, err := readPublicInputFile(c.String("public-input"))
					if err != nil {
						return err
					}
					verificationKey, err := readVerificationKeyFile(c.String("verification-key"))
					if err != nil {
						return err
					}
					result, err := verifyProof(proof, publicInput, verificationKey)
					if err != nil {
						return err
					}
					if result {
						fmt.Println("Proof verified successfully!")
					} else {
						fmt.Println("Proof verification failed.")
					}
					return nil
				},
			},
		},
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}
func readProofFile(filePath string) ([]byte, error) {
	return nil, nil
}

func readPublicInputFile(filePath string) ([]byte, error) {
	return nil, nil
}

func readVerificationKeyFile(filePath string) ([]byte, error) {
	return nil, nil
}

func verifyProof(proof []byte, publicInput []byte, verificationKey []byte) (bool, error) {
	return true, nil
}
