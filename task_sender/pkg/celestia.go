package pkg

import (
	"context"
	"github.com/celestiaorg/celestia-node/blob"
	"github.com/yetanotherco/aligned_layer/common"
	serviceManager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"log"
)

func (ts *TaskSender) PostProofOnCelestia(proof []byte) (*serviceManager.AlignedLayerServiceManagerDAPayload, error) {
	proofChunks := SplitIntoChunks(proof, 100) // TODO: Actual max value

	daChunks := make([]serviceManager.AlignedLayerServiceManagerDAPayloadChunk, len(proofChunks))

	for idx, proofChunk := range proofChunks {

		b, err := blob.NewBlobV0(ts.celestiaConfig.Namespace, proofChunk)
		if err != nil {
			return nil, err
		}

		blobs := []*blob.Blob{b}

		log.Printf("Submitting proof chunk %d to Celestia...", idx)

		height, err := ts.celestiaConfig.Client.Blob.Submit(context.Background(), blobs, blob.DefaultGasPrice())
		if err != nil {
			return nil, err
		}

		daChunks[idx].ProofAssociatedData = b.Commitment
		daChunks[idx].Index = height
	}

	DAPayload := &serviceManager.AlignedLayerServiceManagerDAPayload{
		Solution: common.Celestia,
		Chunks:   daChunks,
	}

	return DAPayload, nil
}
