package chainio

import (
	"fmt"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	sdkavsregistry "github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
)

type AvsReader struct {
	sdkavsregistry.AvsRegistryReader
	AvsContractBindings *AvsServiceBindings
	logger              logging.Logger
}

// NOTE(marian): The initialization of the AVS reader is hardcoded, but should be loaded from a
// configuration file.
// The hardcoded values are:
//   - logger
//   - EthHttpUrl
//   - EthWsUrl
//   - RegistryCoordinatorAddr
//   - OperatorStateRetrieverAddr
//   - alignedLayerServiceManagerAddr
//   - ecdsaPrivateKey

// The following function signature was the one in the aligned_layer_testnet repo:
// func NewAvsReaderFromConfig(c *config.Config) (*AvsReader, error) {
func NewAvsReaderFromConfig() (*AvsReader, error) {
	logger, err := sdklogging.NewZapLogger("development")
	if err != nil {
		fmt.Println("Could not initialize logger")
	}

	buildAllConfig := clients.BuildAllConfig{
		EthHttpUrl:                 "http://localhost:8545",
		EthWsUrl:                   "ws://localhost:8545",
		RegistryCoordinatorAddr:    "0x67d269191c92Caf3cD7723F116c85e6E9bf55933",
		OperatorStateRetrieverAddr: "0x9d4454B023096f34B160D6B654540c56A1F81688",
		AvsName:                    "AlignedLayer",
		PromMetricsIpPortAddress:   ":9090",
	}

	ecdsaPrivateKeyString := "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
	ecdsaPrivateKey, err := crypto.HexToECDSA(ecdsaPrivateKeyString)
	if err != nil {
		logger.Errorf("Cannot parse ecdsa private key", "err", err)
	}

	clients, _ := clients.BuildAll(buildAllConfig, ecdsaPrivateKey, logger)

	alignedLayerServiceManagerAddr := common.HexToAddress("0xc5a5C42992dECbae36851359345FE25997F5C42d")

	avsRegistryReader := clients.AvsRegistryChainReader

	ethHttpClient, err := eth.NewClient(buildAllConfig.EthHttpUrl)
	if err != nil {
		panic(err)
	}

	operatorStateRetrieverAddr := common.HexToAddress(buildAllConfig.OperatorStateRetrieverAddr)
	avsServiceBindings, err := NewAvsServiceBindings(alignedLayerServiceManagerAddr, operatorStateRetrieverAddr, ethHttpClient, logger)
	if err != nil {
		return nil, err
	}

	return &AvsReader{
		AvsRegistryReader:   avsRegistryReader,
		AvsContractBindings: avsServiceBindings,
		logger:              logger,
	}, nil
}
