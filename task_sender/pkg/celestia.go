package pkg

import (
	"context"
	"github.com/celestiaorg/celestia-node/blob"
	"github.com/yetanotherco/aligned_layer/common"
	serviceManager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"log"
)

func (ts *TaskSender) PostProofOnCelestia(proof []byte) (*serviceManager.AlignedLayerServiceManagerTaskDA, error) {
	b, err := blob.NewBlobV0(ts.celestiaConfig.Namespace, proof)
	if err != nil {
		return nil, err
	}

	blobs := []*blob.Blob{b}

	log.Println("Submitting proof to Celestia...")
	height, err := ts.celestiaConfig.Client.Blob.Submit(context.Background(), blobs, 0.1) // TODO: estimate gas price
	if err != nil {
		return nil, err
	}

	taskDA := &serviceManager.AlignedLayerServiceManagerTaskDA{
		Solution: common.Celestia,
		Proof:    b.Commitment,
		Index:    height,
	}

	return taskDA, nil
}
