package main

import (
	"log"
	"os"

	"github.com/urfave/cli/v2"
	"github.com/yetanotherco/aligned_layer/operator/cmd/actions"
)

func main() {
	app := &cli.App{
		Name: "Aligned Layer Node Operator",
		Commands: []*cli.Command{
			actions.RegisterCommand,
			actions.StartCommand,
			actions.DepositIntoStrategyCommand,
		},
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln("Operator failed.", "Message:", err)
	}
}
