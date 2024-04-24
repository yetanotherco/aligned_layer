package main

import (
	"context"
	"fmt"
	"log"
	"os"

	"github.com/urfave/cli/v2"
	"github.com/yetanotherco/aligned_layer/common"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/tests/mocks"
	"github.com/yetanotherco/aligned_layer/core/types"
)

var (
	systemFlag = &cli.StringFlag{
		Name:     "proving-system",
		Aliases:  []string{"s"},
		Required: true,
		Usage:    "the `PROVING SYSTEM` to use (e.g., plonk, groth16)",
	}
	proofFlag = &cli.PathFlag{
		Name:     "proof",
		Aliases:  []string{"p"},
		Required: true,
		Usage:    "path to the `PROOF FILE`",
	}
	publicInputFlag = &cli.PathFlag{
		Name:     "public-input",
		Aliases:  []string{"i"},
		Required: true,
		Usage:    "path to the `PUBLIC INPUT FILE`",
	}
	verificationKeyFlag = &cli.PathFlag{
		Name:     "verification-key",
		Aliases:  []string{"v"},
		Required: false,
		Usage:    "path to the `VERIFICATION KEY FILE`",
	}
)

var flags = []cli.Flag{
	systemFlag,
	proofFlag,
	publicInputFlag,
	verificationKeyFlag,
}

func main() {
	app := &cli.App{
		Name:        "Aligned Layer Task Sender",
		Usage:       "Send a task to the verifier",
		Description: "Service that sends proofs to verify by operator nodes.",
		Flags:       flags,
		Action:      taskSenderMain,
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln("Task sender application failed.", "Message:", err)
	}
}

func taskSenderMain(c *cli.Context) error {
	verificationSystem, err := GetVerificationSystem(c.String(systemFlag.Name))
	if err != nil {
		return fmt.Errorf("error getting verification system: %v", err)
	}

	proofFile, err := os.ReadFile(c.String(proofFlag.Name))
	if err != nil {
		return fmt.Errorf("error loading proofFile file: %v", err)
	}

	publicInputFile, err := os.ReadFile(c.String(publicInputFlag.Name))
	if err != nil {
		return fmt.Errorf("error loading public input file: %v", err)
	}

	var verificationKeyFile []byte
	if len(c.String("verification-key")) > 0 {
		verificationKeyFile, err = os.ReadFile(c.String(verificationKeyFlag.Name))
		if err != nil {
			return fmt.Errorf("error loading verification key file: %v", err)
		}
	}

	err = SendTask(types.NewTask(verificationSystem, proofFile, publicInputFile, verificationKeyFile))
	if err != nil {
		return err
	}

	return nil
}

func SendTask(task *types.Task) error {
	log.Println("Sending task...")
	avsWriter, err := chainio.NewAvsWriterFromConfig(mocks.NewDevnetConfig())
	if err != nil {
		return err
	}

	_, index, err := avsWriter.SendTask(
		context.Background(),
		task.ProvingSystem,
		task.Proof,
		task.PublicInput,
	)
	if err != nil {
		return err
	}
	log.Println("Task sent successfully. Task index:", index)
	return nil
}

// TODO Set corrects verification systems
func GetVerificationSystem(system string) (common.ProvingSystemId, error) {
	var unknownValue common.ProvingSystemId
	switch system {
	case "plonk":
		return common.GnarkPlonkBls12_381, nil
	default:
		return unknownValue, fmt.Errorf("unsupported proof system: %s", system)
	}
}
