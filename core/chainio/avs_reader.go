package chainio

import (
	"context"
	"fmt"
	"math/big"
	"strings"

	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
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

func (r *AvsReader) IsOperatorRegistered(address ethcommon.Address) (bool, error) {
	return r.ChainReader.IsOperatorRegistered(&bind.CallOpts{}, address)
}

// Returns the latest logs starting from the given block
func (r *AvsReader) GetNotRespondedTasksFrom(fromBlock uint64) ([]servicemanager.ContractAlignedLayerServiceManagerNewBatchV3, error) {
	latestBlock, err := r.AvsContractBindings.ethClient.BlockNumber(context.Background())
	if err != nil {
		latestBlock, err = r.AvsContractBindings.ethClientFallback.BlockNumber(context.Background())
		if err != nil {
			return nil, fmt.Errorf("failed to get latest block number: %w", err)
		}
	}

	alignedLayerServiceManagerABI, err := abi.JSON(strings.NewReader(servicemanager.ContractAlignedLayerServiceManagerMetaData.ABI))
	if err != nil {
		return nil, fmt.Errorf("failed to parse ABI: %w", err)
	}

	newBatchEvent := alignedLayerServiceManagerABI.Events["NewBatchV3"]
	if newBatchEvent.ID == (ethcommon.Hash{}) {
		return nil, fmt.Errorf("NewBatch event not found in ABI")
	}

	query := ethereum.FilterQuery{
		FromBlock: big.NewInt(int64(fromBlock)),
		ToBlock:   big.NewInt(int64(latestBlock)),
		Addresses: []ethcommon.Address{r.AlignedLayerServiceManagerAddr},
		Topics:    [][]ethcommon.Hash{{newBatchEvent.ID, {}}},
	}

	logs, err := r.AvsContractBindings.ethClient.FilterLogs(context.Background(), query)
	if err != nil {
		logs, err = r.AvsContractBindings.ethClientFallback.FilterLogs(context.Background(), query)
		if err != nil {
			return nil, fmt.Errorf("failed to get logs: %w", err)
		}
	}

	var tasks []servicemanager.ContractAlignedLayerServiceManagerNewBatchV3

	for _, logEntry := range logs {
		var task servicemanager.ContractAlignedLayerServiceManagerNewBatchV3

		err := alignedLayerServiceManagerABI.UnpackIntoInterface(&task, "NewBatchV3", logEntry.Data)
		if err != nil {
			return nil, fmt.Errorf("failed to unpack log data: %w", err)
		}

		// The second topic is the batch merkle root, as it is an indexed variable in the contract
		task.BatchMerkleRoot = logEntry.Topics[1]

		// now check if its finalized or not before appending
		batchIdentifier := append(task.BatchMerkleRoot[:], task.SenderAddress[:]...)
		batchIdentifierHash := *(*[32]byte)(crypto.Keccak256(batchIdentifier))
		state, err := r.AvsContractBindings.ServiceManager.ContractAlignedLayerServiceManagerCaller.BatchesState(nil, batchIdentifierHash)

		if err != nil {
			return nil, fmt.Errorf("err while getting batch state: %w", err)
		}

		// append the task if not responded yet
		if !state.Responded {
			tasks = append(tasks, task)
		}

	}

	return tasks, nil
}
