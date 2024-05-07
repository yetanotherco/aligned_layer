package pkg

import (
	"context"
	"encoding/hex"
	"github.com/Layr-Labs/eigenda/api/grpc/disperser"
	"github.com/Layr-Labs/eigenda/encoding/utils/codec"
	"github.com/yetanotherco/aligned_layer/common"
	serviceManager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"log"
	"time"
)

func (ts *TaskSender) PostProofOnEigenDA(proof []byte) (*serviceManager.AlignedLayerServiceManagerTaskDA, error) {
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
		time.Sleep(RetryInterval)
		status, err = ts.disperser.GetBlobStatus(context.Background(), getBlobStatusReq)
		if err != nil {
			return nil, err
		}
	}

	verificationProof := status.GetInfo().GetBlobVerificationProof()

	taskDA := &serviceManager.AlignedLayerServiceManagerTaskDA{
		Solution: common.EigenDA,
		Proof:    verificationProof.GetBatchMetadata().GetBatchHeaderHash(),
		Index:    uint64(verificationProof.GetBlobIndex()),
	}

	return taskDA, nil
}
