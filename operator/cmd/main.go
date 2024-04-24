package main

import (
	"context"
	"fmt"
	"log"
	"os"

	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
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
	configPath := ctx.GlobalString(config.ConfigFileFlag.Name)
	nodeConfig := types.NodeConfig{}
	err := sdkutils.ReadYamlConfig(configPath, &nodeConfig)
	if err != nil {
		return err
	}

	// configFile, err := config.NewConfig(ctx)
	// if err != nil {
	// 	return fmt.Errorf("could not load Operator configuration file: %v", err)
	// }

	fmt.Println("NODE CONFIG: ", nodeConfig)

	serviceManagerAddr := nodeConfig.AlignedLayerServiceManagerAddr
	operatorStateRetrieverAddr := nodeConfig.BlsOperatorStateRetrieverAddr
	ethWsClient := nodeConfig.EthWsClient
	logger := nodeConfig.Logger

	avsSubscriber, err := chainio.NewAvsSubscriberFromConfig(serviceManagerAddr, operatorStateRetrieverAddr, ethWsClient, logger)
	newTaskCreatedChan := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewTaskCreated)
	fmt.Println("ANTES")
	sub := avsSubscriber.SubscribeToNewTasks(newTaskCreatedChan)
	fmt.Println("DESPUES")

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
