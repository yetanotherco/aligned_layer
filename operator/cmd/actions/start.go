package actions

import (
	"bytes"
	"context"
	"encoding/json"
	"log"
	"net/http"

	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/urfave/cli/v2"
	"github.com/yetanotherco/aligned_layer/core/config"
	operator "github.com/yetanotherco/aligned_layer/operator/pkg"
	"golang.org/x/crypto/sha3"
)

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

	// hash version
	hash := sha3.NewLegacyKeccak256()
	hash.Write([]byte(ctx.App.Version))

	// get hash
	version := hash.Sum(nil)

	// sign version
	signature, err := crypto.Sign(version[:], operatorConfig.EcdsaConfig.PrivateKey)
	if err != nil {
		return err
	}

	body := map[string]interface{}{
		"version":   ctx.App.Version,
		"signature": signature,
	}
	bodyBuffer := new(bytes.Buffer)

	bodyReader := json.NewEncoder(bodyBuffer)
	err = bodyReader.Encode(body)
	if err != nil {
		return err
	}

	// send version to operator tracker server
	endpoint := operatorConfig.Operator.OperatorTrackerIpPortAddress + "/versions"
	operator.Logger.Info("Sending version to operator tracker server: ", "endpoint", endpoint)

	res, err := http.Post(endpoint, "application/json",
		bodyBuffer)
	if err != nil {
		// Dont prevent operator from starting if operator tracker server is down
		operator.Logger.Warn("Error sending version to metrics server: ", "err", err)
	} else if res.StatusCode != http.StatusCreated && res.StatusCode != http.StatusNoContent {
		operator.Logger.Warn("Error sending version to operator tracker server: ", "status_code", res.StatusCode)
	}

	operator.Logger.Info("Operator starting...")
	err = operator.Start(context.Background())
	if err != nil {
		return err
	}

	log.Println("Operator started")

	return nil
}
