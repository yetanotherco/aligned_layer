package main

import (
	"fmt"
	"github.com/urfave/cli"
	"github.com/yetanotherco/aligned_layer/aggregator/internal/pkg"
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

	baseConfigFilePath := context.String(config.BaseConfigFileFlag.Name)
	aggregatorConfigFilePath := context.String(config.AggregatorConfigFileFlag.Name)

	aggregatorConfig := config.NewAggregatorConfig(baseConfigFilePath, aggregatorConfigFilePath)

	aggregator, err := pkg.NewAggregator(*aggregatorConfig)
	if err != nil {
		aggregatorConfig.BaseConfig.Logger.Error("Cannot create aggregator", "err", err)
		return err
	}

	// Listen for tasks in a separate goroutine
	go func() {
		listenErr := aggregator.ListenForTasks()
		if listenErr != nil {
			// TODO: Retry listening for tasks
			aggregatorConfig.BaseConfig.Logger.Error("Error listening for tasks", "err", listenErr)
		}
	}()

	// Serve the aggregator in the main goroutine
	err = aggregator.Serve()
	if err != nil {
		aggregatorConfig.BaseConfig.Logger.Error("Error serving aggregator", "err", err)
		return err
	}

	return nil
}
