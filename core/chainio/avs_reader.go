package chainio

import (
	"context"
	"encoding/hex"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	gethcommon "github.com/ethereum/go-ethereum/common"
	contractERC20Mock "github.com/yetanotherco/aligned_layer/contracts/bindings/ERC20Mock"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/core/types"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	sdkavsregistry "github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/logging"
)

type AvsReader struct {
	*sdkavsregistry.ChainReader
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

	avsServiceBindings, err := NewAvsServiceBindings(
		baseConfig.AlignedLayerDeploymentConfig.AlignedLayerServiceManagerAddr,
		baseConfig.AlignedLayerDeploymentConfig.AlignedLayerRegistryCoordinatorAddr,
		baseConfig.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr,
		baseConfig.EthRpcClient,
		baseConfig.EthRpcClientFallback,
		baseConfig.Logger,
	)
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
		// Retry with fallback client
		erc20Mock, err = contractERC20Mock.NewContractERC20Mock(tokenAddr, r.AvsContractBindings.ethClientFallback)
		if err != nil {
			r.logger.Error("Failed to fetch ERC20Mock contract", "err", err)
		}
	}
	return erc20Mock, nil
}

func (r *AvsReader) IsOperatorRegistered(address gethcommon.Address) (bool, error) {
	return r.ChainReader.IsOperatorRegistered(&bind.CallOpts{}, address)
}

func (r *AvsReader) GetOperators() (map[eigentypes.OperatorId]types.OperatorData, error) {
	blockNumber, err := r.AvsContractBindings.ethClient.BlockNumber(context.Background())
	if err != nil {
		return nil, err
	}
	quorumNumbers := []byte{0}

	operatorsByQuorum, err := r.AvsContractBindings.OperatorStateRetriever.GetOperatorState(
		&bind.CallOpts{},
		r.AvsContractBindings.ContractBindings.RegistryCoordinatorAddr,
		quorumNumbers,
		uint32(blockNumber), // Converted to uint32 because the contract expects it but ethClient.BlockNumber returns int64
	)
	if err != nil {
		return nil, err
	}

	operators := make(map[eigentypes.OperatorId]types.OperatorData)
	for _, operator := range operatorsByQuorum[0] { // We only use one quorum (0x00)
		operators[operator.OperatorId] = types.OperatorData{
			Address: "0x" + hex.EncodeToString(operator.Operator[:]),
			Id:      "0x" + hex.EncodeToString(operator.OperatorId[:]),
			Name:    "dummy name", // TODO get the name from Metadata
			Stake:   operator.Stake,
		}
	}
	return operators, nil
}
