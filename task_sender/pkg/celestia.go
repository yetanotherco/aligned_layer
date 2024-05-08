package pkg

import (
	"context"
	"github.com/celestiaorg/celestia-node/blob"
	"github.com/yetanotherco/aligned_layer/common"
	serviceManager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"log"
)

func (ts *TaskSender) PostProofOnCelestia(proof []byte) (*serviceManager.AlignedLayerServiceManagerDAPayload, error) {
	b, err := blob.NewBlobV0(ts.celestiaConfig.Namespace, proof)
	if err != nil {
		return nil, err
	}

	blobs := []*blob.Blob{b}

	log.Println("Submitting proof to Celestia...")

	height, err := ts.celestiaConfig.Client.Blob.Submit(context.Background(), blobs, blob.DefaultGasPrice())
	if err != nil {
		return nil, err
	}

	// TODO: Actually split into chunks
	chunk := serviceManager.AlignedLayerServiceManagerDAPayloadChunk{
		ProofAssociatedData: b.Commitment,
		Index:               height,
	}

	DAPayload := &serviceManager.AlignedLayerServiceManagerDAPayload{
		Solution: common.Celestia,
		Chunks:   []serviceManager.AlignedLayerServiceManagerDAPayloadChunk{chunk},
	}

	return DAPayload, nil
}
