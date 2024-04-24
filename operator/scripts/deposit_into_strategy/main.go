package main

import (
	"context"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/elcontracts"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/wallet"
	"github.com/Layr-Labs/eigensdk-go/chainio/txmgr"
	"github.com/Layr-Labs/eigensdk-go/metrics"
	"github.com/Layr-Labs/eigensdk-go/signerv2"
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
)

func main() {
	app := cli.NewApp()
	app.Name = "Operator deposit into strategy"
	app.Flags = append(config.Flags, AmountFlag)
	app.Action = depositIntoStrategy

	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
		return
	}
}

func depositIntoStrategy(ctx *cli.Context) error {
	configuration, err := config.NewConfig(ctx)
	if err != nil {
		return err
	}

	// TODO: read values from output
	delegationManagerAddr := common.HexToAddress("0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9")
	avsDirectoryAddr := common.HexToAddress("0x5FC8d32690cc91D4c39d9d3abcBD16989F875707")
	strategyAddr := common.HexToAddress("0x09635F643e140090A9A8Dcd712eD6285858ceBef")
	amount := big.NewInt(int64(ctx.Int("amount")))

	eigenLayerReader, err := elcontracts.BuildELChainReader(delegationManagerAddr, avsDirectoryAddr,
		configuration.EthHttpClient, configuration.Logger)
	if err != nil {
		return err
	}

	_, tokenAddr, err := eigenLayerReader.GetStrategyAndUnderlyingToken(&bind.CallOpts{}, strategyAddr)
	if err != nil {
		configuration.Logger.Error("Failed to fetch strategy contract", "err", err)
		return err
	}

	avsReader, err := chainio.NewAvsReaderFromConfig(configuration)
	if err != nil {
		return err
	}
	contractErc20Mock, err := avsReader.GetErc20Mock(tokenAddr)
	if err != nil {
		configuration.Logger.Error("Failed to fetch ERC20Mock contract", "err", err)
		return err
	}

	avsWriter, err := chainio.NewAvsWriterFromConfig(configuration)
	if err != nil {
		return err
	}
	txOpts := avsWriter.Signer.GetTxOpts()
	_, err = contractErc20Mock.Mint(txOpts, configuration.OperatorAddress, amount)
	if err != nil {
		configuration.Logger.Errorf("Error assembling Mint tx")
		return err
	}

	// TODO: actually wait
	//configuration.EthHttpClient.WaitForTransactionReceipt(context.Background(), tx.Hash())
	// sleep
	time.Sleep(2 * time.Second)
	//
	//signerFn, err := signerv2.PrivateKeySignerFn(configuration.EcdsaPrivateKey, configuration.ChainId)
	//if err != nil {
	//	return err
	//}
	signerConfig := signerv2.Config{
		PrivateKey: configuration.EcdsaPrivateKey,
		Password:   "",
	}
	signerFn, _, err := signerv2.SignerFromConfig(signerConfig, configuration.ChainId)
	if err != nil {
		return err
	}
	w, err := wallet.NewPrivateKeyWallet(configuration.EthHttpClient, signerFn, configuration.OperatorAddress, configuration.Logger)
	if err != nil {
		return err
	}
	txMgr := txmgr.NewSimpleTxManager(w, configuration.EthHttpClient, configuration.Logger, configuration.OperatorAddress)
	eigenMetrics := metrics.NewNoopMetrics()
	eigenLayerWriter, err := elcontracts.BuildELChainWriter(delegationManagerAddr, avsDirectoryAddr,
		configuration.EthHttpClient, configuration.Logger, eigenMetrics, txMgr)
	if err != nil {
		return err
	}

	_, err = eigenLayerWriter.DepositERC20IntoStrategy(context.Background(), strategyAddr, amount)
	if err != nil {
		configuration.Logger.Errorf("Error depositing into strategy")
		return err
	}
	return nil
}
