package operator

import (
	"bytes"
	"context"
	"encoding/hex"
	"github.com/celestiaorg/celestia-node/blob"
	"github.com/celestiaorg/celestia-node/share"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
)

func (o *Operator) getProofByChunksFromCelestia(newTaskCreatedLog *servicemanager.ContractAlignedLayerServiceManagerNewTaskCreated) ([]byte, error) {
	var proofChunks [][]byte
	for _, chunk := range newTaskCreatedLog.Task.DAPayload.Chunks {
		o.Logger.Infof("Getting proof chunk %d from Celestia...", chunk.Index)
		result, err := o.getProofFromCelestia(chunk.Index, o.Config.CelestiaConfig.Namespace, chunk.ProofAssociatedData)
		if err != nil {
			o.Logger.Errorf("Could not get proof from Celestia: %v", err)
			return nil, err
		}
		proofChunks = append(proofChunks, result)
	}
	return bytes.Join(proofChunks, nil), nil
}

func (o *Operator) getProofFromCelestia(height uint64, namespace share.Namespace, commitment blob.Commitment) ([]byte, error) {
	o.Logger.Debug("Getting proof from Celestia...", "Height:", height, "Namespace:", namespace, "Commitment:", hex.EncodeToString(commitment))

	b, err := o.celestiaClient.Blob.Get(context.Background(), height, namespace, commitment)
	if err != nil {
		return nil, err
	}

	return b.Data, nil
}
