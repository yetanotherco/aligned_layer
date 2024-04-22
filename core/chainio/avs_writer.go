package chainio

import (
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	gethcommon "github.com/ethereum/go-ethereum/common"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/chainio/txmgr"
	logging "github.com/Layr-Labs/eigensdk-go/logging"
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
	return NewAvsWriter(c.Signer, c.AlignedLayerServiceManagerAddr, c.BlsOperatorStateRetrieverAddr, c.EthHttpClient, c.Logger)
}

func NewAvsWriter(signer signer.Signer, txMgr txmgr.TxManager, serviceManagerAddr, blsOperatorStateRetrieverAddr gethcommon.Address, ethHttpClient eth.Client, logger logging.Logger) (*AvsWriter, error) {
	avsServiceBindings, err := NewAvsServiceBindings(serviceManagerAddr, blsOperatorStateRetrieverAddr, ethHttpClient, logger)
	if err != nil {
		logger.Error("Failed to create contract bindings", "err", err)
		return nil, err
	}

	blsRegistryCoordinatorAddr, err := avsServiceBindings.ServiceManager.RegistryCoordinator(&bind.CallOpts{})
	if err != nil {
		return nil, err
	}
	// stakeRegistryAddr, err := avsServiceBindings.ServiceManager.StakeRegistry(&bind.CallOpts{})
	// if err != nil {
	// 	return nil, err
	// }

	avsRegistryWriter, err := avsregistry.BuildAvsRegistryChainWriter(blsRegistryCoordinatorAddr, blsOperatorStateRetrieverAddr, logger, ethHttpClient)

	// blsPubkeyRegistryAddr, err := avsServiceBindings.ServiceManager.BlsPubkeyRegistry(&bind.CallOpts{})
	// if err != nil {
	// 	return nil, err
	// }
	// avsRegistryContractClient, err := sdkclients.NewAvsRegistryContractsChainClient(
	// 	blsRegistryCoordinatorAddr, blsOperatorStateRetrieverAddr, stakeRegistryAddr, blsPubkeyRegistryAddr, ethHttpClient, logger,
	// )
	// if err != nil {
	// 	return nil, err
	// }
	avsRegistryWriter, err := avsregistry.NewAvsRegistryWriter(avsRegistryContractClient, logger, signer, ethHttpClient)
	if err != nil {
		return nil, err
	}

	return &AvsWriter{
		AvsRegistryWriter:   avsRegistryWriter,
		AvsContractBindings: avsServiceBindings,
		logger:              logger,
		Signer:              signer,
		client:              ethHttpClient,
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
