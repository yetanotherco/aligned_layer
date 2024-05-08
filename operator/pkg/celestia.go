package operator

import (
	"context"
	"encoding/hex"
	"github.com/celestiaorg/celestia-node/blob"
	"github.com/celestiaorg/celestia-node/share"
)

func (o *Operator) getProofFromCelestia(height uint64, namespace share.Namespace, commitment blob.Commitment) ([]byte, error) {
	o.Logger.Debug("Getting proof from Celestia...", "Height:", height, "Namespace:", namespace, "Commitment:", hex.EncodeToString(commitment))

	b, err := o.celestiaClient.Blob.Get(context.Background(), height, namespace, commitment)
	if err != nil {
		return nil, err
	}

	return b.Data, nil
}
