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

// EigenDAMaxSize 2 MB
const EigenDAMaxSize = 2 * 1024 * 1024

func (ts *TaskSender) PostProofOnEigenDA(proof []byte) (*serviceManager.AlignedLayerServiceManagerDAPayload, error) {
	proofChunks := SplitIntoChunks(proof, EigenDAMaxSize)

	requestIds := make([][]byte, len(proofChunks))
	daChunks := make([]serviceManager.AlignedLayerServiceManagerDAPayloadChunk, len(proofChunks))

	for idx, proofChunk := range proofChunks {
		data := codec.ConvertByPaddingEmptyByte(proofChunk)
		disperseBlobReq := &disperser.DisperseBlobRequest{
			Data: data,
		}

		log.Printf("Posting proof chunk %d of %d on EigenDA...", idx+1, len(proofChunks))
		disperseBlob, err := ts.eigenDAConfig.Disperser.DisperseBlob(context.Background(), disperseBlobReq)
		if err != nil {
			return nil, err
		}

		log.Printf("Proof chunk %d posted successfully. Request ID: %s", idx+1, hex.EncodeToString(disperseBlob.RequestId))

		requestIds[idx] = disperseBlob.RequestId
	}

	log.Println("Waiting for confirmation of proof chunks...")

	reqConfirmed := make([]bool, len(proofChunks))
	confirmedCount := 0

	for confirmedCount < len(proofChunks) {
		for idx, requestId := range requestIds {
			if reqConfirmed[idx] {
				continue
			}

			getBlobStatusReq := &disperser.BlobStatusRequest{
				RequestId: requestId,
			}

			status, err := ts.eigenDAConfig.Disperser.GetBlobStatus(context.Background(), getBlobStatusReq)
			if err != nil {
				return nil, err
			}

			if status.Status == disperser.BlobStatus_CONFIRMED {
				confirmedCount++
				reqConfirmed[idx] = true

				verificationProof := status.GetInfo().GetBlobVerificationProof()

				daChunks[idx].ProofAssociatedData = verificationProof.GetBatchMetadata().GetBatchHeaderHash()
				daChunks[idx].Index = uint64(verificationProof.GetBlobIndex())

				log.Printf("Proof chunk %d confirmed successfully, %d to go", idx+1, len(proofChunks)-confirmedCount)
			}

			time.Sleep(100 * time.Millisecond)
		}

		time.Sleep(2 * time.Second)
	}

	DAPayload := &serviceManager.AlignedLayerServiceManagerDAPayload{
		Solution: common.EigenDA,
		Chunks:   daChunks,
	}

	return DAPayload, nil
}
