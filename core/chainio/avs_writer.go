package chainio

import (
	"context"
	"fmt"
	common2 "github.com/yetanotherco/aligned_layer/common"
	"math/big"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
)

type AvsWriter struct {
	avsregistry.AvsRegistryWriter
	AvsContractBindings *AvsServiceBindings
	logger              logging.Logger
	Signer              signer.Signer
	Client              eth.Client
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
		Client:              ethHttpClient,
	}, nil
}

func (w *AvsWriter) SendTask(context context.Context, verificationSystemId common2.SystemVerificationId, proof []byte, publicInput []byte) (servicemanager.AlignedLayerServiceManagerTask, uint32, error) {
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
