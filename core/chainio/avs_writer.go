package chainio

import (
	"context"
	"encoding/hex"
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
	retry "github.com/yetanotherco/aligned_layer/core"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/core/utils"
	"github.com/yetanotherco/aligned_layer/metrics"
)

type AvsWriter struct {
	*avsregistry.ChainWriter
	AvsContractBindings *AvsServiceBindings
	logger              logging.Logger
	Signer              signer.Signer
	Client              eth.InstrumentedClient
	ClientFallback      eth.InstrumentedClient
	metrics             *metrics.Metrics
}

func NewAvsWriterFromConfig(baseConfig *config.BaseConfig, ecdsaConfig *config.EcdsaConfig, metrics *metrics.Metrics) (*AvsWriter, error) {

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
		metrics:             metrics,
	}, nil
}

// SendAggregatedResponse continuously sends a RespondToTask transaction until it is included in the blockchain.
// This function:
//  1. Simulates the transaction to calculate the nonce and initial gas price without broadcasting it.
//  2. Repeatedly attempts to send the transaction, bumping the gas price after `timeToWaitBeforeBump` has passed.
//  3. Monitors for the receipt of previously sent transactions or checks the state to confirm if the response
//     has already been processed (e.g., by another transaction).
//  4. Validates that the aggregator and batcher have sufficient balance to cover transaction costs before sending.
//
// Returns:
//   - A transaction receipt if the transaction is successfully included in the blockchain.
//   - If no receipt is found, but the batch state indicates the response has already been processed, it exits
//     without an error (returning `nil, nil`).
//   - An error if the process encounters a fatal issue (e.g., permanent failure in verifying balances or state).
func (w *AvsWriter) SendAggregatedResponse(batchIdentifierHash [32]byte, batchMerkleRoot [32]byte, senderAddress [20]byte, nonSignerStakesAndSignature servicemanager.IBLSSignatureCheckerNonSignerStakesAndSignature, gasBumpPercentage uint, gasBumpIncrementalPercentage uint, gasBumpPercentageLimit uint, timeToWaitBeforeBump time.Duration, metrics *metrics.Metrics, onSetGasPrice func(*big.Int)) (*types.Receipt, error) {
	txOpts := *w.Signer.GetTxOpts()
	txOpts.NoSend = true // simulate the transaction
	simTx, err := w.RespondToTaskV2Retryable(&txOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature, retry.SendToChainRetryParams())
	if err != nil {
		return nil, err
	}

	// Set the nonce, as we might have to replace the transaction with a higher gas price
	txNonce := big.NewInt(int64(simTx.Nonce()))
	txOpts.Nonce = txNonce
	txOpts.GasPrice = nil
	txOpts.NoSend = false
	i := 0

	var sentTxs []*types.Transaction

	batchMerkleRootHashString := hex.EncodeToString(batchMerkleRoot[:])

	respondToTaskV2Func := func() (*types.Receipt, error) {
		gasPrice, err := utils.GetGasPriceRetryable(w.Client, w.ClientFallback, retry.NetworkRetryParams())
		if err != nil {
			return nil, err
		}

		// if txOpts.GasPrice wasn't previously set use the fetched gasPrice
		// this should happen on the first iteration only
		var previousTxGasPrice *big.Int
		if txOpts.GasPrice == nil {
			previousTxGasPrice = gasPrice
		} else {
			previousTxGasPrice = txOpts.GasPrice
		}

		// in order to avoid replacement transaction underpriced
		// the bumped gas price has to be at least 10% higher than the previous one.
		minimumGasPriceBump := utils.CalculateGasPriceBumpBasedOnRetry(previousTxGasPrice, 10, 0, gasBumpPercentageLimit, 0)
		suggestedBumpedGasPrice := utils.CalculateGasPriceBumpBasedOnRetry(
			gasPrice,
			gasBumpPercentage,
			gasBumpIncrementalPercentage,
			gasBumpPercentageLimit,
			i,
		)
		// check the new gas price is sufficiently bumped.
		// if the suggested bump does not meet the minimum threshold, use a fallback calculation to slightly increment the previous gas price.
		if suggestedBumpedGasPrice.Cmp(minimumGasPriceBump) > 0 {
			txOpts.GasPrice = suggestedBumpedGasPrice
		} else {
			txOpts.GasPrice = minimumGasPriceBump
		}

		onSetGasPrice(txOpts.GasPrice)

		if i > 0 {
			w.logger.Infof("Trying to get old sent transaction receipt before sending a new transaction", "merkle root", batchMerkleRootHashString)
			for _, tx := range sentTxs {
				receipt, _ := w.Client.TransactionReceipt(context.Background(), tx.Hash())
				if receipt == nil {
					receipt, _ = w.ClientFallback.TransactionReceipt(context.Background(), tx.Hash())
					if receipt != nil {
						w.updateAggregatorGasCostMetrics(receipt, batchIdentifierHash)
						return receipt, nil
					}
				}
			}
			w.logger.Infof("Receipts for old transactions not found, will check if the batch state has been responded", "merkle root", batchMerkleRootHashString)
			batchState, _ := w.BatchesStateRetryable(&bind.CallOpts{}, batchIdentifierHash, retry.NetworkRetryParams())
			if batchState.Responded {
				w.logger.Infof("Batch state has been already responded", "merkle root", batchMerkleRootHashString)
				return nil, nil
			}
			w.logger.Infof("Batch state has not been responded yet, will send a new tx", "merkle root", batchMerkleRootHashString)

			metrics.IncBumpedGasPriceForAggregatedResponse()
		}

		// We compare both Aggregator funds and Batcher balance in Aligned against respondToTaskFeeLimit
		// Both are required to have some balance, more details inside the function
		err = w.checkAggAndBatcherHaveEnoughBalance(simTx, txOpts, batchIdentifierHash, senderAddress)
		if err != nil {
			w.logger.Errorf("Permanent error when checking aggregator and batcher balances, err %v", err, "merkle root", batchMerkleRootHashString)
			return nil, retry.PermanentError{Inner: err}
		}

		w.logger.Infof("Sending RespondToTask transaction with a gas price of %v", txOpts.GasPrice, "merkle root", batchMerkleRootHashString)
		realTx, err := w.RespondToTaskV2Retryable(&txOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature, retry.SendToChainRetryParams())
		if err != nil {
			w.logger.Errorf("Respond to task transaction err, %v", err, "merkle root", batchMerkleRootHashString)
			return nil, err
		}
		sentTxs = append(sentTxs, realTx)

		w.logger.Infof("Transaction sent, waiting for receipt", "merkle root", batchMerkleRootHashString)
		receipt, err := utils.WaitForTransactionReceiptRetryable(w.Client, w.ClientFallback, realTx.Hash(), retry.WaitForTxRetryParams(timeToWaitBeforeBump))
		if receipt != nil {
			w.updateAggregatorGasCostMetrics(receipt, batchIdentifierHash)
			return receipt, nil
		}

		// if we are here, it means we have reached the receipt waiting timeout
		// we increment the i here to add an incremental percentage to increase the odds of being included in the next blocks
		i++

		w.logger.Infof("RespondToTask receipt waiting timeout has passed, will try again...", "merkle_root", batchMerkleRootHashString)
		if err != nil {
			return nil, err
		}
		return nil, fmt.Errorf("transaction failed")
	}

	// This just retries the bump of a fee in case of a timeout
	// The wait is done before on WaitForTransactionReceiptRetryable, and all the functions are retriable,
	// so this retry doesn't need to wait more time
	return retry.RetryWithData(respondToTaskV2Func, retry.RespondToTaskV2())
}

// Calculates the transaction cost from the receipt and updates the total amount paid by the aggregator metric
// Then, it compares that tx cost with the batcher respondToTaskFeeLimit.
// If the tx cost was higher, it means the aggregator has paid the difference for the batcher (txCost - respondToTaskFeeLimit) and so metrics are updated accordingly.
func (w *AvsWriter) updateAggregatorGasCostMetrics(receipt *types.Receipt, batchIdentifierHash [32]byte) {
	batchState, err := w.BatchesStateRetryable(&bind.CallOpts{}, batchIdentifierHash, retry.NetworkRetryParams())
	if err != nil {
		return
	}
	respondToTaskFeeLimit := batchState.RespondToTaskFeeLimit

	txCost := new(big.Int).Mul(big.NewInt(int64(receipt.GasUsed)), receipt.EffectiveGasPrice)

	txCostInEth := utils.WeiToEth(txCost)
	w.metrics.AddAggregatorGasCostPaidTotal(txCostInEth)

	if respondToTaskFeeLimit.Cmp(txCost) < 0 {
		aggregatorDifferencePaid := new(big.Int).Sub(txCost, respondToTaskFeeLimit)
		aggregatorDifferencePaidInEth := utils.WeiToEth(aggregatorDifferencePaid)
		w.metrics.AddAggregatorGasPaidForBatcher(aggregatorDifferencePaidInEth)
		w.metrics.IncAggregatorPaidForBatcher()
		w.logger.Warnf("cost of transaction was higher than Batch.RespondToTaskFeeLimit, aggregator has paid the for the difference, aprox: %vethers", aggregatorDifferencePaidInEth)
	}
}

func (w *AvsWriter) checkAggAndBatcherHaveEnoughBalance(tx *types.Transaction, txOpts bind.TransactOpts, batchIdentifierHash [32]byte, senderAddress [20]byte) error {
	w.logger.Info("Checking if aggregator and batcher have enough balance for the transaction")
	aggregatorAddress := txOpts.From
	txGasAsBigInt := new(big.Int).SetUint64(tx.Gas())
	txGasPrice := txOpts.GasPrice
	txCost := new(big.Int).Mul(txGasAsBigInt, txGasPrice)
	w.logger.Info("Transaction cost", "cost", txCost)

	batchState, err := w.BatchesStateRetryable(&bind.CallOpts{}, batchIdentifierHash, retry.NetworkRetryParams())
	if err != nil {
		w.logger.Error("Failed to get batch state", "error", err)
		w.logger.Info("Proceeding to check balances against transaction cost")
		return w.compareBalances(txCost, aggregatorAddress, senderAddress)
	}
	respondToTaskFeeLimit := batchState.RespondToTaskFeeLimit
	w.logger.Info("Checking balance against Batch RespondToTaskFeeLimit", "RespondToTaskFeeLimit", respondToTaskFeeLimit)
	// Note: we compare both Aggregator funds and Batcher balance in Aligned against respondToTaskFeeLimit
	// Batcher will pay up to respondToTaskFeeLimit, for this he needs that amount of funds in Aligned
	// Aggregator will pay any extra cost, for this he needs at least respondToTaskFeeLimit in his balance
	return w.compareBalances(respondToTaskFeeLimit, aggregatorAddress, senderAddress)
}

func (w *AvsWriter) compareBalances(amount *big.Int, aggregatorAddress common.Address, senderAddress [20]byte) error {
	if err := w.compareAggregatorBalance(amount, aggregatorAddress); err != nil {
		return err
	}
	if err := w.compareBatcherBalance(amount, senderAddress); err != nil {
		return err
	}
	return nil
}

func (w *AvsWriter) compareAggregatorBalance(amount *big.Int, aggregatorAddress common.Address) error {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	aggregatorBalance, err := w.BalanceAtRetryable(ctx, aggregatorAddress, nil, retry.NetworkRetryParams())
	if err != nil {
		// Ignore and continue.
		w.logger.Error("failed to get aggregator balance: %v", err)
		return nil
	}
	w.logger.Info("Aggregator balance", "balance", aggregatorBalance)
	if aggregatorBalance.Cmp(amount) < 0 {
		return fmt.Errorf("cost is higher than Aggregator balance")
	}
	return nil
}

func (w *AvsWriter) compareBatcherBalance(amount *big.Int, senderAddress [20]byte) error {
	// Get batcher balance
	batcherBalance, err := w.BatcherBalancesRetryable(&bind.CallOpts{}, senderAddress, retry.NetworkRetryParams())
	if err != nil {
		// Ignore and continue.
		w.logger.Error("Failed to get batcherBalance", "error", err)
		return nil
	}
	w.logger.Info("Batcher balance", "balance", batcherBalance)
	if batcherBalance.Cmp(amount) < 0 {
		return fmt.Errorf("cost is higher than Batcher balance")
	}
	return nil
}
