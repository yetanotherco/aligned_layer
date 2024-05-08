package operator

import (
	"context"
	"github.com/celestiaorg/celestia-node/blob"
	"github.com/celestiaorg/celestia-node/share"
)

func (o *Operator) getProofFromCelestia(height uint64, namespace share.Namespace, commitment blob.Commitment) ([]byte, error) {
	b, err := o.celestiaClient.Blob.Get(context.Background(), height, namespace, commitment)
	if err != nil {
		return nil, err
	}

	return b.Data, nil
}
