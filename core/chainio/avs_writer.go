package chainio

import (
	"context"
	"fmt"
	"math/big"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/config"
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

	err = w.checkRespondToTaskFeeLimit(tx, txOpts, batchIdentifierHash, senderAddress)
	if err != nil {
		return nil, err
	}

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

func (w *AvsWriter) checkRespondToTaskFeeLimit(tx *types.Transaction, txOpts bind.TransactOpts, batchIdentifierHash [32]byte, senderAddress [20]byte) error {
	simulatedCost := new(big.Int).Mul(new(big.Int).SetUint64(tx.Gas()), tx.GasPrice())
	w.logger.Info("Simulated cost", "cost", simulatedCost)

	respondToTaskFeeLimit := big.NewInt(0)

	// Get RespondToTaskFeeLimit
	batchState, err := w.AvsContractBindings.ServiceManager.BatchesState(&bind.CallOpts{}, batchIdentifierHash)
	if err != nil {
		// Retry with fallback
		batchState, err = w.AvsContractBindings.ServiceManagerFallback.BatchesState(&bind.CallOpts{}, batchIdentifierHash)
		if err != nil {
			// Ignore and continue.
			// Would be overkill to stop the transaction if checker fails to get this value
			w.logger.Error("Failed to get batch state" ,"error", err)
			respondToTaskFeeLimit = big.NewInt(0)
		}
	}
	if err == nil {
		respondToTaskFeeLimit = batchState.RespondToTaskFeeLimit
	}

	// check SimulatedCost against RespondToTaskFeeLimit.
	if respondToTaskFeeLimit != big.NewInt(0) {
		w.logger.Info("Batch RespondToTaskFeeLimit", "RespondToTaskFeeLimit", respondToTaskFeeLimit)

		if batchState.RespondToTaskFeeLimit.Cmp(simulatedCost) < 0 {
			return fmt.Errorf("cost of transaction is higher than Batch.RespondToTaskFeeLimit")
		}
	}

	// Get batcher balance
	batcherBalance, err := w.AvsContractBindings.ServiceManager.BatchersBalances(&bind.CallOpts{}, senderAddress)
	if err != nil {
		batcherBalance, err = w.AvsContractBindings.ServiceManagerFallback.BatchersBalances(&bind.CallOpts{}, senderAddress)
		if err != nil {
			// Ignore and continue.
			w.logger.Error("Failed to get batcherBalance" ,"error", err)
		}
	}
	if err == nil {
		// Compare against simulatedCost.
		w.logger.Info("Batcher balance", "balance", batcherBalance)

		if batcherBalance.Cmp(simulatedCost) < 0 {
			return fmt.Errorf("cost of transaction is higher than Batcher balance")
		}

		// Compare against RespondToTaskFeeLimit.
		if respondToTaskFeeLimit != big.NewInt(0) {
			if batcherBalance.Cmp(respondToTaskFeeLimit) < 0 {
				return fmt.Errorf("respondToTaskFeeLimit is higher than Batcher balance")
			}
		}
	}

	// Get Agg wallet balance
	aggregatorAddress := txOpts.From
	aggregatorBalance, err := w.Client.BalanceAt(context.TODO(), aggregatorAddress, nil)
	if err != nil {
		aggregatorBalance, err = w.Client.BalanceAt(context.TODO(), aggregatorAddress, nil) // There is no fallback client?
		if err != nil {
			// Ignore and continue.
			w.logger.Error("failed to get aggregator balance: %v", err)
		}
	}
	if err == nil {
		// Compare against simulatedCost.
		w.logger.Info("Aggregator balance", "balance", aggregatorBalance)
		if aggregatorBalance.Cmp(simulatedCost) < 0 {
			return fmt.Errorf("cost of transaction is higher than Aggregator balance")
		}

		// Compare against RespondToTaskFeeLimit.
		if respondToTaskFeeLimit != big.NewInt(0) {
			if aggregatorBalance.Cmp(respondToTaskFeeLimit) < 0 {
				return fmt.Errorf("respondToTaskFeeLimit is higher than Aggregator balance")
			}
		}
	}

	return nil
}
