package chainio

import (
	"fmt"
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"
)

type AvsWriter struct {
	avsregistry.AvsRegistryWriter
	AvsContractBindings *AvsServiceBindings
	logger              logging.Logger
	Signer              signer.Signer
	client              eth.Client
}

// NOTE(marian): The initialization of the AVS writer is hardcoded, but should be loaded from a
// configuration file.
// The hardcoded values are:
//   - logger
//   - EthHttpUrl
//   - EthWsUrl
//   - RegistryCoordinatorAddr
//   - OperatorStateRetrieverAddr
//   - alignedLayerServiceManagerAddr
//   - ecdsaPrivateKey
//   - chainId

// The following function signature was the one in the aligned_layer_testnet repo:
// func NewAvsWriterFromConfig(c *config.Config) (*AvsWriter, error) {
func NewAvsWriterFromConfig() (*AvsWriter, error) {
	logger, err := sdklogging.NewZapLogger("development")
	if err != nil {
		fmt.Println("Could not initialize logger")
	}
	buildAllConfig := clients.BuildAllConfig{
		EthHttpUrl:                 "http://localhost:8545",
		EthWsUrl:                   "ws://localhost:8545",
		RegistryCoordinatorAddr:    "0x67d269191c92Caf3cD7723F116c85e6E9bf55933",
		OperatorStateRetrieverAddr: "0x9d4454B023096f34B160D6B654540c56A1F81688",
		AvsName:                    "AlignedLayer",
		PromMetricsIpPortAddress:   ":9090",
	}
	ecdsaPrivateKeyString := "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
	ecdsaPrivateKey, err := crypto.HexToECDSA(ecdsaPrivateKeyString)
	if err != nil {
		logger.Errorf("Cannot parse ecdsa private key", "err", err)
	}

	clients, err := clients.BuildAll(buildAllConfig, ecdsaPrivateKey, logger)
	alignedLayerServiceManagerAddr := common.HexToAddress("0xc5a5C42992dECbae36851359345FE25997F5C42d")

	ethHttpClient, err := eth.NewClient(buildAllConfig.EthHttpUrl)
	if err != nil {
		panic(err)
	}

	operatorStateRetrieverAddr := common.HexToAddress(buildAllConfig.OperatorStateRetrieverAddr)
	avsServiceBindings, err := NewAvsServiceBindings(alignedLayerServiceManagerAddr, operatorStateRetrieverAddr, ethHttpClient, logger)

	chainId := big.NewInt(31337)

	privateKeySigner, err := signer.NewPrivateKeySigner(ecdsaPrivateKey, chainId)
	if err != nil {
		logger.Error("Cannot create signer", "err", err)
		return nil, err
	}

	avsRegistryWriter := clients.AvsRegistryChainWriter

	return &AvsWriter{
		AvsRegistryWriter:   avsRegistryWriter,
		AvsContractBindings: avsServiceBindings,
		logger:              logger,
		Signer:              privateKeySigner,
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
