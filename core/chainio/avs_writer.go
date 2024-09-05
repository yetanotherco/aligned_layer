package chainio

import (
	"fmt"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/config"
	"math/big"
	"github.com/ethereum/go-ethereum/core/types"
)

type AvsWriter struct {
	*avsregistry.ChainWriter
	AvsContractBindings *AvsServiceBindings
	logger              logging.Logger
	Signer              signer.Signer
	Client              eth.Client
}

func NewAvsWriterFromConfig(baseConfig *config.BaseConfig, ecdsaConfig *config.EcdsaConfig) (*AvsWriter, error) {

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
		baseConfig.Logger.Error("Cannot build signer config", "err", err)
		return nil, err
	}

	avsServiceBindings, err := NewAvsServiceBindings(baseConfig.AlignedLayerDeploymentConfig.AlignedLayerServiceManagerAddr, baseConfig.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr, baseConfig.EthRpcClient, baseConfig.EthRpcClientFallback, baseConfig.Logger)

	if err != nil {
		baseConfig.Logger.Error("Cannot create avs service bindings", "err", err)
		return nil, err
	}

	privateKeySigner, err := signer.NewPrivateKeySigner(ecdsaConfig.PrivateKey, baseConfig.ChainId)
	if err != nil {
		baseConfig.Logger.Error("Cannot create signer", "err", err)
		return nil, err
	}

	chainWriter := clients.AvsRegistryChainWriter

	return &AvsWriter{
		ChainWriter:         chainWriter,
		AvsContractBindings: avsServiceBindings,
		logger:              baseConfig.Logger,
		Signer:              privateKeySigner,
		Client:              baseConfig.EthRpcClient,
	}, nil
}

func (w *AvsWriter) SendAggregatedResponse(batchIdentifierHash [32]byte, batchMerkleRoot [32]byte, senderAddress [20]byte, nonSignerStakesAndSignature servicemanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*common.Hash, error) {
	txOpts := *w.Signer.GetTxOpts()
	txOpts.NoSend = true // simulate the transaction
	tx, err := w.AvsContractBindings.ServiceManager.RespondToTaskV2(&txOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature)
	if err != nil {
		// Retry with fallback
		tx, err = w.AvsContractBindings.ServiceManagerFallback.RespondToTaskV2(&txOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature)
		if err != nil {
			return nil, err
		}
	}

	// simulatedCost := new(big.Int).Mul(new(big.Int).SetUint64(tx.Gas()), tx.GasPrice())
	// w.logger.Info("Simulated cost", "cost", simulatedCost)

	// batchState, err := w.AvsContractBindings.ServiceManager.BatchesState(&bind.CallOpts{}, batchIdentifierHash)
	// if err != nil {
	// 	return nil, err
	// }
	// w.logger.Info("Batch MaxFeeAllowedToRespond", "MaxFeeAllowedToRespond", batchState.MaxFeeAllowedToRespond)

	// if batchState.MaxFeeAllowedToRespond.Cmp(simulatedCost) < 0 {
	// 	return nil, fmt.Errorf("cost of transaction is higher than Batch.MaxFeeAllowedToRespond")
	// }

	// // TODO wip check batcher balance against maxFeeAllowedToRespond.
	// batcherBalance, err := w.AvsContractBindings.ServiceManager.BatchersBalances(&bind.CallOpts{}, senderAddress)
	// if err != nil {
	// 	return nil, err
	// }
	// w.logger.Info("Batcher balance", "balance", batcherBalance)

	// if batcherBalance.Cmp(simulatedCost) < 0 {
	// 	return nil, fmt.Errorf("cost of transaction is higher than Batcher balance")
	// }
	// TODO check Agg wallet balance against maxFeeAllowedToRespond.
	// Should I make this check?
	// aggregatorAddress := txOpts.From

	// Send the transaction
	txOpts.NoSend = false
	txOpts.GasLimit = tx.Gas() * 110 / 100 // Add 10% to the gas limit
	tx, err = w.AvsContractBindings.ServiceManager.RespondToTaskV2(&txOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature)
	if err != nil {
		// Retry with fallback
		tx, err = w.AvsContractBindings.ServiceManagerFallback.RespondToTaskV2(&txOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature)
		if err != nil {
			return nil, err
		}
	}

	txHash := tx.Hash()

	return &txHash, nil
}

func (w *AvsWriter) checkRespondToTaskFeeLimit(tx *types.Transaction, batchIdentifierHash [32]byte, senderAddress [20]byte) (error){
	simulatedCost := new(big.Int).Mul(new(big.Int).SetUint64(tx.Gas()), tx.GasPrice())
	w.logger.Info("Simulated cost", "cost", simulatedCost)

	batchState, err := w.AvsContractBindings.ServiceManager.BatchesState(&bind.CallOpts{}, batchIdentifierHash)
	if err != nil {
		return err
	}
	w.logger.Info("Batch MaxFeeAllowedToRespond", "MaxFeeAllowedToRespond", batchState.MaxFeeAllowedToRespond)

	if batchState.MaxFeeAllowedToRespond.Cmp(simulatedCost) < 0 {
		return fmt.Errorf("cost of transaction is higher than Batch.MaxFeeAllowedToRespond")
	}

	// TODO wip check batcher balance against maxFeeAllowedToRespond.
	batcherBalance, err := w.AvsContractBindings.ServiceManager.BatchersBalances(&bind.CallOpts{}, senderAddress)
	if err != nil {
		return err
	}
	w.logger.Info("Batcher balance", "balance", batcherBalance)

	if batcherBalance.Cmp(simulatedCost) < 0 {
		return fmt.Errorf("cost of transaction is higher than Batcher balance")
	}
	return nil
}
