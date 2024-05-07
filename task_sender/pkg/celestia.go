package pkg

import (
	"context"
	"github.com/celestiaorg/celestia-node/blob"
	"github.com/celestiaorg/celestia-node/share"
	serviceManager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
)

func (ts *TaskSender) PostProofOnCelestia(proof []byte) (*serviceManager.AlignedLayerServiceManagerTaskDA, error) {
	ns, err := share.NewBlobNamespaceV0([]byte("Aligned")) // TODO: should be a constant / saved when client is created
	if err != nil {
		return nil, err
	}
	b, err := blob.NewBlobV0(ns, proof)
	if err != nil {
		return nil, err
	}

	blobs := []*blob.Blob{b}
	height, err := ts.celestiaClient.Blob.Submit(context.Background(), blobs, 0.1) // TODO: estimate gas price
	if err != nil {
		return nil, err
	}

	taskDA := &serviceManager.AlignedLayerServiceManagerTaskDA{
		Solution:   0,
		Commitment: b.Commitment,
		Index:      height,
	}

	return taskDA, nil
}
