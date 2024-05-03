package main

import (
	"context"
	"log"
	"math/big"
	"os"

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

var flags = []cli.Flag{
	AmountFlag,
	StrategyAddressFlag,
	config.ConfigFileFlag,
}

func main() {
	app := cli.NewApp()
	app.Name = "Operator deposit into strategy"
	app.Flags = flags
	app.Action = depositIntoStrategy

	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
		return
	}
}

func depositIntoStrategy(ctx *cli.Context) error {
	amount := big.NewInt(int64(ctx.Int(AmountFlag.Name)))
	if amount.Cmp(big.NewInt(0)) <= 0 {
		log.Println("Amount must be greater than 0")
		return nil
	}

	configuration := config.NewOperatorConfig(ctx.String(config.ConfigFileFlag.Name))
	strategyAddressStr := ctx.String(StrategyAddressFlag.Name)
	if strategyAddressStr == "" {
		log.Println("Strategy address is required")
		return nil
	}
	log.Println("Depositing into strategy", strategyAddressStr)
	strategyAddr := common.HexToAddress(strategyAddressStr)

	delegationManagerAddr := configuration.BaseConfig.EigenLayerDeploymentConfig.DelegationManagerAddr
	avsDirectoryAddr := configuration.BaseConfig.EigenLayerDeploymentConfig.AVSDirectoryAddr

	signerConfig := signerv2.Config{
		PrivateKey: configuration.EcdsaConfig.PrivateKey,
	}
	signerFn, _, err := signerv2.SignerFromConfig(signerConfig, configuration.BaseConfig.ChainId)
	if err != nil {
		return err
	}
	w, err := wallet.NewPrivateKeyWallet(configuration.BaseConfig.EthRpcClient, signerFn,
		configuration.Operator.Address, configuration.BaseConfig.Logger)

	if err != nil {
		return err
	}

	txMgr := txmgr.NewSimpleTxManager(w, configuration.BaseConfig.EthRpcClient, configuration.BaseConfig.Logger,
		configuration.Operator.Address)
	eigenMetrics := metrics.NewNoopMetrics()
	eigenLayerWriter, err := elcontracts.BuildELChainWriter(delegationManagerAddr, avsDirectoryAddr,
		configuration.BaseConfig.EthRpcClient, configuration.BaseConfig.Logger, eigenMetrics, txMgr)
	if err != nil {
		return err
	}

	_, err = eigenLayerWriter.DepositERC20IntoStrategy(context.Background(), strategyAddr, amount)
	if err != nil {
		configuration.BaseConfig.Logger.Errorf("Error depositing into strategy")
		return err
	}
	return nil
}
