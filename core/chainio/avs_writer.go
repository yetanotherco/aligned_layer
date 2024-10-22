package chainio

import (
	"context"
	"fmt"
	"math/big"
	"time"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	connection "github.com/yetanotherco/aligned_layer/core"
	"github.com/yetanotherco/aligned_layer/core/config"
)

type AvsWriter struct {
	*avsregistry.ChainWriter
	AvsContractBindings *AvsServiceBindings
	logger              logging.Logger
	Signer              signer.Signer
	Client              eth.InstrumentedClient
	ClientFallback      eth.InstrumentedClient
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
		ClientFallback:      baseConfig.EthRpcClientFallback,
	}, nil
}

func (w *AvsWriter) SendAggregatedResponse(batchIdentifierHash [32]byte, batchMerkleRoot [32]byte, senderAddress [20]byte, nonSignerStakesAndSignature servicemanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*common.Hash, error) {
	txOpts := *w.Signer.GetTxOpts()
	txOpts.NoSend = true // simulate the transaction
	respondToTaskV2_func := func() (*types.Transaction, error) {
		return w.AvsContractBindings.ServiceManager.RespondToTaskV2(&txOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature)
	}
	tx, err := connection.RetryWithData(respondToTaskV2_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
	if err != nil {
		// Retry with fallback
		tx, err = connection.RetryWithData(respondToTaskV2_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
		if err != nil {
			return nil, fmt.Errorf("transaction simulation failed: %v", err)
		}
	}

	err = w.checkRespondToTaskFeeLimitRetryable(tx, txOpts, batchIdentifierHash, senderAddress)
	if err != nil {
		return nil, err
	}

	// Send the transaction
	txOpts.NoSend = false
	txOpts.GasLimit = tx.Gas() * 110 / 100 // Add 10% to the gas limit
	respondToTaskV2_func = func() (*types.Transaction, error) {
		return w.AvsContractBindings.ServiceManager.RespondToTaskV2(&txOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature)
	}
	tx, err = connection.RetryWithData(respondToTaskV2_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
	if err != nil {
		// Retry with fallback
		tx, err = connection.RetryWithData(respondToTaskV2_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
		if err != nil {
			return nil, err
		}
	}

	txHash := tx.Hash()

	return &txHash, nil
}

func (w *AvsWriter) checkRespondToTaskFeeLimitRetryable(tx *types.Transaction, txOpts bind.TransactOpts, batchIdentifierHash [32]byte, senderAddress [20]byte) error {
	aggregatorAddress := txOpts.From
	simulatedCost := new(big.Int).Mul(new(big.Int).SetUint64(tx.Gas()), tx.GasPrice())
	w.logger.Info("Simulated cost", "cost", simulatedCost)

	// Get RespondToTaskFeeLimit
	//TODO: make this a public type
	batchesState_func := func() (*struct {
		TaskCreatedBlock      uint32
		Responded             bool
		RespondToTaskFeeLimit *big.Int
	}, error) {
		state, err := w.AvsContractBindings.ServiceManager.BatchesState(&bind.CallOpts{}, batchIdentifierHash)
		return &state, err
	}
	batchState, err := connection.RetryWithData(batchesState_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
	if err != nil {
		// Retry with fallback
		batchState, err = connection.RetryWithData(batchesState_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
		if err != nil {
			// Fallback also failed
			// Proceed to check values against simulated costs
			w.logger.Error("Failed to get batch state", "error", err)
			w.logger.Info("Proceeding with simulated cost checks")
			return w.compareBalancesRetryable(simulatedCost, aggregatorAddress, senderAddress)
		}
	}
	// At this point, batchState was successfully retrieved
	// Proceed to check values against RespondToTaskFeeLimit
	respondToTaskFeeLimit := batchState.RespondToTaskFeeLimit
	w.logger.Info("Batch RespondToTaskFeeLimit", "RespondToTaskFeeLimit", respondToTaskFeeLimit)

	if respondToTaskFeeLimit.Cmp(simulatedCost) < 0 {
		return fmt.Errorf("cost of transaction is higher than Batch.RespondToTaskFeeLimit")
	}

	return w.compareBalancesRetryable(respondToTaskFeeLimit, aggregatorAddress, senderAddress)
}

func (w *AvsWriter) compareBalancesRetryable(amount *big.Int, aggregatorAddress common.Address, senderAddress [20]byte) error {
	if err := w.compareAggregatorBalanceRetryable(amount, aggregatorAddress); err != nil {
		return err
	}
	if err := w.compareBatcherBalanceRetryable(amount, senderAddress); err != nil {
		return err
	}
	return nil
}

func (w *AvsWriter) compareAggregatorBalanceRetryable(amount *big.Int, aggregatorAddress common.Address) error {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	// Get Agg wallet balance
	balanceAt_func := func() (*big.Int, error) {
		return w.Client.BalanceAt(ctx, aggregatorAddress, nil)
	}
	aggregatorBalance, err := connection.RetryWithData(balanceAt_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
	if err != nil {
		aggregatorBalance, err = connection.RetryWithData(balanceAt_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
		if err != nil {
			// Ignore and continue.
			w.logger.Error("failed to get aggregator balance: %v", err)
			return nil
		}
	}
	w.logger.Info("Aggregator balance", "balance", aggregatorBalance)
	if aggregatorBalance.Cmp(amount) < 0 {
		return fmt.Errorf("cost is higher than Aggregator balance")
	}
	return nil
}

func (w *AvsWriter) compareBatcherBalanceRetryable(amount *big.Int, senderAddress [20]byte) error {
	// Get batcher balance
	batcherBalances_func := func() (*big.Int, error) {
		return w.AvsContractBindings.ServiceManager.BatchersBalances(&bind.CallOpts{}, senderAddress)
	}
	batcherBalance, err := connection.RetryWithData(batcherBalances_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
	if err != nil {
		batcherBalance, err = connection.RetryWithData(batcherBalances_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
		if err != nil {
			// Ignore and continue.
			w.logger.Error("Failed to get batcherBalance", "error", err)
			return nil
		}
	}
	w.logger.Info("Batcher balance", "balance", batcherBalance)
	if batcherBalance.Cmp(amount) < 0 {
		return fmt.Errorf("cost is higher than Batcher balance")
	}
	return nil
}
