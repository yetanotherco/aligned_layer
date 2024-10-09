package actions

import (
	"context"
	"log"
	"math/big"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/elcontracts"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/wallet"
	"github.com/Layr-Labs/eigensdk-go/chainio/txmgr"
	"github.com/Layr-Labs/eigensdk-go/metrics"
	"github.com/Layr-Labs/eigensdk-go/signerv2"
	"github.com/ethereum/go-ethereum/common"
	"github.com/urfave/cli/v2"
	"github.com/yetanotherco/aligned_layer/core/config"
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

var DepositIntoStrategyCommand = &cli.Command{
	Name:        "deposit-into-strategy",
	Description: "CLI command to deposit into a given strategy",
	Flags:       depositFlags,
	Action:      depositIntoStrategyMain,
}

var depositFlags = []cli.Flag{
	AmountFlag,
	StrategyAddressFlag,
	config.ConfigFileFlag,
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
	w, err := wallet.NewPrivateKeyWallet(&config.BaseConfig.EthRpcClient, signerFn,
		config.Operator.Address, config.BaseConfig.Logger)

	if err != nil {
		return err
	}

	txMgr := txmgr.NewSimpleTxManager(w, &config.BaseConfig.EthRpcClient, config.BaseConfig.Logger,
		config.Operator.Address)
	eigenMetrics := metrics.NewNoopMetrics()
	eigenLayerWriter, err := elcontracts.BuildELChainWriter(delegationManagerAddr, avsDirectoryAddr,
		&config.BaseConfig.EthRpcClient, config.BaseConfig.Logger, eigenMetrics, txMgr)
	if err != nil {
		return err
	}

	_, err = eigenLayerWriter.DepositERC20IntoStrategy(context.Background(), strategyAddr, amount, true)
	if err != nil {
		config.BaseConfig.Logger.Errorf("Error depositing into strategy")
		return err
	}
	return nil
}
