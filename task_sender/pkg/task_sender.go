package pkg

import (
	"context"
	"log"
	"math/big"
	"time"

	"github.com/Layr-Labs/eigensdk-go/types"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
)

type BatchProofVerificationTask struct {
	VerificationData           []servicemanager.AlignedLayerServiceManagerProofVerificationData
	QuorumNumbers              types.QuorumNums
	QuorumThresholdPercentages types.QuorumThresholdPercentages
	Fee                        *big.Int
}

func NewTask(proofVerificationData []servicemanager.AlignedLayerServiceManagerProofVerificationData, quorumNumbers types.QuorumNums, quorumThresholdPercentages types.QuorumThresholdPercentages, fee *big.Int) *BatchProofVerificationTask {
	return &BatchProofVerificationTask{
		VerificationData:           proofVerificationData,
		QuorumNumbers:              quorumNumbers,
		QuorumThresholdPercentages: quorumThresholdPercentages,
		Fee:                        fee,
	}
}

type TaskSender struct {
	avsWriter      *chainio.AvsWriter
	eigenDAConfig  *config.EigenDADisperserConfig
	celestiaConfig *config.CelestiaConfig
}

const RetryInterval = 1 * time.Second

func NewTaskSender(config *config.TaskSenderConfig, avsWriter *chainio.AvsWriter) *TaskSender {
	return &TaskSender{
		avsWriter:      avsWriter,
		eigenDAConfig:  config.EigenDADisperserConfig,
		celestiaConfig: config.CelestiaConfig,
	}
}

func (ts *TaskSender) SendTask(task *BatchProofVerificationTask) error {
	log.Println("Sending task...")
	_, index, err := ts.avsWriter.SendTask(
		context.Background(),
		task.VerificationData,
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
