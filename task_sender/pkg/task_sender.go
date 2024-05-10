package pkg

import (
	"context"
	"crypto/ecdsa"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/types"
	"github.com/yetanotherco/aligned_layer/common"
	serviceManager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
	"log"
	"math/big"
	"time"
)

type Task struct {
	ProvingSystem              common.ProvingSystemId
	DAPayload                  serviceManager.AlignedLayerServiceManagerDAPayload
	PublicInput                []byte
	VerificationKey            []byte
	QuorumNumbers              types.QuorumNums
	QuorumThresholdPercentages types.QuorumThresholdPercentages
	Fee                        *big.Int
}

func NewTask(provingSystemId common.ProvingSystemId, DAPayload serviceManager.AlignedLayerServiceManagerDAPayload, publicInput []byte, verificationKey []byte, quorumNumbers types.QuorumNums, quorumThresholdPercentages types.QuorumThresholdPercentages, fee *big.Int) *Task {
	return &Task{
		ProvingSystem:              provingSystemId,
		DAPayload:                  DAPayload,
		PublicInput:                publicInput,
		VerificationKey:            verificationKey,
		QuorumNumbers:              quorumNumbers,
		QuorumThresholdPercentages: quorumThresholdPercentages,
		Fee:                        fee,
	}
}

type TaskSender struct {
	avsWriter      *chainio.AvsWriter
	EthRpcClient   eth.Client
	EcdsaPrivKey   *ecdsa.PrivateKey
	eigenDAConfig  *config.EigenDADisperserConfig
	celestiaConfig *config.CelestiaConfig
	blobsConfig    *config.BlobsConfig
}

const RetryInterval = 1 * time.Second

func NewTaskSender(config *config.TaskSenderConfig, avsWriter *chainio.AvsWriter) *TaskSender {
	return &TaskSender{
		avsWriter:      avsWriter,
		EthRpcClient:   config.BaseConfig.EthRpcClient,
		EcdsaPrivKey:   config.EcdsaConfig.PrivateKey,
		eigenDAConfig:  config.EigenDADisperserConfig,
		celestiaConfig: config.CelestiaConfig,
		blobsConfig:    config.BlobsConfig,
	}
}

func (ts *TaskSender) SendTask(task *Task) error {
	log.Println("Sending task...")
	_, index, err := ts.avsWriter.SendTask(
		context.Background(),
		task.ProvingSystem,
		task.DAPayload,
		task.PublicInput,
		task.VerificationKey,
		task.QuorumNumbers,
		task.QuorumThresholdPercentages,
		task.Fee,
	)
	if err != nil {
		return err
	}
	log.Println("Task sent successfully. Task index:", index)
	return nil
}
