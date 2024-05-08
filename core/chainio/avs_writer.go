package chainio

import (
	"context"
	"fmt"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"
	"github.com/Layr-Labs/eigensdk-go/types"
	gethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/yetanotherco/aligned_layer/common"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/core/utils"
	"math/big"
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

func (w *AvsWriter) SendTask(context context.Context, provingSystemId common.ProvingSystemId,
	DAPayload servicemanager.AlignedLayerServiceManagerDAPayload, publicInput []byte,
	verificationKey []byte, quorumNumbers types.QuorumNums,
	quorumThresholdPercentages types.QuorumThresholdPercentages, fee *big.Int) (servicemanager.AlignedLayerServiceManagerTask, uint32, error) {

	txOpts := w.Signer.GetTxOpts()

	txOpts.Value = fee

	tx, err := w.AvsContractBindings.ServiceManager.CreateNewTask(
		txOpts,
		uint16(provingSystemId),
		DAPayload,
		publicInput,
		verificationKey,
		quorumNumbers.UnderlyingType(),
		quorumThresholdPercentages.UnderlyingType(),
	)
	if err != nil {
		w.logger.Error("Error assembling CreateNewTask tx", "err", err)
		return servicemanager.AlignedLayerServiceManagerTask{}, 0, err
	}

	receipt, err := utils.WaitForTransactionReceipt(w.Client, context, tx.Hash())
	if err != nil {
		return servicemanager.AlignedLayerServiceManagerTask{}, 0, err
	}

	newTaskCreatedEvent, err := w.AvsContractBindings.ServiceManager.ContractAlignedLayerServiceManagerFilterer.ParseNewTaskCreated(*receipt.Logs[0])
	if err != nil {
		return servicemanager.AlignedLayerServiceManagerTask{}, 0, err
	}
	return newTaskCreatedEvent.Task, newTaskCreatedEvent.TaskIndex, nil
}

func (w *AvsWriter) SendAggregatedResponse(ctx context.Context, task servicemanager.AlignedLayerServiceManagerTask, taskResponse servicemanager.AlignedLayerServiceManagerTaskResponse, nonSignerStakesAndSignature servicemanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*gethtypes.Receipt, error) {

	txOpts := w.Signer.GetTxOpts()

	// Don't send the transaction, just estimate the gas
	txOpts.NoSend = true

	tx, err := w.AvsContractBindings.ServiceManager.RespondToTask(txOpts, task, taskResponse, nonSignerStakesAndSignature)
	if err != nil {
		w.logger.Error("Error simulating respondToTask", "err", err)
		return nil, err
	}

	if tx.Cost().Cmp(task.Fee) > 0 {
		w.logger.Error("Gas estimate is higher than the task fee", "gas", tx.Cost(), "fee", task.Fee)

		// return error
		return nil, fmt.Errorf("gas estimate is higher than the task fee, gas: %s, fee: %s", tx.Cost(), task.Fee)
	}

	txOpts.NoSend = false
	txOpts.GasLimit = tx.Gas()
	txOpts.GasPrice = tx.GasPrice()

	tx, err = w.AvsContractBindings.ServiceManager.RespondToTask(txOpts, task, taskResponse, nonSignerStakesAndSignature)
	if err != nil {
		w.logger.Error("Error submitting SubmitTaskResponse tx while calling respondToTask", "err", err)
		return nil, err
	}

	w.logger.Info("Submitted task response to contract", "taskIndex", taskResponse.TaskIndex,
		"proofIsValid", taskResponse.ProofIsCorrect)

	receipt, err := utils.WaitForTransactionReceipt(w.Client, ctx, tx.Hash())
	if err != nil {
		return nil, err
	}

	taskRespondedEvent, err := w.AvsContractBindings.ServiceManager.ContractAlignedLayerServiceManagerFilterer.ParseTaskResponded(*receipt.Logs[0])
	if err != nil {
		return nil, err
	}

	// FIXME(marian): Dummy log to check integration with the contract
	w.logger.Infof("TASK RESPONDED EVENT: %+v", taskRespondedEvent)
	return receipt, nil
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
