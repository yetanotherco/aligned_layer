package main

import (
	"fmt"
	"log"
	"os"

	"github.com/urfave/cli/v2"
)

func runCLI() {
	app := &cli.App{
		Name:      "Aligned Layer Verifier",
		UsageText: "\n" + "verify plonk proof.txt public_input_file.txt " + "verification_key.txt"
		+ "\n"  + "verify groth16 proof.txt public_input_file.txt " + "verification_key.txt"

		Action: func(c *cli.Context) error {
			if c.NArg() < 5 || c.Args().Get(0) != "verify" {
				return cli.ShowAppHelp(c)
			}

			system := c.Args().Get(1)
			proof := c.Args().Get(2)
			publicInput := c.Args().Get(3)
			verificationKey := c.Args().Get(4)

			switch system {
			case "groth16":
				err := verify("groth16", proof, publicInput, verificationKey)
				if err != nil {
					return err
				}
			case "plonk":
				err := verify("plonk", proof, publicInput, verificationKey)
				if err != nil {
					return err
				}
			default:
				fmt.Println("Unsupported proof system:", system)
				return nil
			}

			return nil
		},
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}

func verify(system, proof, publicInput, verificationKey string) error {
	fmt.Printf("Verifying using %s...\n", system)
	fmt.Println("Proof:", proof)
	fmt.Println("Public Input:", publicInput)
	fmt.Println("Verification Key:", verificationKey)
	return nil
}
