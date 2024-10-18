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

// We use the nolint directive extracted from golangci-lint to supress linting
// -> https://github.com/golangci/golangci-lint/blob/master/docs/src/docs/usage/false-positives.mdx#nolint-directive
//
//nolint:all
func main() {
	panic("THIS IS THE PANIC IN MAIN")
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
