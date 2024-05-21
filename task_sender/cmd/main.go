package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"
	"strconv"
	"encoding/json"
	"crypto/sha256"

	"github.com/urfave/cli/v2"
	"github.com/yetanotherco/aligned_layer/common"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/task_sender/pkg"
	generateproof "github.com/yetanotherco/aligned_layer/task_sender/test_examples/gnark_groth16_bn254_infinite_script/pkg"
	operator "github.com/yetanotherco/aligned_layer/operator/pkg"
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

var infiniteTasksFlags = []cli.Flag{
	provingSystemFlag,
	// proofFlag, //this doesn't go since it must generate a new one for each send //TODO fix since it is 'required'
	// publicInputFlag, //this doesn't go since it must generate a new one for each send //TODO fix since it is 'required'
	// verificationKeyFlag, //tied to the circuit, in this case : 'x!=0 ?' //TODO make it read only once, i think it is generating one for each new cycle
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
				Action: func(c *cli.Context) error {
					return taskSenderMain(c)
				},
			},
			{
				Name:        "loop-tasks",
				Usage:       "Send a task every `INTERVAL` seconds",
				Description: "Service that sends proofs to verify by operator nodes.",
				Flags:       loopTasksFlags,
				Action:      taskSenderLoopMain,
			},
			{
				Name:        "infinite-tasks",
				Usage:       "Send a different task every `INTERVAL` seconds",
				Description: "Service that sends proofs to verify by operator nodes.",
				Flags:       infiniteTasksFlags,
				Action:      taskSenderInfiniteMain,
			},
		},
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln("Task sender application failed.", "Message:", err)
	}
}

func taskSenderMain(c *cli.Context, xParam ...int) error {
	x := 0
	if len(xParam) > 0 {
		x = xParam[0]
	}

	taskSenderConfig := config.NewTaskSenderConfig(c.String(config.ConfigFileFlag.Name))
	avsWriter, err := chainio.NewAvsWriterFromConfig(taskSenderConfig.BaseConfig, taskSenderConfig.EcdsaConfig)
	if err != nil {
		return err
	}

	taskSender := pkg.NewTaskSender(taskSenderConfig, avsWriter)

	var batchMerkleRoot [32]byte
	var batchDataPointer string

	if x == 0 { //use hardcoded value
		// TODO(marian): Remove this hardcoded merkle root
		batchMerkleRoot[0] = byte(123)
		batchMerkleRoot[1] = byte(123)
		// TODO(marian): Remove this dummy S3 url
		batchDataPointer = "https://storage.alignedlayer.com/b4b654a31b43c7b5711206eea7d44f884ece1fe7164b478fa16215be77dc84cb.json"
	} else { //we can calculate the real value
		outputDir := "task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/"
		ProofByteArray, err := os.ReadFile(outputDir + "ineq_" + strconv.Itoa(x) + "_groth16.proof") //TODO un-hardcode provingSystem
		if err != nil {
			return err
		}
		PubInputByteArray, err := os.ReadFile(outputDir + "ineq_" + strconv.Itoa(x) + "_groth16.pub")
		if err != nil {
			return err
		}
		VerificationKeyByteArray, err := os.ReadFile(outputDir + "ineq_" + strconv.Itoa(x) + "_groth16.vk")
		if err != nil {
			return err
		}
		provingSystem, err := ParseProvingSystem(provingSystemFlag.Value)
		if err != nil {
			return err
		}
		data := operator.VerificationData{
			ProvingSystemId: provingSystem,
			Proof:           ProofByteArray,
			PubInput:        PubInputByteArray,
			VerificationKey: VerificationKeyByteArray,
			// VmProgramCode:   []byte                 `json:"vm_program_code"` //TODO add in correct format
			VmProgramCode:  []byte(""),
		}


		byteArray, err := json.Marshal(data)
		if err != nil {
			return err
		}
		batchMerkleRoot = sha256.Sum256(byteArray)
		// batchDataPointer = outputDir + "ineq_" + strconv.Itoa(x) + "_groth16" //local files
		batchDataPointer = "https://storage.alignedlayer.com/b4b654a31b43c7b5711206eea7d44f884ece1fe7164b478fa16215be77dc84cb.json"

	}


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

func taskSenderInfiniteMain(c *cli.Context) error {
	interval := c.Int(intervalFlag.Name)

	if interval < 1 {
		return fmt.Errorf("interval must be greater than 0")
	}

	x := 0
	for {
		x += 1
		generateproof.GenerateIneqProof(x)
		err := taskSenderMain(c, x)
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
