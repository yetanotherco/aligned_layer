package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"

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
	daFlag = &cli.StringFlag{
		Name: "da",
		// Can be either "eigen" or "celestia"
		Usage: "the `DA` to use (calldata | eigen | celestia)",
		Value: "calldata",
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
	daFlag,
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
	daFlag,
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

	taskSenderConfig := config.NewTaskSenderConfig(c.String(config.ConfigFileFlag.Name))
	avsWriter, err := chainio.NewAvsWriterFromConfig(taskSenderConfig.BaseConfig, taskSenderConfig.EcdsaConfig)
	if err != nil {
		return err
	}

	taskSender := pkg.NewTaskSender(taskSenderConfig, avsWriter)

	// TODO(marian): Remove this hardcoded merkle root
	var batchMerkleRoot [32]byte
	batchMerkleRoot[0] = byte(13)
	batchMerkleRoot[1] = byte(14)

	// TODO(marian): Remove this dummy S3 url
	batchDataPointer := "b4b654a31b43c7b5711206eea7d44f884ece1fe7164b478fa16215be77dc84cb.json"

	task := pkg.NewTask(batchMerkleRoot, batchDataPointer)

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

func ParseProvingSystem(provingSystemStr string) (common.ProvingSystemId, error) {
	provingSystemStr = strings.TrimSpace(provingSystemStr)
	switch provingSystemStr {
	case "plonk_bls12_381":
		return common.GnarkPlonkBls12_381, nil
	case "plonk_bn254":
		return common.GnarkPlonkBn254, nil
	case "groth16_bn254":
		return common.Groth16Bn254, nil
	case "sp1":
		return common.SP1, nil
	default:
		var unknownValue common.ProvingSystemId
		return unknownValue, fmt.Errorf("unsupported proving system: %s", provingSystemStr)
	}
}
