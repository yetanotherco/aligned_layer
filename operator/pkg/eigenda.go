package operator

import (
	"context"
	"fmt"
	"github.com/Layr-Labs/eigenda/api/grpc/disperser"
	"github.com/Layr-Labs/eigenda/encoding/utils/codec"
)

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
