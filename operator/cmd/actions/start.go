package actions

import (
	"context"
	"log"

	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"github.com/urfave/cli/v2"
	"github.com/sanbornm/go-selfupdate/selfupdate"
	"github.com/yetanotherco/aligned_layer/core/config"
	operator "github.com/yetanotherco/aligned_layer/operator/pkg"
)

var version = "1.0"

//TODO: initialize this in main to configure
var updater = &selfupdate.Updater{
	CurrentVersion: version,                  // Manually update the const, or set it using `go build -ldflags="-X main.VERSION=<newver>" -o hello-updater src/hello-updater/main.go`
	ApiURL:         "http://localhost:8080/", // The server hosting `$CmdName/$GOOS-$ARCH.json` which contains the checksum for the binary
	BinURL:         "http://localhost:8080/", // The server hosting the zip file containing the binary application which is a fallback for the patch method
	DiffURL:        "http://localhost:8080/", // The server hosting the binary patch diff for incremental updates
	Dir:            "update/",                // The directory created by the app when run which stores the cktime file
	CmdName:        "operator",          // The app name which is appended to the ApiURL to look for an update
	ForceCheck:     true,                     // For this example, always check for an update unless the version is "dev"
}


var StartFlags = []cli.Flag{
	config.ConfigFileFlag,
}

var StartCommand = &cli.Command{
	Name:        "start",
	Description: "CLI command to boot operator",
	Flags:       StartFlags,
	Action:      operatorMain,
}

func operatorMain(ctx *cli.Context) error {
	operatorConfigFilePath := ctx.String("config")
	operatorConfig := config.NewOperatorConfig(operatorConfigFilePath)
	err := sdkutils.ReadYamlConfig(operatorConfigFilePath, &operatorConfig)
	if err != nil {
		return err
	}

	operator, err := operator.NewOperatorFromConfig(*operatorConfig)
	if err != nil {
		return err
	}

	log.Println("Updating Operator to latest version...")
	err = updater.BackgroundRun()
	if err != nil {
		log.Println("Failed to update operator:", err)
	}

	log.Println("Operator starting...")
	err = operator.Start(context.Background())
	if err != nil {
		return err
	}

	log.Println("Operator started")

	return nil
}
