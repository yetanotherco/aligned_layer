package main

import (
	"context"
	"log"
	"os"

	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"github.com/urfave/cli"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/core/types"
	operator "github.com/yetanotherco/aligned_layer/operator/pkg"
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
		Flags:  config.Flags,
		Action: operatorMain,
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln("Operator failed.", "Message:", err)
	}
}

func operatorMain(ctx *cli.Context) error {
	operatorConfigFilePath := ctx.String("operator-config-file")
	nodeConfig := types.NodeConfig{}
	err := sdkutils.ReadYamlConfig(operatorConfigFilePath, &nodeConfig)
	if err != nil {
		return err
	}

	operator, _ := operator.NewOperatorFromConfig(nodeConfig)

	log.Println("Operator starting...")
	err = operator.Start(context.Background())
	if err != nil {
		return err
	}

	log.Println("Operator started")

	return nil
}
