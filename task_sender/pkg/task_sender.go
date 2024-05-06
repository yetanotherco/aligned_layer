package pkg

import (
	"context"
	"encoding/hex"
	"github.com/Layr-Labs/eigenda/api/grpc/disperser"
	"github.com/Layr-Labs/eigenda/encoding/utils/codec"
	"github.com/Layr-Labs/eigensdk-go/types"
	"github.com/yetanotherco/aligned_layer/common"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"log"
	"math/big"
	"time"
)

type Task struct {
	ProvingSystem              common.ProvingSystemId
	EigenDABatchHeaderHash     []byte
	EigenDABlobIndex           uint32
	PublicInput                []byte
	VerificationKey            []byte
	QuorumNumbers              types.QuorumNums
	QuorumThresholdPercentages types.QuorumThresholdPercentages
	Fee                        *big.Int
}

func NewTask(provingSystemId common.ProvingSystemId, eigenDABatchHeaderHash []byte, eigenDABlobIndex uint32, publicInput []byte, verificationKey []byte, quorumNumbers types.QuorumNums, quorumThresholdPercentages types.QuorumThresholdPercentages, fee *big.Int) *Task {
	return &Task{
		ProvingSystem:              provingSystemId,
		EigenDABatchHeaderHash:     eigenDABatchHeaderHash,
		EigenDABlobIndex:           eigenDABlobIndex,
		PublicInput:                publicInput,
		VerificationKey:            verificationKey,
		QuorumNumbers:              quorumNumbers,
		QuorumThresholdPercentages: quorumThresholdPercentages,
		Fee:                        fee,
	}
}

type TaskSender struct {
	avsWriter *chainio.AvsWriter
	disperser disperser.DisperserClient
}

const RETRY_INTERVAL = 1 * time.Second

func NewTaskSender(avsWriter *chainio.AvsWriter, disperser disperser.DisperserClient) *TaskSender {
	return &TaskSender{
		avsWriter: avsWriter,
		disperser: disperser,
	}
}

func (ts *TaskSender) SendTask(task *Task) error {
	log.Println("Sending task...")
	_, index, err := ts.avsWriter.SendTask(
		context.Background(),
		task.ProvingSystem,
		task.EigenDABatchHeaderHash,
		task.EigenDABlobIndex,
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

func (ts *TaskSender) PostProofOnEigenDA(proof []byte) (*disperser.BlobStatusReply, error) {
	data := codec.ConvertByPaddingEmptyByte(proof)
	disperseBlobReq := &disperser.DisperseBlobRequest{
		Data: data,
	}

	log.Println("Posting proof on EigenDA...")
	disperseBlob, err := ts.disperser.DisperseBlob(context.Background(), disperseBlobReq)
	if err != nil {
		return nil, err
	}

	log.Println("Proof posted successfully. Request ID:", hex.EncodeToString(disperseBlob.RequestId))

	log.Println("Waiting for confirmation...")

	getBlobStatusReq := &disperser.BlobStatusRequest{
		RequestId: disperseBlob.RequestId,
	}

	status, err := ts.disperser.GetBlobStatus(context.Background(), getBlobStatusReq)
	if err != nil {
		return nil, err
	}

	for status.Status == disperser.BlobStatus_PROCESSING {
		time.Sleep(RETRY_INTERVAL)
		status, err = ts.disperser.GetBlobStatus(context.Background(), getBlobStatusReq)
		if err != nil {
			return nil, err
		}
	}

	return status, nil
}
