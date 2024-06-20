package chainio

import (
	"context"
	"encoding/hex"
	"fmt"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"
	"github.com/ethereum/go-ethereum/common"
	gethtypes "github.com/ethereum/go-ethereum/core/types"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/core/utils"
	"math/big"
	"time"
)

const (
	LowFeeMaxRetries          = 25
	LowFeeSleepTime           = 25 * time.Second
	LowFeeIncrementPercentage = 15
)

type AvsWriter struct {
	avsregistry.AvsRegistryWriter
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

	avsServiceBindings, err := NewAvsServiceBindings(baseConfig.AlignedLayerDeploymentConfig.AlignedLayerServiceManagerAddr, baseConfig.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr, baseConfig.EthRpcClient, baseConfig.Logger)

	if err != nil {
		baseConfig.Logger.Error("Cannot create avs service bindings", "err", err)
		return nil, err
	}

	privateKeySigner, err := signer.NewPrivateKeySigner(ecdsaConfig.PrivateKey, baseConfig.ChainId)
	if err != nil {
		baseConfig.Logger.Error("Cannot create signer", "err", err)
		return nil, err
	}

	avsRegistryWriter := clients.AvsRegistryChainWriter

	return &AvsWriter{
		AvsRegistryWriter:   avsRegistryWriter,
		AvsContractBindings: avsServiceBindings,
		logger:              baseConfig.Logger,
		Signer:              privateKeySigner,
		Client:              baseConfig.EthRpcClient,
	}, nil
}

func (w *AvsWriter) SendTask(context context.Context, batchMerkleRoot [32]byte, batchDataPointer string) error {

	txOpts := w.Signer.GetTxOpts()

	tx, err := w.AvsContractBindings.ServiceManager.CreateNewTask(
		txOpts,
		batchMerkleRoot,
		batchDataPointer,
	)
	if err != nil {
		w.logger.Error("Error assembling CreateNewTask tx", "err", err)
		return err
	}

	_, err = utils.WaitForTransactionReceipt(w.Client, context, tx.Hash(), 25, 5*time.Second)
	if err != nil {
		return err
	}

	return nil
}

func (w *AvsWriter) SendAggregatedResponse(batchMerkleRoot [32]byte, nonSignerStakesAndSignature servicemanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*common.Hash, *big.Int, error) {
	txOpts := *w.Signer.GetTxOpts()
	txOpts.NoSend = true // simulate the transaction
	tx, err := w.AvsContractBindings.ServiceManager.RespondToTask(&txOpts, batchMerkleRoot, nonSignerStakesAndSignature)
	if err != nil {
		return nil, nil, err
	}

	// Send the transaction
	txOpts.NoSend = false
	txOpts.GasLimit = tx.Gas() * 110 / 100 // Add 10% to the gas limit
	tx, err = w.AvsContractBindings.ServiceManager.RespondToTask(&txOpts, batchMerkleRoot, nonSignerStakesAndSignature)
	if err != nil {
		return nil, nil, err
	}

	txHash := tx.Hash()
	txNonce := new(big.Int).SetUint64(tx.Nonce())

	return &txHash, txNonce, nil
}

func (w *AvsWriter) WaitForTransactionReceiptWithIncreasingTip(ctx context.Context, txHash common.Hash, txNonce *big.Int, batchMerkleRoot [32]byte, nonSignerStakesAndSignature servicemanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*gethtypes.Receipt, error) {
	for i := 0; i < LowFeeMaxRetries; i++ {
		time.Sleep(LowFeeSleepTime)
		// Attempt to get the transaction receipt
		receipt, err := w.Client.TransactionReceipt(ctx, txHash)
		if err == nil && receipt != nil {
			return receipt, nil
		}

		// Simulate the transaction to get the gas limit and gas tip cap again
		txOpts := *w.Signer.GetTxOpts()
		txOpts.NoSend = true

		// Set the nonce to the original value (replacement transaction)
		txOpts.Nonce = txNonce

		tx, err := w.AvsContractBindings.ServiceManager.RespondToTask(&txOpts, batchMerkleRoot, nonSignerStakesAndSignature)
		if err != nil {
			return nil, err
		}

		w.logger.Info("Bumping gas price for", "txHash", txHash.String(),
			"batchMerkleRoot", hex.EncodeToString(batchMerkleRoot[:]))

		// Increase the gas price
		incrementPercentage := LowFeeIncrementPercentage + i*LowFeeIncrementPercentage
		if incrementPercentage > 100 {
			incrementPercentage = 100
		}

		newGasPrice := new(big.Int).Mul(big.NewInt(int64(incrementPercentage+100)), tx.GasPrice())
		newGasPrice.Div(newGasPrice, big.NewInt(100))

		txOpts.GasPrice = newGasPrice

		// Submit the transaction with the new gas price cap
		txOpts.NoSend = false
		tx, err = w.AvsContractBindings.ServiceManager.RespondToTask(&txOpts, batchMerkleRoot, nonSignerStakesAndSignature)
		if err != nil {
			return nil, err
		}

		if txNonce.Uint64() != tx.Nonce() {
			return nil, fmt.Errorf("tx nonce mismatch after bumping gas price: expected %d, got %d", txNonce.Uint64(), tx.Nonce())
		}
		w.logger.Info("New tx hash after bumping gas price", "txHash", tx.Hash().String())

		// Update the transaction hash for the next retry
		txHash = tx.Hash()
	}

	return nil, fmt.Errorf("transaction receipt not found for txHash: %s", txHash.String())
}

// func (w *AvsWriter) RaiseChallenge(
// 	ctx context.Context,
// 	task cstaskmanager.IAlignedLayerTaskManagerTask,
// 	taskResponse cstaskmanager.IAlignedLayerTaskManagerTaskResponse,
// 	taskResponseMetadata cstaskmanager.IAlignedLayerTaskManagerTaskResponseMetadata,
// 	pubkeysOfNonSigningOperators []cstaskmanager.BN254G1Point,
// ) (*types.Receipt, error) {
// 	txOpts := w.Signer.GetTxOpts()
// 	tx, err := w.AvsContractBindings.TaskManager.RaiseAndResolveChallenge(txOpts, task, taskResponse, taskResponseMetadata, pubkeysOfNonSigningOperators)
// 	if err != nil {
// 		w.logger.Errorf("Error assembling RaiseChallenge tx")
// 		return nil, err
// 	}
// 	receipt := w.client.WaitForTransactionReceipt(ctx, tx.Hash())
// 	return receipt, nil
// }
