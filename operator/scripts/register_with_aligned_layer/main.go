package main

import (
	"context"
	"crypto/ecdsa"
	"encoding/hex"
	clitypes "github.com/Layr-Labs/eigenlayer-cli/pkg/types"
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	ecdsa2 "github.com/Layr-Labs/eigensdk-go/crypto/ecdsa"
	"github.com/Layr-Labs/eigensdk-go/types"
	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/urfave/cli"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	"log"
	"math/big"
	"os"
	"time"
)

var (
	OperatorConfigFileFlag = cli.StringFlag{
		Name:     "operator-config",
		Required: true,
		Usage:    "Load operator configuration from `FILE`",
	}
)

func main() {
	app := cli.NewApp()
	app.Name = "aligned_layer"
	app.Usage = "Tool for registering operator to AlignedLayer AVS"
	app.Flags = append(config.Flags, OperatorConfigFileFlag)
	app.Action = registerWithAlignedLayer

	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}

func registerWithAlignedLayer(ctx *cli.Context) error {
	configuration, err := config.NewConfig(ctx)
	if err != nil {
		return err
	}

	configPath := ctx.GlobalString(OperatorConfigFileFlag.Name)
	nodeConfig := clitypes.OperatorConfig{}
	err = sdkutils.ReadYamlConfig(configPath, &nodeConfig)
	if err != nil {
		return err
	}

	ecdsaPrivateKeyPassword := os.Getenv("ECDSA_PRIVATE_KEY_PASSWORD")
	if ecdsaPrivateKeyPassword == "" {
		log.Println("ECDSA_PRIVATE_KEY_PASSWORD environment variable not set, using empty string")
	}

	privateKey, err := ecdsa2.ReadKey(nodeConfig.PrivateKeyStorePath, ecdsaPrivateKeyPassword)
	if err != nil {
		return err
	}

	configuration.Logger.Info("Registering operator", "private_key", hex.EncodeToString(privateKey.D.Bytes()))

	blsPrivateKeyPassword := os.Getenv("BLS_PRIVATE_KEY_PASSWORD")
	if blsPrivateKeyPassword == "" {
		log.Println("BLS_PRIVATE_KEY_PASSWORD environment variable not set, using empty string")
	}

	blsKeyPair, err := bls.ReadPrivateKeyFromFile(nodeConfig.BlsPrivateKeyStorePath, blsPrivateKeyPassword)
	if err != nil {
		return err
	}

	quorumNumbers := []byte{0}

	// Generate salt and expiry
	privateKeyBytes := []byte(blsKeyPair.PrivKey.String())
	salt := [32]byte{}

	copy(salt[:], crypto.Keccak256([]byte("churn"), []byte(time.Now().String()), quorumNumbers, privateKeyBytes))

	expiry := big.NewInt(time.Now().Add(10 * time.Minute).Unix())
	quorumNumbersArr := types.QuorumNums{0}
	socket := "Not Needed"

	err = RegisterOperator(context.Background(), configuration,
		blsKeyPair, socket, quorumNumbersArr, privateKey, salt, expiry)
	if err != nil {
		configuration.Logger.Error("Failed to register operator", "err", err)
		return err
	}

	return nil
}

// RegisterOperator operator registers the operator with the given public key for the given quorum IDs.
// RegisterOperator registers a new operator with the given public key and socket with the provided quorum ids.
// If the operator is already registered with a given quorum id, the transaction will fail (noop) and an error
// will be returned.
func RegisterOperator(
	ctx context.Context,
	configuration *config.Config,
	keypair *bls.KeyPair,
	socket string,
	quorumNumbers types.QuorumNums,
	operatorEcdsaPrivateKey *ecdsa.PrivateKey,
	operatorToAvsRegistrationSigSalt [32]byte,
	operatorToAvsRegistrationSigExpiry *big.Int,
) error {
	writer, err := chainio.NewAvsWriterFromConfig(configuration)
	if err != nil {
		configuration.Logger.Error("Failed to create AVS writer", "err", err)
		return err
	}

	_, err = writer.RegisterOperatorInQuorumWithAVSRegistryCoordinator(ctx, operatorEcdsaPrivateKey,
		operatorToAvsRegistrationSigSalt, operatorToAvsRegistrationSigExpiry, keypair,
		quorumNumbers, socket)

	if err != nil {
		configuration.Logger.Error("Failed to register operator", "err", err)
		return err
	}

	return nil
}
