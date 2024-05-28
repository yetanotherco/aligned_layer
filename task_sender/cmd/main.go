package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"time"

	"github.com/urfave/cli/v2"
	"github.com/yetanotherco/aligned_layer/common"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	operator "github.com/yetanotherco/aligned_layer/operator/pkg"
	"github.com/yetanotherco/aligned_layer/task_sender/pkg"
	groth16generateproof "github.com/yetanotherco/aligned_layer/task_sender/test_examples/gnark_groth16_bn254_infinite_script/pkg"
	plonkgenerateproof "github.com/yetanotherco/aligned_layer/task_sender/test_examples/gnark_plonk_bn254_infinite_script/pkg"

	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/credentials"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/s3"

	godotenv "github.com/joho/godotenv"
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

var infiniteTasksFlags = []cli.Flag{
	provingSystemFlag,
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

	batchMerkleRoot, batchDataPointer, err := getAndUploadProofData(c, x)
	if err != nil {
		return err
	}

	task := pkg.NewTask(batchMerkleRoot, batchDataPointer)

	err = taskSender.SendTask(task)
	if err != nil {
		return err
	}

	return nil
}

func getAndUploadProofData(c *cli.Context, x int) ([32]byte, string, error) {
	var proofFile, pubInputFile, verificationKeyFile string

	provingSystem, err := ParseProvingSystem(c.String(provingSystemFlag.Name))
	if err != nil {
		return [32]byte{}, "", err
	}

	outputDir := "task_sender/test_examples/"
	provingSystemDir := ""

	switch provingSystem {
	case common.GnarkPlonkBls12_381:
		provingSystemDir = "gnark_plonk_bls12_381_infinite_script/infinite_proofs/"
	case common.GnarkPlonkBn254:
		provingSystemDir = "gnark_plonk_bn254_infinite_script/infinite_proofs/"
	case common.Groth16Bn254:
		provingSystemDir = "gnark_groth16_bn254_infinite_script/infinite_proofs/"
	case common.SP1:
		provingSystemDir = "sp1_infinite_script/infinite_proofs/"
	default:
		return [32]byte{}, "", fmt.Errorf("unsupported proving system: %s", fmt.Sprint(provingSystem))
	}

	outputDir += provingSystemDir

	if x == 0 { // previous version, not generated by infinite-generator, read proof from flag parameters
		proofFile = c.String(proofFlag.Name)
		pubInputFile = c.String(publicInputFlag.Name)
		verificationKeyFile = c.String(verificationKeyFlag.Name)
	} else { // new version, generated by infinite-generator, calculate the real values
		switch provingSystem {
		case common.GnarkPlonkBls12_381, common.GnarkPlonkBn254:
			proofFile = outputDir + "ineq_" + strconv.Itoa(x) + "_plonk.proof"
			pubInputFile = outputDir + "ineq_" + strconv.Itoa(x) + "_plonk.pub"
			verificationKeyFile = outputDir + "ineq_" + strconv.Itoa(x) + "_plonk.vk"
		case common.Groth16Bn254:
			proofFile = outputDir + "ineq_" + strconv.Itoa(x) + "_groth16.proof"
			pubInputFile = outputDir + "ineq_" + strconv.Itoa(x) + "_groth16.pub"
			verificationKeyFile = outputDir + "ineq_" + strconv.Itoa(x) + "_groth16.vk"
		}
	}

	log.Printf("Using proof file: %s\n", proofFile)
	log.Printf("Using public input file: %s\n", pubInputFile)
	log.Printf("Using verification key file: %s\n", verificationKeyFile)

	ProofByteArray, err := os.ReadFile(proofFile)
	if err != nil {
		return [32]byte{}, "", fmt.Errorf("failed to read proof file: %v", err)
	}
	PubInputByteArray, err := os.ReadFile(pubInputFile)
	if err != nil {
		return [32]byte{}, "", fmt.Errorf("failed to read public input file: %v", err)
	}
	VerificationKeyByteArray, err := os.ReadFile(verificationKeyFile)
	if err != nil {
		return [32]byte{}, "", fmt.Errorf("failed to read verification key file: %v", err)
	}

	var data []operator.VerificationData
	if provingSystem == common.SP1 { // currently, this is the correct way of handling VerificationKey/VmProgramCode
		data = []operator.VerificationData{{
			ProvingSystemId: provingSystem,
			Proof:           ProofByteArray,
			PubInput:        PubInputByteArray,
			VerificationKey: []byte(""),
			VmProgramCode:   VerificationKeyByteArray,
		}}
	} else {
		data = []operator.VerificationData{{
			ProvingSystemId: provingSystem,
			Proof:           ProofByteArray,
			PubInput:        PubInputByteArray,
			VerificationKey: VerificationKeyByteArray,
			VmProgramCode:   []byte(""),
		}}
	}
	byteArray, err := json.Marshal(data)
	if err != nil {
		return [32]byte{}, "", err
	}
	merkleRoot := sha256.Sum256(byteArray)
	batchDataPointer, err := uploadObjectToS3(byteArray, merkleRoot)
	if err != nil {
		return [32]byte{}, "", err
	}

	return merkleRoot, batchDataPointer, nil
}

func uploadObjectToS3(byteArray []byte, merkleRoot [32]byte) (string, error) {
	// I want to upload the bytearray to my S3 bucket, with merkleRoot as the object name
	err := godotenv.Load("./task_sender/.env")
	if err != nil {
		log.Fatalf("Error loading .env file: %v", err)
	}
	region := os.Getenv("AWS_REGION")
	accessKey := os.Getenv("AWS_ACCESS_KEY")
	secretKey := os.Getenv("AWS_SECRET")
	bucket := os.Getenv("AWS_S3_BUCKET")
	if region == "" || accessKey == "" || secretKey == "" || bucket == "" {
		fmt.Println("Fail.\nPlease set the AWS_REGION, AWS_ACCESS_KEY, AWS_SECRET, and AWS_S3_BUCKET environment variables. \nYou can use task_sender/.env.example as a template.")
		return "", fmt.Errorf("missing AWS environment variables")
	}

	sess, err := session.NewSession(&aws.Config{
		Region:      aws.String(region),
		Credentials: credentials.NewStaticCredentials(accessKey, secretKey, ""),
	})
	if err != nil {
		fmt.Println("Error creating aws session:", err)
		fmt.Println("Did you set the AWS_REGION, AWS_ACCESS_KEY, and AWS_SECRET environment variables?\nYou can yse task_Sender/.env.example as a template.")
		return "", err
	}
	svc := s3.New(sess)

	merkleRootHex := hex.EncodeToString(merkleRoot[:])
	key := merkleRootHex + ".json"

	// This uploads the contents to S3
	_, err = svc.PutObject(&s3.PutObjectInput{
		Bucket: aws.String(bucket),
		Key:    aws.String(key),
		Body:   bytes.NewReader(byteArray),
	})
	if err != nil {
		fmt.Println("Error uploading file:", err)
		return "", err
	}

	fmt.Println("File uploaded successfully!!!")

	batchDataPointer := "https://storage.alignedlayer.com/" + key
	fmt.Println(batchDataPointer)

	return batchDataPointer, nil
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

	provingSystem, err := ParseProvingSystem(c.String(provingSystemFlag.Name))
	if err != nil {
		return err
	}

	for {
		x += 1

		switch provingSystem {
		case common.GnarkPlonkBn254:
			plonkgenerateproof.GenerateIneqProof(x)
		case common.Groth16Bn254:
			groth16generateproof.GenerateIneqProof(x)
		default:
			return fmt.Errorf("unsupported proving system: %s", fmt.Sprint(provingSystem))
		}

		err = taskSenderMain(c, x)
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
