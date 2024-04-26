package mocks

import (
	"fmt"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/yetanotherco/aligned_layer/core/config"
	"math/big"
)

func NewMockConfig() *config.BaseConfig {
	etcRpcUrl := "http://localhost:8545"
	ethWsUrl := "ws://localhost:8545"
	eigenMetricsIpPortAddress := "localhost:9090"
	alignedLayerOperatorStateRetrieverAddr := common.HexToAddress("0x9d4454b023096f34b160d6b654540c56a1f81688")
	alignedLayerServiceManagerAddr := common.HexToAddress("0xc5a5c42992decbae36851359345fe25997f5c42d")
	alignedLayerRegistryCoordinatorAddr := common.HexToAddress("0x67d269191c92caf3cd7723f116c85e6e9bf55933")
	chainId := big.NewInt(31337)
	slasherAddr := common.HexToAddress("0x")
	delegationManagerAddress := common.HexToAddress("0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9")

	logger, err := sdklogging.NewZapLogger("development")
	if err != nil {
		fmt.Println("Could not initialize logger")
	}

	ecdsaPrivateKey, err := crypto.HexToECDSA("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
	if err != nil {
		logger.Errorf("Cannot parse ecdsa private key", "err", err)
	}

	ethRpcClient, err := eth.NewClient(etcRpcUrl)
	if err != nil {
		logger.Errorf("Cannot create http ethclient", "err", err)
	}

	ethWsClient, err := eth.NewClient(ethWsUrl)
	if err != nil {
		logger.Errorf("Cannot create ws ethclient", "err", err)
	}

	return &config.BaseConfig{
		EcdsaPrivateKey:           ecdsaPrivateKey,
		BlsPrivateKey:             nil,
		Logger:                    logger,
		EigenMetricsIpPortAddress: eigenMetricsIpPortAddress,
		EthRpcUrl:                 etcRpcUrl,
		EthWsUrl:                  ethWsUrl,
		EthRpcClient:              ethRpcClient,
		EthWsClient:               ethWsClient,
		ChainId:                   chainId,
		Signer:                    nil,
		AlignedLayerDeploymentConfig: &config.AlignedLayerDeploymentConfig{
			AlignedLayerOperatorStateRetrieverAddr: alignedLayerOperatorStateRetrieverAddr,
			AlignedLayerServiceManagerAddr:         alignedLayerServiceManagerAddr,
			AlignedLayerRegistryCoordinatorAddr:    alignedLayerRegistryCoordinatorAddr,
		},
		EigenLayerDeploymentConfig: &config.EigenLayerDeploymentConfig{
			DelegationManagerAddr: delegationManagerAddress,
			SlasherAddr:           slasherAddr,
		},
	}
}
