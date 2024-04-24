package main

import (
	"context"
	"log"
	"os"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"github.com/ethereum/go-ethereum/common"
	"github.com/urfave/cli"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/core/types"
)

var (
	configFlag = &cli.StringFlag{
		Name:     "config",
		Required: true,
		Usage:    "the path to the Operators `CONFIGURATION FILE`",
	}
)

var flags = []cli.Flag{
	configFlag,
}

func main() {
	log.Println("Booting operator ...")

	app := &cli.App{
		Name:   "Aligned Layer Operator",
		Flags:  flags,
		Action: operatorMain,
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln("Operator failed.", "Message:", err)
	}
}

func operatorMain(ctx *cli.Context) error {

	logger, err := sdklogging.NewZapLogger("development")
	if err != nil {
		return err
	}

	configPath := ctx.GlobalString(config.ConfigFileFlag.Name)
	nodeConfig := types.NodeConfig{}
	err = sdkutils.ReadYamlConfig(configPath, &nodeConfig)
	if err != nil {
		return err
	}

	serviceManagerAddr := common.HexToAddress(nodeConfig.AlignedLayerServiceManagerAddr)
	operatorStateRetrieverAddr := common.HexToAddress(nodeConfig.OperatorStateRetrieverAddr)
	ethWsClient, err := eth.NewClient(nodeConfig.EthWsUrl)
	if err != nil {
		log.Fatalf("Cannot create ws ethclient", "err", err)
		return err
	}

	avsSubscriber, err := chainio.NewAvsSubscriberFromConfig(serviceManagerAddr, operatorStateRetrieverAddr, ethWsClient, logger)
	newTaskCreatedChan := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewTaskCreated)
	sub := avsSubscriber.SubscribeToNewTasks(newTaskCreatedChan)

	for {
		select {
		case <-context.Background().Done():
			log.Println("Operator shutting down...")
			return nil
		case err := <-sub.Err():
			log.Println("Error in websocket subscription", "err", err)
			sub.Unsubscribe()
			sub = avsSubscriber.SubscribeToNewTasks(newTaskCreatedChan)
		case newTaskCreatedLog := <-newTaskCreatedChan:

			log.Println("The received task's index is: %d", newTaskCreatedLog.TaskIndex)

			// taskResponse := o.ProcessNewTaskCreatedLog(newTaskCreatedLog)
			// signedTaskResponse, err := o.SignTaskResponse(taskResponse)
			// if err != nil {
			// 	continue
			// }
			// go o.aggregatorRpcClient.SendSignedTaskResponseToAggregator(signedTaskResponse)
		}
	}
}
