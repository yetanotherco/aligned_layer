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
	sdkavsregistry.AvsRegistryReader
	AvsContractBindings *AvsServiceBindings
	logger              logging.Logger
}

const blockRange = 100

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

	avsRegistryReader := clients.AvsRegistryChainReader

	avsServiceBindings, err := NewAvsServiceBindings(baseConfig.AlignedLayerDeploymentConfig.AlignedLayerServiceManagerAddr, baseConfig.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr, baseConfig.EthRpcClient, baseConfig.Logger)
	if err != nil {
		return nil, err
	}

	return &AvsReader{
		AvsRegistryReader:   avsRegistryReader,
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
	return r.AvsRegistryReader.IsOperatorRegistered(&bind.CallOpts{}, address)
}

// func (r *AvsReader) GetNewTaskCreated(taskIndex uint32) (*contractAlignedLayerServiceManager.ContractAlignedLayerServiceManagerNewTaskCreated, error) {
// 	latestBlock, err := r.AvsContractBindings.ethClient.BlockNumber(context.Background())
// 	if err != nil {
// 		r.logger.Error("Failed to get latest block number", "err", err)
// 		return nil, err
// 	}
// 	startBlock := uint64(0)
// 	if latestBlock > blockRange {
// 		startBlock = latestBlock - blockRange
// 	}
// 	filterOpts := bind.FilterOpts{
// 		Start: startBlock,
// 	}

// 	itr, err := r.AvsContractBindings.ServiceManager.FilterNewTaskCreated(&filterOpts, []uint32{taskIndex})
// 	if err != nil {
// 		return nil, err
// 	}

// 	itr.Next()
// 	event := itr.Event
// 	err = itr.Close()
// 	if err != nil {
// 		return nil, err
// 	}

// 	if event != nil && event.TaskIndex == taskIndex {
// 		return event, nil
// 	}

// 	return nil, fmt.Errorf("task index %d not found", taskIndex)
// }
