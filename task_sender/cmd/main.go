package main

import (
	"context"
	"errors"
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/signer"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/urfave/cli"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	gethCommon "github.com/ethereum/go-ethereum/common"
	"github.com/yetanotherco/aligned_layer/common"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/task_generator"
)

var (
	// Version is the version of the binary.
	Version   string
	GitCommit string
	GitDate   string
)

var (
	ProofFileFlag = cli.StringFlag{
		Name:     "proof",
		Required: true,
		Usage:    "Load proof from `PROOF_FILE`",
	}

	VerifierIdFlag = cli.StringFlag{
		Name:     "verifier-id",
		Required: true,
		Usage:    "Set verifier ID",
	}

	PubInputIdFlag = cli.StringFlag{
		Name:     "pub-input",
		Required: false,
		Usage:    "Load public inputs from `PUB_INPUT_FILE`",
	}
)

var flags = []cli.Flag{
	ProofFileFlag,
	VerifierIdFlag,
	PubInputIdFlag,
}

func main() {
	app := cli.NewApp()
	app.Flags = flags
	app.Version = fmt.Sprintf("%s-%s-%s", Version, GitCommit, GitDate)
	app.Name = "Aligned Layer Task Sender"
	app.Usage = "Aligned Layer Task Sender"
	app.Description = "Service that sends proofs to verify by operator nodes."

	app.Action = taskSenderMain
	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln("Task sender application failed.", "Message:", err)
	}
}

func taskSenderMain(ctx *cli.Context) error {
	log.Println("Initializing Task Sender...")
	config, err := dummyConfig()
	if err != nil {
		return err
	}

	taskGen, err := task_generator.NewTaskGenerator(config)
	if err != nil {
		return err
	}

	proofFilePath := ctx.GlobalString(ProofFileFlag.Name)
	proof, err := os.ReadFile(proofFilePath)
	if err != nil {
		panic("Could not read proof file")
	}

	verifierId, err := parseVerifierId(ctx.GlobalString((VerifierIdFlag.Name)))
	if err != nil {
		return err
	}

	var pubInput []byte
	// When we have a PLONK or Kimchi proof, we should check for the public inputs.
	// Cairo proofs have public inputs embedded, so no need to check for this CLI input for the moment.
	// This should be done for every proving system.
	if verifierId == common.GnarkPlonkBls12_381 || verifierId == common.Kimchi {
		pubInputFilePath := ctx.GlobalString(PubInputIdFlag.Name)
		pubInput, err = os.ReadFile(pubInputFilePath)
		if err != nil {
			panic("Could not public input file")
		}
	}

	err = taskGen.SendNewTask(proof, pubInput, verifierId)
	if err != nil {
		return err
	}

	log.Println("Task successfully sent\n")

	return nil
}

func parseVerifierId(verifierIdStr string) (common.VerifierId, error) {
	// standard whitespace trimming
	verifierIdStr = strings.TrimSpace(verifierIdStr)
	switch verifierIdStr {
	case "cairo":
		return common.LambdaworksCairo, nil
	case "plonk":
		return common.GnarkPlonkBls12_381, nil
	case "kimchi":
		return common.Kimchi, nil
	case "sp1":
		return common.Sp1BabyBearBlake3, nil
	default:
		// returning this just to return something, the error should be handled
		// by the caller.
		return common.LambdaworksCairo, errors.New("could not parse verifier ID")
	}
}

// This function is almost identical to NewConfig, but with hardcoded values.
// Its only purpose is to make the usage of the task sender CLI easier, since we don't really
// need to setup special configurations for the moment.
func dummyConfig() (*config.Config, error) {
	var configRaw config.ConfigRaw
	configFilePath := "config-files/aggregator.yaml"
	sdkutils.ReadYamlConfig(configFilePath, &configRaw)

	var alignedLayerDeploymentRaw config.AlignedLayerDeploymentRaw
	alignedLayerDeploymentFilePath := "contracts/script/output/31337/aligned_layer_avs_deployment_output.json"
	if _, err := os.Stat(alignedLayerDeploymentFilePath); errors.Is(err, os.ErrNotExist) {
		panic("Path " + alignedLayerDeploymentFilePath + " does not exist")
	}
	sdkutils.ReadJsonConfig(alignedLayerDeploymentFilePath, &alignedLayerDeploymentRaw)

	var sharedAvsContractsDeploymentRaw config.SharedAvsContractsRaw
	sharedAvsContractsDeploymentFilePath := "contracts/script/output/31337/shared_avs_contracts_deployment_output.json"
	if _, err := os.Stat(sharedAvsContractsDeploymentFilePath); errors.Is(err, os.ErrNotExist) {
		panic("Path " + sharedAvsContractsDeploymentFilePath + " does not exist")
	}
	sdkutils.ReadJsonConfig(sharedAvsContractsDeploymentFilePath, &sharedAvsContractsDeploymentRaw)

	logger, err := sdklogging.NewZapLogger(configRaw.Environment)
	if err != nil {
		return nil, err
	}

	ethRpcClient, err := eth.NewClient(configRaw.EthRpcUrl)
	if err != nil {
		logger.Errorf("Cannot create http ethclient", "err", err)
		return nil, err
	}

	ethWsClient, err := eth.NewClient(configRaw.EthWsUrl)
	if err != nil {
		logger.Errorf("Cannot create ws ethclient", "err", err)
		return nil, err
	}

	ecdsaPrivateKeyString := "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6"
	if ecdsaPrivateKeyString[:2] == "0x" {
		ecdsaPrivateKeyString = ecdsaPrivateKeyString[2:]
	}
	ecdsaPrivateKey, err := crypto.HexToECDSA(ecdsaPrivateKeyString)
	if err != nil {
		logger.Errorf("Cannot parse ecdsa private key", "err", err)
		return nil, err
	}

	operatorAddr, err := sdkutils.EcdsaPrivateKeyToAddress(ecdsaPrivateKey)
	if err != nil {
		logger.Error("Cannot get operator address", "err", err)
		return nil, err
	}

	chainId, err := ethRpcClient.ChainID(context.Background())
	if err != nil {
		logger.Error("Cannot get chainId", "err", err)
		return nil, err
	}

	privateKeySigner, err := signer.NewPrivateKeySigner(ecdsaPrivateKey, chainId)
	if err != nil {
		logger.Error("Cannot create signer", "err", err)
		return nil, err
	}

	config := &config.Config{
		EcdsaPrivateKey:                ecdsaPrivateKey,
		Logger:                         logger,
		EthRpcUrl:                      configRaw.EthRpcUrl,
		EthHttpClient:                  ethRpcClient,
		EthWsClient:                    ethWsClient,
		BlsOperatorStateRetrieverAddr:  gethCommon.HexToAddress(sharedAvsContractsDeploymentRaw.BlsOperatorStateRetrieverAddr),
		AlignedLayerServiceManagerAddr: gethCommon.HexToAddress(alignedLayerDeploymentRaw.Addresses.AlignedLayerServiceManagerAddr),
		SlasherAddr:                    gethCommon.HexToAddress(""),
		AggregatorServerIpPortAddr:     configRaw.AggregatorServerIpPortAddr,
		RegisterOperatorOnStartup:      configRaw.RegisterOperatorOnStartup,
		Signer:                         privateKeySigner,
		OperatorAddress:                operatorAddr,
		BlsPublicKeyCompendiumAddress:  gethCommon.HexToAddress(configRaw.BLSPubkeyCompendiumAddr),
		AVSServiceManagerAddress:       gethCommon.HexToAddress(configRaw.AvsServiceManagerAddress),
		EnableMetrics:                  configRaw.EnableMetrics,
	}
	config.Validate()
	return config, nil
}
