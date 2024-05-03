package main

import (
	"context"
	"log"
	"math/big"
	"os"
	"time"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/elcontracts"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/wallet"
	"github.com/Layr-Labs/eigensdk-go/chainio/txmgr"
	"github.com/Layr-Labs/eigensdk-go/metrics"
	"github.com/Layr-Labs/eigensdk-go/signerv2"
	"github.com/Layr-Labs/eigensdk-go/types"
	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/urfave/cli/v2"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	operator "github.com/yetanotherco/aligned_layer/operator/pkg"
)

var (
	AmountFlag = &cli.IntFlag{
		Name:     "amount",
		Usage:    "Amount to deposit",
		Value:    100,
		Required: true,
	}
	StrategyAddressFlag = &cli.StringFlag{
		Name:     "strategy-address",
		Usage:    "Address of the strategy contract",
		Required: true,
		EnvVars:  []string{"STRATEGY_ADDRESS"},
	}
)

var registerFlags = []cli.Flag{
	config.ConfigFileFlag,
}

var depositFlags = []cli.Flag{
	AmountFlag,
	StrategyAddressFlag,
	config.ConfigFileFlag,
}

var startFlags = []cli.Flag{
	config.ConfigFileFlag,
}

func main() {
	app := &cli.App{
		Name: "Aligned Layer Node Operator",
		Commands: []*cli.Command{
			{
				Name:        "register",
				Usage:       "Send a single task to the verifier",
				Description: "Service that sends proofs to verify by operator nodes.",
				Flags:       registerFlags,
				Action:      registerOperatorMain,
			},
			{
				Name:        "start",
				Description: "Service that sends proofs to verify by operator nodes.",
				Flags:       startFlags,
				Action:      operatorMain,
			},
			{
				Name:        "deposit-into-strategy",
				Description: "Service that sends proofs to verify by operator nodes.",
				Flags:       depositFlags,
				Action:      depositIntoStrategyMain,
			},
		},
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln("Operator failed.", "Message:", err)
	}
}

func operatorMain(ctx *cli.Context) error {
	operatorConfigFilePath := ctx.String("config")
	operatorConfig := config.NewOperatorConfig(operatorConfigFilePath)
	err := sdkutils.ReadYamlConfig(operatorConfigFilePath, &operatorConfig)
	if err != nil {
		return err
	}

	operator, err := operator.NewOperatorFromConfig(*operatorConfig)
	if err != nil {
		return err
	}

	log.Println("Operator starting...")
	err = operator.Start(context.Background())
	if err != nil {
		return err
	}

	log.Println("Operator started")

	return nil
}

func registerOperatorMain(ctx *cli.Context) error {
	config := config.NewOperatorConfig(ctx.String(config.ConfigFileFlag.Name))

	quorumNumbers := []byte{0}

	// Generate salt and expiry
	privateKeyBytes := []byte(config.BlsConfig.KeyPair.PrivKey.String())
	salt := [32]byte{}

	copy(salt[:], crypto.Keccak256([]byte("churn"), []byte(time.Now().String()), quorumNumbers, privateKeyBytes))

	expiry := big.NewInt(time.Now().Add(10 * time.Minute).Unix())
	quorumNumbersArr := types.QuorumNums{0}
	socket := "Not Needed"

	err := registerOperator(context.Background(), config,
		socket, quorumNumbersArr, salt, expiry)
	if err != nil {
		config.BaseConfig.Logger.Error("Failed to register operator", "err", err)
		return err
	}

	return nil
}

// RegisterOperator operator registers the operator with the given public key for the given quorum IDs.
// RegisterOperator registers a new operator with the given public key and socket with the provided quorum ids.
// If the operator is already registered with a given quorum id, the transaction will fail (noop) and an error
// will be returned.
func registerOperator(
	ctx context.Context,
	configuration *config.OperatorConfig,
	socket string,
	quorumNumbers types.QuorumNums,
	operatorToAvsRegistrationSigSalt [32]byte,
	operatorToAvsRegistrationSigExpiry *big.Int,
) error {
	writer, err := chainio.NewAvsWriterFromConfig(configuration.BaseConfig, configuration.EcdsaConfig)
	if err != nil {
		configuration.BaseConfig.Logger.Error("Failed to create AVS writer", "err", err)
		return err
	}

	_, err = writer.RegisterOperatorInQuorumWithAVSRegistryCoordinator(ctx, configuration.EcdsaConfig.PrivateKey,
		operatorToAvsRegistrationSigSalt, operatorToAvsRegistrationSigExpiry, configuration.BlsConfig.KeyPair,
		quorumNumbers, socket)

	if err != nil {
		configuration.BaseConfig.Logger.Error("Failed to register operator", "err", err)
		return err
	}

	return nil
}

func depositIntoStrategyMain(ctx *cli.Context) error {
	amount := big.NewInt(int64(ctx.Int(AmountFlag.Name)))
	if amount.Cmp(big.NewInt(0)) <= 0 {
		log.Println("Amount must be greater than 0")
		return nil
	}

	config := config.NewOperatorConfig(ctx.String(config.ConfigFileFlag.Name))
	strategyAddressStr := ctx.String(StrategyAddressFlag.Name)
	if strategyAddressStr == "" {
		log.Println("Strategy address is required")
		return nil
	}
	log.Println("Depositing into strategy", strategyAddressStr)
	strategyAddr := common.HexToAddress(strategyAddressStr)

	delegationManagerAddr := config.BaseConfig.EigenLayerDeploymentConfig.DelegationManagerAddr
	avsDirectoryAddr := config.BaseConfig.EigenLayerDeploymentConfig.AVSDirectoryAddr

	signerConfig := signerv2.Config{
		PrivateKey: config.EcdsaConfig.PrivateKey,
	}
	signerFn, _, err := signerv2.SignerFromConfig(signerConfig, config.BaseConfig.ChainId)
	if err != nil {
		return err
	}
	w, err := wallet.NewPrivateKeyWallet(config.BaseConfig.EthRpcClient, signerFn,
		config.Operator.Address, config.BaseConfig.Logger)

	if err != nil {
		return err
	}

	txMgr := txmgr.NewSimpleTxManager(w, config.BaseConfig.EthRpcClient, config.BaseConfig.Logger,
		config.Operator.Address)
	eigenMetrics := metrics.NewNoopMetrics()
	eigenLayerWriter, err := elcontracts.BuildELChainWriter(delegationManagerAddr, avsDirectoryAddr,
		config.BaseConfig.EthRpcClient, config.BaseConfig.Logger, eigenMetrics, txMgr)
	if err != nil {
		return err
	}

	_, err = eigenLayerWriter.DepositERC20IntoStrategy(context.Background(), strategyAddr, amount)
	if err != nil {
		config.BaseConfig.Logger.Errorf("Error depositing into strategy")
		return err
	}
	return nil
}
