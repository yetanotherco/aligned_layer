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

func NewAvsReaderFromConfig(c *config.AvsConfig) (*AvsReader, error) {

	buildAllConfig := clients.BuildAllConfig{
		EthHttpUrl:                 c.BaseConfig.EthRpcUrl,
		EthWsUrl:                   c.BaseConfig.EthWsUrl,
		RegistryCoordinatorAddr:    c.BaseConfig.AlignedLayerDeploymentConfig.AlignedLayerRegistryCoordinatorAddr.String(),
		OperatorStateRetrieverAddr: c.BaseConfig.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr.String(),
		AvsName:                    "AlignedLayer",
		PromMetricsIpPortAddress:   c.BaseConfig.EigenMetricsIpPortAddress,
	}

	clients, err := clients.BuildAll(buildAllConfig, c.EcdsaConfig.PrivateKey, c.BaseConfig.Logger)
	if err != nil {
		return nil, err
	}

	avsRegistryReader := clients.AvsRegistryChainReader

	avsServiceBindings, err := NewAvsServiceBindings(c.BaseConfig.AlignedLayerDeploymentConfig.AlignedLayerServiceManagerAddr, c.BaseConfig.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr, c.BaseConfig.EthRpcClient, c.BaseConfig.Logger)
	if err != nil {
		return nil, err
	}

	return &AvsReader{
		AvsRegistryReader:   avsRegistryReader,
		AvsContractBindings: avsServiceBindings,
		logger:              c.BaseConfig.Logger,
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
