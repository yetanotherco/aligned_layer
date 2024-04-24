package main

import (
	"fmt"
	"github.com/urfave/cli"
	"github.com/yetanotherco/aligned_layer/aggregator/internal/rpc_server"
	"github.com/yetanotherco/aligned_layer/core/config"
	"log"
	"os"
)

var (
	// Version is the version of the binary.
	Version   string
	GitCommit string
	GitDate   string
)

func main() {
	app := cli.NewApp()

	app.Flags = config.Flags
	app.Version = fmt.Sprintf("%s-%s-%s", Version, GitCommit, GitDate)
	app.Name = "aligned-layer-aggregator"
	app.Usage = "Aligned Layer Aggregator"
	app.Description = "Service that aggregates signed responses from operator nodes."
	app.Action = aggregatorMain

	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln("Application failed.", "Message:", err)
	}
}

func aggregatorMain(context *cli.Context) {
	log.Println("Starting aggregator...")

	setupConfigFilePath := context.String("setup-config")
	baseConfig, err := config.NewBaseConfig(setupConfigFilePath)

	if err != nil {
		log.Fatal("Error reading base config: ", err)
	}

	configFilePath := context.String("config")
	alignedLayerDeploymentFilePath := context.String("aligned-layer-deployment")
	ecdsaPrivateKeyString := context.String("ecdsa-private-key")

	aggregatorConfig, err := config.NewConfig(configFilePath, alignedLayerDeploymentFilePath, ecdsaPrivateKeyString, baseConfig.Logger, baseConfig.EthRpcClient, baseConfig.EthWsClient)

	if err != nil {
		log.Fatal("Error reading aggregator config: ", err)
	}

	err = rpc_server.Serve(aggregatorConfig)

	if err != nil {
		log.Fatal("Error starting aggregator server: ", err)
	}
}
