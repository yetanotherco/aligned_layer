package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"time"

	"github.com/urfave/cli/v2"
	"github.com/yetanotherco/aligned_layer/aggregator/internal/pkg"
	"github.com/yetanotherco/aligned_layer/core/config"
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

const garbageCollectorPeriod = time.Second * 150 //TODO change to time.Day * 1
const garbageCollectorTasksAge = uint64(10)     //TODO change to 2592000, 1 month of blocks

func aggregatorMain(ctx *cli.Context) error {

	configFilePath := ctx.String(config.ConfigFileFlag.Name)
	aggregatorConfig := config.NewAggregatorConfig(configFilePath)

	aggregator, err := pkg.NewAggregator(*aggregatorConfig)
	if err != nil {
		aggregatorConfig.BaseConfig.Logger.Error("Cannot create aggregator", "err", err)
		return err
	}

	// Supervisor revives garbage collector
	go func() {
		for {
			aggregator.ClearTasksFromMaps(garbageCollectorPeriod, garbageCollectorTasksAge)
		}
	}()

	// Listen for new task created in the ServiceManager contract in a separate goroutine, both V1 and V2 subscriptions:
	go func() {
		listenErr := aggregator.SubscribeToNewTasks()
		if listenErr != nil {
			aggregatorConfig.BaseConfig.Logger.Fatal("Error subscribing for new tasks", "err", listenErr)
		}
	}()

	err = aggregator.Start(context.Background())

	return err
}
