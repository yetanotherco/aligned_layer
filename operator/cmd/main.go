package main

import (
	"log"
	"os"

	"github.com/urfave/cli/v2"
	"github.com/yetanotherco/aligned_layer/operator/cmd/actions"
)

var (
	// This will be set by the Go linker during build time
	Version string
)

func main() {
	app := &cli.App{
		Name: "Aligned Layer Node Operator",
		Commands: []*cli.Command{
			actions.RegisterCommand,
			actions.StartCommand,
			actions.DepositIntoStrategyCommand,
		},
		Version: Version,
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln("Operator failed.", "Message:", err)
	}
}
