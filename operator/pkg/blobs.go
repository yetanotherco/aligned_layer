package operator

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"errors"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"io"
	"net/http"
	"strconv"
)

type BlobResponse struct {
	Data []struct {
		Index string `json:"index"`
		Blob  string `json:"blob"`
	} `json:"data"`
}

func (o *Operator) getProofByChunksFromBlobs(newTaskCreatedLog *servicemanager.ContractAlignedLayerServiceManagerNewTaskCreated) ([]byte, error) {
	var proofChunks [][]byte
	// TODO: For now we assume that all the blobs are in the same beacon root, so for example we use the first one
	blobResponse, err := o.getResponseFromBeaconRoot(newTaskCreatedLog.Task.DAPayload.Chunks[0].ProofAssociatedData)
	if err != nil {
		o.Logger.Errorf("Could not get response from block root hash: %v", err)
		return nil, err
	}
	for _, chunk := range newTaskCreatedLog.Task.DAPayload.Chunks {
		o.Logger.Infof("Getting proof chunk for blob %v...", chunk.Index)
		proofChunk, err := o.getProofChunkFromBlobResponse(blobResponse, chunk.Index)
		if err != nil {
			o.Logger.Errorf("Could not get proof from blobs: %v", err)
			return nil, err
		}
		proofChunks = append(proofChunks, proofChunk)
	}
	return bytes.Join(proofChunks, nil), nil
}

func (o *Operator) getResponseFromBeaconRoot(beaconRoot []byte) (*BlobResponse, error) {
	beaconRootStr := string(beaconRoot)
	resp, err := http.Get(o.Config.BlobsConfig.BeaconChainRpcUrl + "/eth/v1/beacon/blob_sidecars/" + beaconRootStr)
	if err != nil {
		return nil, err
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	decodedBody := BlobResponse{}
	err = json.Unmarshal(body, &decodedBody)
	if err != nil {
		return nil, err
	}

	return &decodedBody, nil
}

func (o *Operator) getProofChunkFromBlobResponse(blobResponse *BlobResponse, index uint64) ([]byte, error) {
	for _, blob := range blobResponse.Data {
		blobIndexInt, err := strconv.Atoi(blob.Index)
		if err != nil {
			return nil, err
		}

		blobIndex := uint64(blobIndexInt)
		if blobIndex == index {
			decodedBlob, err := hex.DecodeString(blob.Blob)
			if err != nil {
				return nil, err
			}
			return decodedBlob, nil
		}
	}
	return nil, errors.New("index not found in blob response")
}
