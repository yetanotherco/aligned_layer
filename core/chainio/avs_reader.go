package chainio

import (
	"context"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	ethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	contractERC20Mock "github.com/yetanotherco/aligned_layer/contracts/bindings/ERC20Mock"
	"github.com/yetanotherco/aligned_layer/core/config"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	sdkavsregistry "github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/logging"
)

type AvsReader struct {
	*sdkavsregistry.ChainReader
	AvsContractBindings            *AvsServiceBindings
	AlignedLayerServiceManagerAddr ethcommon.Address
	logger                         logging.Logger
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

	avsServiceBindings, err := NewAvsServiceBindings(baseConfig.AlignedLayerDeploymentConfig.AlignedLayerServiceManagerAddr, baseConfig.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr, baseConfig.EthRpcClient, baseConfig.EthRpcClientFallback, baseConfig.Logger)
	if err != nil {
		return nil, err
	}

	return &AvsReader{
		ChainReader:                    chainReader,
		AvsContractBindings:            avsServiceBindings,
		AlignedLayerServiceManagerAddr: baseConfig.AlignedLayerDeploymentConfig.AlignedLayerServiceManagerAddr,
		logger:                         baseConfig.Logger,
	}, nil
}

func (r *AvsReader) GetErc20Mock(tokenAddr ethcommon.Address) (*contractERC20Mock.ContractERC20Mock, error) {
	erc20Mock, err := contractERC20Mock.NewContractERC20Mock(tokenAddr, &r.AvsContractBindings.ethClient)
	if err != nil {
		// Retry with fallback client
		erc20Mock, err = contractERC20Mock.NewContractERC20Mock(tokenAddr, &r.AvsContractBindings.ethClientFallback)
		if err != nil {
			r.logger.Error("Failed to fetch ERC20Mock contract", "err", err)
		}
	}
	return erc20Mock, nil
}

func (r *AvsReader) IsOperatorRegistered(address ethcommon.Address) (bool, error) {
	return r.ChainReader.IsOperatorRegistered(&bind.CallOpts{}, address)
}

// Returns all the "NewBatchV3" logs that have not been responded starting from the given block number
func (r *AvsReader) GetNotRespondedTasksFrom(fromBlock uint64) ([]servicemanager.ContractAlignedLayerServiceManagerNewBatchV3, error) {
	logs, err := r.AvsContractBindings.ServiceManager.FilterNewBatchV3(&bind.FilterOpts{Start: fromBlock, End: nil, Context: context.Background()}, nil)

	if err != nil {
		return nil, err
	}

	var tasks []servicemanager.ContractAlignedLayerServiceManagerNewBatchV3

	for logs.Next() {
		task, err := r.AvsContractBindings.ServiceManager.ParseNewBatchV3(logs.Event.Raw)
		if err != nil {
			return nil, err
		}

		// now check if its finalized or not before appending
		batchIdentifier := append(task.BatchMerkleRoot[:], task.SenderAddress[:]...)
		batchIdentifierHash := *(*[32]byte)(crypto.Keccak256(batchIdentifier))
		state, err := r.AvsContractBindings.ServiceManager.ContractAlignedLayerServiceManagerCaller.BatchesState(nil, batchIdentifierHash)

		if err != nil {
			return nil, err
		}

		// append the task if not responded yet
		if !state.Responded {
			tasks = append(tasks, *task)
		}
	}

	return tasks, nil
}
