package chainio

import (
	"context"
	"github.com/yetanotherco/aligned_layer/common"
	"time"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/config"
)

type AvsWriter struct {
	avsregistry.AvsRegistryWriter
	AvsContractBindings *AvsServiceBindings
	logger              logging.Logger
	Signer              signer.Signer
	Client              eth.Client
}

func NewAvsWriterFromConfig(c *config.Config) (*AvsWriter, error) {

	buildAllConfig := clients.BuildAllConfig{
		EthHttpUrl:                 c.EthRpcUrl,
		EthWsUrl:                   c.EthWsUrl,
		RegistryCoordinatorAddr:    c.AlignedLayerRegistryCoordinatorAddr.String(),
		OperatorStateRetrieverAddr: c.AlignedLayerOperatorStateRetrieverAddr.String(),
		AvsName:                    "AlignedLayer",
		PromMetricsIpPortAddress:   c.EigenMetricsIpPortAddress,
	}

	clients, err := clients.BuildAll(buildAllConfig, c.EcdsaPrivateKey, c.Logger)
	if err != nil {
		c.Logger.Error("Cannot create clients", "err", err)
		return nil, err
	}

	if err != nil {
		c.Logger.Error("Cannot build signer config", "err", err)
		return nil, err
	}

	avsServiceBindings, err := NewAvsServiceBindings(c.AlignedLayerServiceManagerAddr, c.AlignedLayerOperatorStateRetrieverAddr, c.EthHttpClient, c.Logger)
	if err != nil {
		c.Logger.Error("Cannot create avsServiceBindings", "err", err)
		return nil, err
	}

	if err != nil {
		c.Logger.Error("Cannot create avs service bindings", "err", err)
		return nil, err
	}

	privateKeySigner, err := signer.NewPrivateKeySigner(c.EcdsaPrivateKey, c.ChainId)

	if err != nil {
		c.Logger.Error("Cannot create signer", "err", err)
		return nil, err
	}

	avsRegistryWriter := clients.AvsRegistryChainWriter

	return &AvsWriter{
		AvsRegistryWriter:   avsRegistryWriter,
		AvsContractBindings: avsServiceBindings,
		logger:              c.Logger,
		Signer:              privateKeySigner,
		Client:              c.EthHttpClient,
	}, nil
}

func (w *AvsWriter) SendTask(context context.Context, verificationSystemId common.SystemVerificationId, proof []byte, publicInput []byte) (servicemanager.AlignedLayerServiceManagerTask, uint64, error) {
	txOpts := w.Signer.GetTxOpts()
	tx, err := w.AvsContractBindings.ServiceManager.CreateNewTask(
		txOpts,
		uint16(verificationSystemId),
		proof,
		publicInput,
	)
	if err != nil {
		w.logger.Error("Error assembling CreateNewTask tx", "err", err)
		return servicemanager.AlignedLayerServiceManagerTask{}, 0, err
	}
	// TODO wait for transaction receipt. ethClient does not have this method
	// EigenSDK has a method called WaitForTransactionReceipt in InstrumentedEthClient
	// But is needs telemetry to work
	// https://github.com/Layr-Labs/eigensdk-go/blob/master/chainio/clients/eth/instrumented_client.go
	//receipt := avsWriter.Client.WaitForTransactionReceipt(context.Background(), tx.Hash())
	time.Sleep(2 * time.Second)

	receipt, err := w.Client.TransactionReceipt(context, tx.Hash())
	if err != nil {
		return servicemanager.AlignedLayerServiceManagerTask{}, 0, err
	}
	newTaskCreatedEvent, err := w.AvsContractBindings.ServiceManager.ContractAlignedLayerServiceManagerFilterer.ParseNewTaskCreated(*receipt.Logs[0])
	if err != nil {
		return servicemanager.AlignedLayerServiceManagerTask{}, 0, err

	}
	return newTaskCreatedEvent.Task, newTaskCreatedEvent.TaskIndex, nil
}

// func (w *AvsWriter) SendAggregatedResponse(ctx context.Context, task cstaskmanager.IAlignedLayerTaskManagerTask, taskResponse cstaskmanager.IAlignedLayerTaskManagerTaskResponse, nonSignerStakesAndSignature cstaskmanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Receipt, error) {
// 	txOpts := w.Signer.GetTxOpts()
// 	tx, err := w.AvsContractBindings.TaskManager.RespondToTask(txOpts, task, taskResponse, nonSignerStakesAndSignature)
// 	if err != nil {
// 		w.logger.Error("Error submitting SubmitTaskResponse tx while calling respondToTask", "err", err)
// 		return nil, err
// 	}
// 	receipt := w.client.WaitForTransactionReceipt(ctx, tx.Hash())
// 	return receipt, nil
// }

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
