package main

import (
	"aligned_layer/task_sender/pkg"
	"fmt"
	"github.com/urfave/cli/v2"
	"log"
	"os"
)

var (
	systemFlag = &cli.StringFlag{
		Name:     "system",
		Aliases:  []string{"s"},
		Required: true,
		Usage:    "the proof `SYSTEM` to use (e.g., plonk, groth16)",
	}
	proofFlag = &cli.PathFlag{
		Name:     "proof",
		Aliases:  []string{"p"},
		Required: true,
		Usage:    "path to the proof `FILE`",
	}
	publicInputFlag = &cli.PathFlag{
		Name:     "public-input",
		Aliases:  []string{"i"},
		Required: true,
		Usage:    "path to the public input `FILE`",
	}
	verificationKeyFlag = &cli.PathFlag{
		Name:     "verification-key",
		Aliases:  []string{"v"},
		Required: false,
		Usage:    "path to the verification key `FILE`",
	}
)

var flags = []cli.Flag{
	systemFlag,
	proofFlag,
	publicInputFlag,
	verificationKeyFlag,
}

func main() {
	fmt.Println("Task sender")
	app := &cli.App{
		Name:        "Aligned Layer Task Sender",
		Usage:       "Send a task to the verifier",
		Description: "Service that sends proofs to verify by operator nodes.",
		Flags:       flags,
		Action:      pkg.SendTask,
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln("Task sender application failed.", "Message:", err)
	}
}
