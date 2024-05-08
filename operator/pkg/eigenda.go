package operator

import (
	"bytes"
	"context"
	"fmt"
	"github.com/Layr-Labs/eigenda/api/grpc/disperser"
	"github.com/Layr-Labs/eigenda/encoding/utils/codec"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
)

func (o *Operator) getProofByChunksFromEigenDA(newTaskCreatedLog *servicemanager.ContractAlignedLayerServiceManagerNewTaskCreated) ([]byte, error) {
	var proofChunks [][]byte
	for i, chunk := range newTaskCreatedLog.Task.DAPayload.Chunks {
		o.Logger.Infof("Getting proof chunk %d from EigenDA...", chunk.Index)
		result, err := o.getProofFromEigenDA(newTaskCreatedLog.Task.DAPayload.Chunks[i].ProofAssociatedData, newTaskCreatedLog.Task.DAPayload.Chunks[i].Index)
		if err != nil {
			o.Logger.Errorf("Could not get proof from EigenDA: %v", err)
			return nil, err
		}
		proofChunks = append(proofChunks, result)
	}
	return bytes.Join(proofChunks, nil), nil
}

func (o *Operator) getProofFromEigenDA(eigenDABatchHeaderHash []byte, eigenDABlobIndex uint64) ([]byte, error) {
	if eigenDABlobIndex > 0xFFFFFFFF {
		return nil, fmt.Errorf("blob index %d is too large", eigenDABlobIndex)
	}

	blobIndex := uint32(eigenDABlobIndex)

	ctx := context.Background()

	req := disperser.RetrieveBlobRequest{
		BatchHeaderHash: eigenDABatchHeaderHash,
		BlobIndex:       blobIndex,
	}

	blob, err := o.disperser.RetrieveBlob(ctx, &req)
	if err != nil {
		return nil, err
	}

	return codec.RemoveEmptyByteFromPaddedBytes(blob.Data), nil
}
