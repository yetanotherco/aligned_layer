package chainio

import (
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	gethcommon "github.com/ethereum/go-ethereum/common"
	contractERC20Mock "github.com/yetanotherco/aligned_layer/contracts/bindings/ERC20Mock"
	"github.com/yetanotherco/aligned_layer/core/config"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	sdkavsregistry "github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/logging"
)

type AvsReader struct {
	ChainReader         *sdkavsregistry.ChainReader
	AvsContractBindings *AvsServiceBindings
	logger              logging.Logger
}

func NewAvsReaderFromConfig(baseConfig *config.BaseConfig, ecdsaConfig *config.EcdsaConfig) (*AvsReader, error) {

	buildAllConfig := clients.BuildAllConfig{
		EthHttpUrl:                 baseConfig.EthRpcUrl,
		EthWsUrl:                   baseConfig.EthWsUrl,
		RegistryCoordinatorAddr:    baseConfig.AlignedLayerDeploymentConfig.AlignedLayerRegistryCoordinatorAddr.String(),
		OperatorStateRetrieverAddr: baseConfig.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr.String(),
		AvsName:                    "AlignedLayer",
		PromMetricsIpPortAddress:   baseConfig.EigenMetricsIpPortAddress,
	}

	clients, err := clients.BuildAll(buildAllConfig, ecdsaConfig.PrivateKey, baseConfig.Logger)
	if err != nil {
		return nil, err
	}

	chainReader := clients.AvsRegistryChainReader

	avsServiceBindings, err := NewAvsServiceBindings(baseConfig.AlignedLayerDeploymentConfig.AlignedLayerServiceManagerAddr, baseConfig.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr, baseConfig.EthRpcClient, baseConfig.Logger)
	if err != nil {
		return nil, err
	}

	return &AvsReader{
		ChainReader:         chainReader,
		AvsContractBindings: avsServiceBindings,
		logger:              baseConfig.Logger,
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

func (r *AvsReader) IsOperatorRegistered(address gethcommon.Address) (bool, error) {
	return r.ChainReader.IsOperatorRegistered(&bind.CallOpts{}, address)
}
