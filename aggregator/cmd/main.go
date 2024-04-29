package main

import (
	"fmt"
	"github.com/urfave/cli/v2"
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

var flags = []cli.Flag{
	config.ConfigFileFlag,
}

func main() {
	app := cli.NewApp()

	app.Flags = flags
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

	configFilePath := context.String(config.ConfigFileFlag.Name)
	aggregatorConfig := config.NewAggregatorConfig(configFilePath)

	aggregator, err := pkg.NewAggregator(*aggregatorConfig)
	if err != nil {
		aggregatorConfig.BaseConfig.Logger.Error("Cannot create aggregator", "err", err)
		return err
	}

	// Listen for new task created in the ServiceManager contract in a separate goroutine
	go func() {
		listenErr := aggregator.SubscribeToNewTasks()
		if listenErr != nil {
			// TODO: Retry listening for tasks
			aggregatorConfig.BaseConfig.Logger.Error("Error listening for tasks", "err", listenErr)
		}
	}()

	// Listens for task responses signed by operators
	err = aggregator.ServeOperators()
	if err != nil {
		aggregatorConfig.BaseConfig.Logger.Error("Error serving aggregator", "err", err)
		return err
	}

	return nil
}
