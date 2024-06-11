package chainio

import (
	"context"
	"fmt"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"
	gethcommon "github.com/ethereum/go-ethereum/common"
	gethtypes "github.com/ethereum/go-ethereum/core/types"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/core/utils"
	"math/big"
	"time"
)

const (
	MaxRetries                  = 25
	SleepTime                   = 5 * time.Second
	IncrementPercentage         = 25
	IncrementPercentageInterval = 20 * time.Second
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

	_, err = utils.WaitForTransactionReceipt(w.Client, context, tx.Hash())
	if err != nil {
		return err
	}

	return nil
}

func (w *AvsWriter) SendAggregatedResponse(ctx context.Context, batchMerkleRoot [32]byte, nonSignerStakesAndSignature servicemanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*gethtypes.Receipt, error) {
	txOpts := *w.Signer.GetTxOpts()
	txOpts.NoSend = true // simulate the transaction
	tx, err := w.AvsContractBindings.ServiceManager.RespondToTask(&txOpts, batchMerkleRoot, nonSignerStakesAndSignature)
	if err != nil {
		return nil, err
	}

	// Send the transaction
	txOpts.NoSend = false
	txOpts.GasLimit = tx.Gas() * 110 / 100 // Add 10% to the gas limit
	tx, err = w.AvsContractBindings.ServiceManager.RespondToTask(&txOpts, batchMerkleRoot, nonSignerStakesAndSignature)
	if err != nil {
		return nil, err
	}

	txNonce := big.NewInt(int64(tx.Nonce()))
	receipt, err := w.WaitForTransactionReceiptWithIncreasingTip(ctx, tx.Hash(), txNonce, batchMerkleRoot, nonSignerStakesAndSignature)
	w.logger.Info("Transaction receipt:", "receipt", receipt)
	if err != nil {
		return nil, err
	}

	return receipt, nil
}

func (w *AvsWriter) WaitForTransactionReceiptWithIncreasingTip(ctx context.Context, txHash gethcommon.Hash, txNonce *big.Int, batchMerkleRoot [32]byte, nonSignerStakesAndSignature servicemanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*gethtypes.Receipt, error) {
	currentSleepTime := 0 * time.Second

	for i := 0; i < MaxRetries; i++ {
		receipt, err := w.Client.TransactionReceipt(ctx, txHash)

		if err == nil {
			return receipt, nil
		}

		currentSleepTime += SleepTime
		time.Sleep(SleepTime)

		// If incrementPercentageInterval elapses, increase the gas limit and gas tip cap
		if currentSleepTime%IncrementPercentageInterval == 0 {
			// Simulate the transaction to get the gas limit again
			txOpts := *w.Signer.GetTxOpts()
			txOpts.NoSend = true
			tx, err := w.AvsContractBindings.ServiceManager.RespondToTask(&txOpts, batchMerkleRoot, nonSignerStakesAndSignature)
			if err != nil {
				return nil, err
			}

			// Use the same nonce as the original transaction
			txOpts.Nonce = txNonce

			// Add 10% to the gas limit
			txOpts.GasLimit = tx.Gas() * 110 / 100

			// Increase the gas tip cap by 10%
			newGasTipCap := new(big.Int).Mul(big.NewInt(int64(IncrementPercentage+100)), tx.GasTipCap())
			newGasTipCap.Div(newGasTipCap, big.NewInt(100))
			txOpts.GasTipCap = newGasTipCap

			// Submit the transaction with the new gas tip cap
			txOpts.NoSend = false
			tx, err = w.AvsContractBindings.ServiceManager.RespondToTask(&txOpts, batchMerkleRoot, nonSignerStakesAndSignature)
			if err != nil {
				return nil, err
			}

			// Update the transaction hash
			txHash = tx.Hash()
		}
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
