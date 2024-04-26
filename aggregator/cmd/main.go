package main

import (
	"fmt"
	"github.com/urfave/cli/v2"
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

func aggregatorMain(context *cli.Context) error {
	log.Println("Starting aggregator...")

	baseConfigFilePath := context.String("base-config-file")
	aggregatorConfigFilePath := context.String("aggregator-config-file")

	aggregatorConfig := config.NewAggregatorConfig(baseConfigFilePath, aggregatorConfigFilePath)

	err := rpc_server.Serve(aggregatorConfig)

	if err != nil {
		log.Fatal("Error starting aggregator server: ", err)
	}

	return nil
}
