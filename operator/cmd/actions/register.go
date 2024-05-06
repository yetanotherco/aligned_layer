package actions

import (
	"context"
	"math/big"
	"time"

	"github.com/Layr-Labs/eigensdk-go/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/urfave/cli/v2"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
)

var registerFlags = []cli.Flag{
	config.ConfigFileFlag,
}

var RegisterCommand = &cli.Command{
	Name:        "register",
	Usage:       "Register operator with Aligned Layer",
	Description: "CLI command to register opeartor with Aligned Layer",
	Flags:       registerFlags,
	Action:      registerOperatorMain,
}

func registerOperatorMain(ctx *cli.Context) error {
	config := config.NewOperatorConfig(ctx.String(config.ConfigFileFlag.Name))

	quorumNumbers := []byte{0}

	// Generate salt and expiry
	privateKeyBytes := []byte(config.BlsConfig.KeyPair.PrivKey.String())
	salt := [32]byte{}

	copy(salt[:], crypto.Keccak256([]byte("churn"), []byte(time.Now().String()), quorumNumbers, privateKeyBytes))

	expiry := big.NewInt(time.Now().Add(10 * time.Minute).Unix())
	quorumNumbersArr := types.QuorumNums{0}
	socket := "Not Needed"

	err := registerOperator(context.Background(), config,
		socket, quorumNumbersArr, salt, expiry)
	if err != nil {
		config.BaseConfig.Logger.Error("Failed to register operator", "err", err)
		return err
	}

	return nil
}

// RegisterOperator operator registers the operator with the given public key for the given quorum IDs.
// RegisterOperator registers a new operator with the given public key and socket with the provided quorum ids.
// If the operator is already registered with a given quorum id, the transaction will fail (noop) and an error
// will be returned.
func registerOperator(
	ctx context.Context,
	configuration *config.OperatorConfig,
	socket string,
	quorumNumbers types.QuorumNums,
	operatorToAvsRegistrationSigSalt [32]byte,
	operatorToAvsRegistrationSigExpiry *big.Int,
) error {
	writer, err := chainio.NewAvsWriterFromConfig(configuration.BaseConfig, configuration.EcdsaConfig)
	if err != nil {
		configuration.BaseConfig.Logger.Error("Failed to create AVS writer", "err", err)
		return err
	}

	_, err = writer.RegisterOperatorInQuorumWithAVSRegistryCoordinator(ctx, configuration.EcdsaConfig.PrivateKey,
		operatorToAvsRegistrationSigSalt, operatorToAvsRegistrationSigExpiry, configuration.BlsConfig.KeyPair,
		quorumNumbers, socket)

	if err != nil {
		configuration.BaseConfig.Logger.Error("Failed to register operator", "err", err)
		return err
	}

	return nil
}
