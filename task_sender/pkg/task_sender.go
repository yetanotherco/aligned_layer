package pkg

import (
	"context"
	"crypto/tls"
	"github.com/Layr-Labs/eigenda/api/grpc/disperser"
	"github.com/Layr-Labs/eigenda/encoding/utils/codec"
	"github.com/Layr-Labs/eigensdk-go/types"
	"github.com/yetanotherco/aligned_layer/common"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
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
}

const RETRY_INTERVAL = 1 * time.Second

func NewTaskSender(avsWriter *chainio.AvsWriter) *TaskSender {
	return &TaskSender{
		avsWriter: avsWriter,
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

func PostProofOnEigenDA(proof []byte) (*disperser.BlobStatusReply, error) {
	// TODO: Disperser client should be in core & instantiated once

	config := &tls.Config{}
	credential := credentials.NewTLS(config)

	// TODO: dispeser-holesky.eigenda.xyz should be in config
	clientConn, err := grpc.NewClient("disperser-holesky.eigenda.xyz:443", grpc.WithTransportCredentials(credential))
	if err != nil {
		return nil, err
	}
	disperserClient := disperser.NewDisperserClient(clientConn)

	data := codec.ConvertByPaddingEmptyByte(proof)
	disperseBlobReq := &disperser.DisperseBlobRequest{
		Data:      data,
		AccountId: "f39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
	}

	log.Println("Posting proof on EigenDA...")
	disperseBlob, err := disperserClient.DisperseBlob(context.Background(), disperseBlobReq)
	if err != nil {
		return nil, err
	}

	log.Println("Proof posted successfully. Request ID:", disperseBlob.RequestId, "waiting for confirmation...")

	getBlobStatusReq := &disperser.BlobStatusRequest{
		RequestId: disperseBlob.RequestId,
	}

	status, err := disperserClient.GetBlobStatus(context.Background(), getBlobStatusReq)
	if err != nil {
		return nil, err
	}

	for status.Status == disperser.BlobStatus_PROCESSING {
		time.Sleep(RETRY_INTERVAL)
		status, err = disperserClient.GetBlobStatus(context.Background(), getBlobStatusReq)
		if err != nil {
			return nil, err
		}
	}

	return status, nil
}
