// package main

// import (
// 	"context"
// 	"log"
// 	"math/big"
// 	"os"
// 	"time"

// 	"github.com/Layr-Labs/eigensdk-go/types"
// 	"github.com/ethereum/go-ethereum/crypto"
// 	"github.com/urfave/cli/v2"
// 	"github.com/yetanotherco/aligned_layer/core/chainio"
// 	"github.com/yetanotherco/aligned_layer/core/config"
// )

// var flags = []cli.Flag{
// 	config.ConfigFileFlag,
// }

// func main() {
// 	app := cli.NewApp()
// 	app.Name = "aligned_layer"
// 	app.Usage = "Tool for registering operator to AlignedLayer AVS"
// 	app.Flags = flags
// 	app.Action = registerWithAlignedLayer

// 	err := app.Run(os.Args)
// 	if err != nil {
// 		log.Fatal(err)
// 	}
// }

// func registerWithAlignedLayer(ctx *cli.Context) error {
// 	configuration := config.NewOperatorConfig(ctx.String(config.ConfigFileFlag.Name))

// 	quorumNumbers := []byte{0}

// 	// Generate salt and expiry
// 	privateKeyBytes := []byte(configuration.BlsConfig.KeyPair.PrivKey.String())
// 	salt := [32]byte{}

// 	copy(salt[:], crypto.Keccak256([]byte("churn"), []byte(time.Now().String()), quorumNumbers, privateKeyBytes))

// 	expiry := big.NewInt(time.Now().Add(10 * time.Minute).Unix())
// 	quorumNumbersArr := types.QuorumNums{0}
// 	socket := "Not Needed"

// 	err := RegisterOperator(context.Background(), configuration,
// 		socket, quorumNumbersArr, salt, expiry)
// 	if err != nil {
// 		configuration.BaseConfig.Logger.Error("Failed to register operator", "err", err)
// 		return err
// 	}

// 	return nil
// }

// // RegisterOperator operator registers the operator with the given public key for the given quorum IDs.
// // RegisterOperator registers a new operator with the given public key and socket with the provided quorum ids.
// // If the operator is already registered with a given quorum id, the transaction will fail (noop) and an error
// // will be returned.
// func RegisterOperator(
// 	ctx context.Context,
// 	configuration *config.OperatorConfig,
// 	socket string,
// 	quorumNumbers types.QuorumNums,
// 	operatorToAvsRegistrationSigSalt [32]byte,
// 	operatorToAvsRegistrationSigExpiry *big.Int,
// ) error {
// 	writer, err := chainio.NewAvsWriterFromConfig(configuration.BaseConfig, configuration.EcdsaConfig)
// 	if err != nil {
// 		configuration.BaseConfig.Logger.Error("Failed to create AVS writer", "err", err)
// 		return err
// 	}

// 	_, err = writer.RegisterOperatorInQuorumWithAVSRegistryCoordinator(ctx, configuration.EcdsaConfig.PrivateKey,
// 		operatorToAvsRegistrationSigSalt, operatorToAvsRegistrationSigExpiry, configuration.BlsConfig.KeyPair,
// 		quorumNumbers, socket)

// 	if err != nil {
// 		configuration.BaseConfig.Logger.Error("Failed to register operator", "err", err)
// 		return err
// 	}

// 	return nil
// }
