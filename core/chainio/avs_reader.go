package chainio

import (
	gethcommon "github.com/ethereum/go-ethereum/common"
	contractERC20Mock "github.com/yetanotherco/aligned_layer/contracts/bindings/ERC20Mock"
	"github.com/yetanotherco/aligned_layer/core/config"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	sdkavsregistry "github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/logging"
)

type AvsReader struct {
	sdkavsregistry.AvsRegistryReader
	AvsContractBindings *AvsServiceBindings
	logger              logging.Logger
}

func NewAvsReaderFromConfig(c *config.BaseConfig) (*AvsReader, error) {

	buildAllConfig := clients.BuildAllConfig{
		EthHttpUrl:                 c.EthRpcUrl,
		EthWsUrl:                   c.EthWsUrl,
		RegistryCoordinatorAddr:    c.AlignedLayerDeploymentConfig.AlignedLayerRegistryCoordinatorAddr.String(),
		OperatorStateRetrieverAddr: c.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr.String(),
		AvsName:                    "AlignedLayer",
		PromMetricsIpPortAddress:   c.EigenMetricsIpPortAddress,
	}

	clients, err := clients.BuildAll(buildAllConfig, c.EcdsaPrivateKey, c.Logger)
	if err != nil {
		return nil, err
	}

	avsRegistryReader := clients.AvsRegistryChainReader

	avsServiceBindings, err := NewAvsServiceBindings(c.AlignedLayerDeploymentConfig.AlignedLayerServiceManagerAddr, c.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr, c.EthRpcClient, c.Logger)
	if err != nil {
		return nil, err
	}

	return &AvsReader{
		AvsRegistryReader:   avsRegistryReader,
		AvsContractBindings: avsServiceBindings,
		logger:              c.Logger,
	}, nil
}

func (r *AvsReader) GetErc20Mock(tokenAddr gethcommon.Address) (*contractERC20Mock.ContractERC20Mock, error) {
	erc20Mock, err := contractERC20Mock.NewContractERC20Mock(tokenAddr, r.AvsContractBindings.ethClient)
	if err != nil {
		r.logger.Error("Failed to fetch ERC20Mock contract", "err", err)
		return nil, err
	}
	return erc20Mock, nil
}
