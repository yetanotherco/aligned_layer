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
	"github.com/yetanotherco/aligned_layer/core/utils"
)

const (
	gasBumpPercentage int = 20
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

// Sends AggregatedResponse and waits for the receipt for three blocks, if not received
// it will try again bumping the last tx gas price based on `CalculateGasPriceBump`
// This process happens indefinitely until the transaction is included.
// Note: If the rpc endpoints fail, the retry will stop, as it will infinitely try
func (w *AvsWriter) SendAggregatedResponse(batchIdentifierHash [32]byte, batchMerkleRoot [32]byte, senderAddress [20]byte, nonSignerStakesAndSignature servicemanager.IBLSSignatureCheckerNonSignerStakesAndSignature, onRetry func()) (*types.Receipt, error) {
	txOpts := *w.Signer.GetTxOpts()
	txOpts.NoSend = true // simulate the transaction
	tx, err := w.AvsContractBindings.ServiceManager.RespondToTaskV2(&txOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature, new(big.Int).SetUint64(5))

	if err != nil {
		// Retry with fallback
		tx, err = w.AvsContractBindings.ServiceManagerFallback.RespondToTaskV2(&txOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature, new(big.Int).SetUint64(5))
		if err != nil {
			return nil, fmt.Errorf("transaction simulation failed: %v", err)
		}
	}

	err = w.checkRespondToTaskFeeLimit(tx, txOpts, batchIdentifierHash, senderAddress)
	if err != nil {
		return nil, err
	}

	// Set the nonce, as we might have to replace the transaction with a higher fee
	txNonce := new(big.Int).SetUint64(tx.Nonce())
	txOpts.NoSend = false
	txOpts.Nonce = txNonce

	i := 0
	sendTransaction := func() (*types.Receipt, error) {
		if i > 0 {
			onRetry()
		}
		i++

		// get gas price
		gasPrice, err := w.Client.SuggestGasPrice(context.Background())
		if err != nil {
			// Retry with fallback
			gasPrice, err = w.ClientFallback.SuggestGasPrice(context.Background())
			if err != nil {
				return nil, fmt.Errorf("transaction simulation failed: %v", err)
			}
		}
		txOpts.GasPrice = utils.CalculateGasPriceBumpBasedOnRetry(gasPrice, gasBumpPercentage, i)

		w.logger.Infof("Sending ResponseToTask transaction with a gas price of %v", txOpts.GasPrice)
		err = w.checkRespondToTaskFeeLimit(tx, txOpts, batchIdentifierHash, senderAddress)

		if err != nil {
			return nil, connection.PermanentError{Inner: err}
		}

		tx, err = w.AvsContractBindings.ServiceManager.RespondToTaskV2(&txOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature, new(big.Int).SetUint64(i))
		if err != nil {
			// Retry with fallback
			tx, err = w.AvsContractBindings.ServiceManagerFallback.RespondToTaskV2(&txOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature, new(big.Int).SetUint64(i))
			if err != nil {
				return nil, connection.PermanentError{Inner: err}
			}
		}

		ctx, cancel := context.WithTimeout(context.Background(), time.Second*2)
		defer cancel()
		receipt, err := utils.WaitForTransactionReceipt(w.Client, ctx, tx.Hash())

		if receipt != nil {
			return receipt, nil
		}
		// if we are here, this means we have reached the timeout (after three blocks it hasn't been included)
		// so we try again by bumping the fee to make sure its included
		if err != nil {
			return nil, err
		}
		return nil, fmt.Errorf("transaction failed")
	}

	return connection.RetryWithData(sendTransaction, 1000, 2, 0)
}

func (w *AvsWriter) checkRespondToTaskFeeLimit(tx *types.Transaction, txOpts bind.TransactOpts, batchIdentifierHash [32]byte, senderAddress [20]byte) error {
	aggregatorAddress := txOpts.From
	simulatedCost := new(big.Int).Mul(new(big.Int).SetUint64(tx.Gas()), tx.GasPrice())
	w.logger.Info("Simulated cost", "cost", simulatedCost)

	// Get RespondToTaskFeeLimit
	batchState, err := w.AvsContractBindings.ServiceManager.BatchesState(&bind.CallOpts{}, batchIdentifierHash)
	if err != nil {
		// Retry with fallback
		batchState, err = w.AvsContractBindings.ServiceManagerFallback.BatchesState(&bind.CallOpts{}, batchIdentifierHash)
		if err != nil {
			// Fallback also failed
			// Proceed to check values against simulated costs
			w.logger.Error("Failed to get batch state", "error", err)
			w.logger.Info("Proceeding with simulated cost checks")
			return w.compareBalances(simulatedCost, aggregatorAddress, senderAddress)
		}
	}
	// At this point, batchState was successfully retrieved
	// Proceed to check values against RespondToTaskFeeLimit
	respondToTaskFeeLimit := batchState.RespondToTaskFeeLimit
	w.logger.Info("Batch RespondToTaskFeeLimit", "RespondToTaskFeeLimit", respondToTaskFeeLimit)

	if respondToTaskFeeLimit.Cmp(simulatedCost) < 0 {
		return fmt.Errorf("cost of transaction is higher than Batch.RespondToTaskFeeLimit")
	}

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
	// Get Agg wallet balance
	aggregatorBalance, err := w.Client.BalanceAt(ctx, aggregatorAddress, nil)
	if err != nil {
		aggregatorBalance, err = w.ClientFallback.BalanceAt(ctx, aggregatorAddress, nil)
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

func (w *AvsWriter) compareBatcherBalance(amount *big.Int, senderAddress [20]byte) error {
	// Get batcher balance
	batcherBalance, err := w.AvsContractBindings.ServiceManager.BatchersBalances(&bind.CallOpts{}, senderAddress)
	if err != nil {
		batcherBalance, err = w.AvsContractBindings.ServiceManagerFallback.BatchersBalances(&bind.CallOpts{}, senderAddress)
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
