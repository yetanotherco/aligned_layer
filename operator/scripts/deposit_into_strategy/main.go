package main

import (
	"context"
	"errors"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/elcontracts"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/wallet"
	"github.com/Layr-Labs/eigensdk-go/chainio/txmgr"
	"github.com/Layr-Labs/eigensdk-go/metrics"
	"github.com/Layr-Labs/eigensdk-go/signerv2"
	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/urfave/cli"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	"log"
	"math/big"
	"os"
	"time"
)

var (
	AmountFlag = cli.IntFlag{
		Name:     "amount",
		Usage:    "Amount to deposit",
		Value:    100,
		Required: true,
	}
	StrategyDeploymentOutputFlag = cli.StringFlag{
		Name:     "strategy-deployment-output",
		Usage:    "Path to strategy deployment output file",
		Required: true,
	}
)

func main() {
	app := cli.NewApp()
	app.Name = "Operator deposit into strategy"
	app.Flags = append(config.Flags, AmountFlag, StrategyDeploymentOutputFlag)
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

	configuration := config.NewOperatorConfig(ctx.String(config.BaseConfigFileFlag.Name),
		ctx.String(config.OperatorConfigFileFlag.Name))

	strategyContracts := newStrategyDeploymentConfig(ctx.String(StrategyDeploymentOutputFlag.Name))

	delegationManagerAddr := configuration.BaseConfig.EigenLayerDeploymentConfig.DelegationManagerAddr
	avsDirectoryAddr := configuration.BaseConfig.EigenLayerDeploymentConfig.AVSDirectoryAddr
	strategyAddr := strategyContracts.StrategyAddr

	eigenLayerReader, err := elcontracts.BuildELChainReader(delegationManagerAddr, avsDirectoryAddr,
		configuration.BaseConfig.EthRpcClient, configuration.BaseConfig.Logger)
	if err != nil {
		configuration.BaseConfig.Logger.Error("Failed to build ELChainReader", "err", err)
		return err
	}

	_, tokenAddr, err := eigenLayerReader.GetStrategyAndUnderlyingToken(&bind.CallOpts{}, strategyAddr)
	if err != nil {
		configuration.BaseConfig.Logger.Error("Failed to fetch strategy contract", "err", err)
		return err
	}

	avsReader, err := chainio.NewAvsReaderFromConfig(configuration.BaseConfig)
	if err != nil {
		return err
	}
	contractErc20Mock, err := avsReader.GetErc20Mock(tokenAddr)
	if err != nil {
		configuration.BaseConfig.Logger.Error("Failed to fetch ERC20Mock contract", "err", err)
		return err
	}

	avsWriter, err := chainio.NewAvsWriterFromConfig(configuration.BaseConfig)
	if err != nil {
		return err
	}
	txOpts := avsWriter.Signer.GetTxOpts()
	_, err = contractErc20Mock.Mint(txOpts, configuration.Operator.Address, amount)
	if err != nil {
		configuration.BaseConfig.Logger.Errorf("Error assembling Mint tx")
		return err
	}

	// TODO: actually wait, need instrumented client
	//configuration.EthHttpClient.WaitForTransactionReceipt(context.Background(), tx.Hash())
	// sleep
	time.Sleep(2 * time.Second)

	signerConfig := signerv2.Config{
		PrivateKey: configuration.BaseConfig.EcdsaPrivateKey,
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

type StrategyDeploymentConfig struct {
	ERC20Mock    common.Address `json:"erc20Mock"`
	StrategyAddr common.Address `json:"erc20MockStrategy"`
}

func newStrategyDeploymentConfig(strategyDeploymentFilePath string) *StrategyDeploymentConfig {
	if _, err := os.Stat(strategyDeploymentFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup eigen layer deployment file does not exist")
	}

	var strategyDeploymentConfig StrategyDeploymentConfig
	err := sdkutils.ReadJsonConfig(strategyDeploymentFilePath, &strategyDeploymentConfig)

	if err != nil {
		log.Fatal("Error reading eigen layer deployment config: ", err)
	}

	if strategyDeploymentConfig.ERC20Mock == common.HexToAddress("0x0") {
		log.Fatal("ERC20Mock address not found in strategy deployment config")
	}

	if strategyDeploymentConfig.StrategyAddr == common.HexToAddress("0x0") {
		log.Fatal("Strategy address not found in strategy deployment config")
	}

	return &strategyDeploymentConfig

}
