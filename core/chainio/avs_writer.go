package chainio

import (
	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"
	"github.com/yetanotherco/aligned_layer/core/config"
)

type AvsWriter struct {
	avsregistry.AvsRegistryWriter
	AvsContractBindings *AvsServiceBindings
	logger              logging.Logger
	Signer              signer.Signer
	client              eth.Client
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

	avsServiceBindings, err := NewAvsServiceBindings(c.AlignedLayerServiceManagerAddr, c.AlignedLayerOperatorStateRetrieverAddr, c.EthHttpClient, c.Logger)

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
		client:              c.EthHttpClient,
	}, nil
}

// returns the tx receipt, as well as the task index (which it gets from parsing the tx receipt logs)
// func (w *AvsWriter) SendNewTaskVerifyProof(ctx context.Context, proof []byte, pubInput []byte, verifierId common.VerifierId, quorumThresholdPercentage uint32, quorumNumbers []byte) (cstaskmanager.IAlignedLayerTaskManagerTask, uint32, error) {
// 	txOpts := w.Signer.GetTxOpts()
// 	tx, err := w.AvsContractBindings.TaskManager.CreateNewTask(txOpts, proof, pubInput, uint16(verifierId), quorumThresholdPercentage, quorumNumbers)
// 	if err != nil {
// 		w.logger.Errorf("Error assembling CreateNewTask tx")
// 		return cstaskmanager.IAlignedLayerTaskManagerTask{}, 0, err
// 	}
// 	receipt := w.client.WaitForTransactionReceipt(ctx, tx.Hash())
// 	newTaskCreatedEvent, err := w.AvsContractBindings.TaskManager.ContractAlignedLayerTaskManagerFilterer.ParseNewTaskCreated(*receipt.Logs[0])
// 	if err != nil {
// 		w.logger.Error("Aggregator failed to parse new task created event", "err", err)
// 		return cstaskmanager.IAlignedLayerTaskManagerTask{}, 0, err
// 	}
// 	return newTaskCreatedEvent.Task, newTaskCreatedEvent.TaskIndex, nil
// }

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
