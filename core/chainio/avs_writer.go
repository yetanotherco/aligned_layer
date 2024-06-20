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
		receipt, err := utils.WaitForTransactionReceipt(w.Client, ctx, txHash, 6, 5*time.Second)
		if err == nil && receipt != nil {
			if receipt.Status == 0 {
				w.logger.Warn("Transaction failed", "txHash", txHash.String(), "batchMerkleRoot", hex.EncodeToString(batchMerkleRoot[:]))
				return receipt, nil
			}
			return receipt, nil
		}

		w.logger.Info("Receipt not found. Bumping gas price", "txHash", txHash.String(),
			"batchMerkleRoot", hex.EncodeToString(batchMerkleRoot[:]))

		txOpts := *w.Signer.GetTxOpts()
		txOpts.Nonce = txNonce

		// Increase the gas base fee and gas tip cap by 15% * retry number
		incrementPercentage := LowFeeIncrementPercentage * (i + 1)
		if incrementPercentage > 200 {
			incrementPercentage = 200
		}

		gasTipCap, err := w.Client.SuggestGasTipCap(ctx)
		if err != nil {
			w.logger.Error("Failed to get suggested gas tip cap", "err", err)
			return nil, err
		}

		gasTipCap.Mul(gasTipCap, big.NewInt(int64(incrementPercentage)))
		gasTipCap.Div(gasTipCap, big.NewInt(100))

		w.logger.Info("Sending bump gas price replacement transaction",
			"batchMerkleRoot", hex.EncodeToString(batchMerkleRoot[:]),
			"gasTipCap", gasTipCap.String())

		tx, err := w.AvsContractBindings.ServiceManager.RespondToTask(&txOpts, batchMerkleRoot, nonSignerStakesAndSignature)
		if err != nil {
			w.logger.Error("Failed to send bump gas price replacement transaction", "err", err)
			return nil, err
		}

		txHash = tx.Hash()
		txNonce = new(big.Int).SetUint64(tx.Nonce())
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
