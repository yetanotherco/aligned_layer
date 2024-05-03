package main

import (
	"fmt"
	"log"
	"math/big"
	"os"
	"strings"
	"time"

	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/urfave/cli/v2"
	"github.com/yetanotherco/aligned_layer/common"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/task_sender/pkg"
)

var (
	provingSystemFlag = &cli.StringFlag{
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
	intervalFlag = &cli.IntFlag{
		Name:    "interval",
		Aliases: []string{"t"},
		Value:   1,
		Usage:   "the `INTERVAL` in seconds to send tasks",
	}
	feeFlag = &cli.IntFlag{
		Name:     "fee",
		Required: false,
		Value:    1,
		Usage:    "the `FEE` in wei to send when sending a task",
	}
	quorumThresholdFlag = &cli.UintFlag{
		Name:    "quorum-threshold",
		Aliases: []string{"q"},
		Value:   100,
		Usage:   "the `QUORUM THRESHOLD PERCENTAGE` for tasks",
	}
)

var sendTaskFlags = []cli.Flag{
	provingSystemFlag,
	proofFlag,
	publicInputFlag,
	verificationKeyFlag,
	config.ConfigFileFlag,
	feeFlag,
	quorumThresholdFlag,
}

var loopTasksFlags = []cli.Flag{
	provingSystemFlag,
	proofFlag,
	publicInputFlag,
	verificationKeyFlag,
	config.ConfigFileFlag,
	intervalFlag,
	feeFlag,
	quorumThresholdFlag,
}

func main() {
	app := &cli.App{
		Name: "Aligned Layer Task Sender",
		Commands: []*cli.Command{
			{
				Name:        "send-task",
				Usage:       "Send a single task to the verifier",
				Description: "Service that sends proofs to verify by operator nodes.",
				Flags:       sendTaskFlags,
				Action:      taskSenderMain,
			},
			{
				Name:        "loop-tasks",
				Usage:       "Send a task every `INTERVAL` seconds",
				Description: "Service that sends proofs to verify by operator nodes.",
				Flags:       loopTasksFlags,
				Action:      taskSenderLoopMain,
			},
		},
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln("Task sender application failed.", "Message:", err)
	}
}

func taskSenderMain(c *cli.Context) error {
	provingSystem, err := parseProvingSystem(c.String(provingSystemFlag.Name))
	if err != nil {
		return fmt.Errorf("error getting verification system: %v", err)
	}

	proofFile, err := os.ReadFile(c.String(proofFlag.Name))
	if err != nil {
		return fmt.Errorf("error loading proof file: %v", err)
	}

	publicInputFile, err := os.ReadFile(c.String(publicInputFlag.Name))
	if err != nil {
		return fmt.Errorf("error loading public input file: %v", err)
	}

	var verificationKeyFile []byte
	if provingSystem == common.GnarkPlonkBls12_381 || provingSystem == common.GnarkPlonkBn254 || provingSystem == common.Groth16Bn254 {
		if len(c.String("verification-key")) == 0 {
			return fmt.Errorf("the proving system needs a verification key but it is empty")
		}
		verificationKeyFile, err = os.ReadFile(c.String(verificationKeyFlag.Name))
		if err != nil {
			return fmt.Errorf("error loading verification key file: %v", err)
		}
	}

	fee := big.NewInt(int64(c.Int(feeFlag.Name)))

	taskSenderConfig := config.NewTaskSenderConfig(c.String(config.ConfigFileFlag.Name))
	avsWriter, err := chainio.NewAvsWriterFromConfig(taskSenderConfig.BaseConfig, taskSenderConfig.EcdsaConfig)
	if err != nil {
		return err
	}

	taskSender := pkg.NewTaskSender(avsWriter)
	quorumThresholdPercentage := c.Uint(quorumThresholdFlag.Name)

	// Hardcoded value for `quorumNumbers` - should we get this information from another source? Maybe configuration or CLI parameters?
	quorumNumbers := eigentypes.QuorumNums{0}
	quorumThresholdPercentages := []eigentypes.QuorumThresholdPercentage{eigentypes.QuorumThresholdPercentage(quorumThresholdPercentage)}
	task := pkg.NewTask(provingSystem, proofFile, publicInputFile, verificationKeyFile, quorumNumbers, quorumThresholdPercentages, fee)

	err = taskSender.SendTask(task)
	if err != nil {
		return err
	}

	return nil
}

func taskSenderLoopMain(c *cli.Context) error {
	interval := c.Int(intervalFlag.Name)

	if interval < 1 {
		return fmt.Errorf("interval must be greater than 0")
	}

	for {
		err := taskSenderMain(c)
		if err != nil {
			return err
		}
		time.Sleep(time.Duration(interval) * time.Second)
	}
}

func parseProvingSystem(provingSystemStr string) (common.ProvingSystemId, error) {
	provingSystemStr = strings.TrimSpace(provingSystemStr)
	switch provingSystemStr {
	case "plonk_bls12_381":
		return common.GnarkPlonkBls12_381, nil
	case "plonk_bn254":
		return common.GnarkPlonkBn254, nil
	case "groth16_bn254":
		return common.Groth16Bn254, nil
	default:
		var unknownValue common.ProvingSystemId
		return unknownValue, fmt.Errorf("unsupported proving system: %s", provingSystemStr)
	}
}
