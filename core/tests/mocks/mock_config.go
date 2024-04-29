package mocks

import (
	"fmt"
	"math/big"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/yetanotherco/aligned_layer/core/config"
)

func NewMockConfig() *config.BaseConfig {
	etcRpcUrl := "http://localhost:8545"
	ethWsUrl := "ws://localhost:8545"
	eigenMetricsIpPortAddress := "localhost:9090"
	alignedLayerOperatorStateRetrieverAddr := common.HexToAddress("0x809d550fca64d94Bd9F66E60752A544199cfAC3D")
	alignedLayerServiceManagerAddr := common.HexToAddress("0xc3e53F4d16Ae77Db1c982e75a937B9f60FE63690")
	alignedLayerRegistryCoordinatorAddr := common.HexToAddress("0x84eA74d481Ee0A5332c457a4d796187F6Ba67fEB")
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
