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

func NewMockConfig(ecdsaPrivateKeyStr, alignedLayerOperatorStateRetrieverAddrStr, alignedLayerServiceManagerAddrStr string) *config.Config {
	etcRpcUrl := "http://localhost:8545"
	ethWsUrl := "ws://localhost:8545"
	eigenMetricsIpPortAddress := "localhost:9090"
	alignedLayerOperatorStateRetrieverAddr := common.HexToAddress(alignedLayerOperatorStateRetrieverAddrStr)
	alignedLayerServiceManagerAddr := common.HexToAddress(alignedLayerServiceManagerAddrStr)
	alignedLayerRegistryCoordinatorAddr := common.HexToAddress("0x67d269191c92caf3cd7723f116c85e6e9bf55933")
	chainId := big.NewInt(31337)
	blsPublicKeyCompendiumAddress := common.HexToAddress("0x322813fd9a801c5507c9de605d63cea4f2ce6c44")
	slasherAddr := common.HexToAddress("0x")
	operatorAddress := common.HexToAddress("0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266")
	avsServiceManagerAddress := common.HexToAddress("0xc3e53f4d16ae77db1c982e75a937b9f60fe63690")

	logger, err := sdklogging.NewZapLogger("development")
	if err != nil {
		fmt.Println("Could not initialize logger")
	}

	ecdsaPrivateKey, err := crypto.HexToECDSA(ecdsaPrivateKeyStr)
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

	return &config.Config{
		EcdsaPrivateKey:                        ecdsaPrivateKey,
		BlsPrivateKey:                          nil,
		Logger:                                 logger,
		EigenMetricsIpPortAddress:              eigenMetricsIpPortAddress,
		EthRpcUrl:                              etcRpcUrl,
		EthWsUrl:                               ethWsUrl,
		EthHttpClient:                          ethRpcClient,
		EthWsClient:                            ethWsClient,
		AlignedLayerOperatorStateRetrieverAddr: alignedLayerOperatorStateRetrieverAddr,
		AlignedLayerServiceManagerAddr:         alignedLayerServiceManagerAddr,
		AlignedLayerRegistryCoordinatorAddr:    alignedLayerRegistryCoordinatorAddr,
		ChainId:                                chainId,
		BlsPublicKeyCompendiumAddress:          blsPublicKeyCompendiumAddress,
		SlasherAddr:                            slasherAddr,
		OperatorAddress:                        operatorAddress,
		AVSServiceManagerAddress:               avsServiceManagerAddress,
		EnableMetrics:                          true,
	}
}
