package operator

import (
	"context"
	"github.com/Layr-Labs/eigensdk-go/types"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	"math/big"
	"time"
)

// RegisterOperator operator registers the operator with the given public key for the given quorum IDs.
// RegisterOperator registers a new operator with the given public key and socket with the provided quorum ids.
// If the operator is already registered with a given quorum id, the transaction will fail (noop) and an error
// will be returned.
func RegisterOperator(
	ctx context.Context,
	configuration *config.OperatorConfig,
	operatorToAvsRegistrationSigSalt [32]byte,
) error {
	writer, err := chainio.NewAvsWriterFromConfig(configuration.BaseConfig, configuration.EcdsaConfig)
	if err != nil {
		configuration.BaseConfig.Logger.Error("Failed to create AVS writer", "err", err)
		return err
	}

	operatorToAvsRegistrationSigExpiry := big.NewInt(time.Now().Add(10 * time.Minute).Unix())
	socket := "Not Needed"

	quorumNumbers := types.QuorumNums{0}

	_, err = writer.RegisterOperatorInQuorumWithAVSRegistryCoordinator(ctx, configuration.EcdsaConfig.PrivateKey,
		operatorToAvsRegistrationSigSalt, operatorToAvsRegistrationSigExpiry, configuration.BlsConfig.KeyPair,
		quorumNumbers, socket)

	if err != nil {
		configuration.BaseConfig.Logger.Error("Failed to register operator", "err", err)
		return err
	}

	return nil
}
