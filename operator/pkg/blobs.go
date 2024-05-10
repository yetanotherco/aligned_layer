package operator

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"errors"
	"github.com/ethereum/go-ethereum/rlp"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"io"
	"log"
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

	log.Printf("Got %d blobs from beacon root %s", len(blobResponse.Data), newTaskCreatedLog.Task.DAPayload.Chunks[0].ProofAssociatedData)
	for _, chunk := range newTaskCreatedLog.Task.DAPayload.Chunks {
		o.Logger.Infof("Getting proof chunk for blob %v...", chunk.Index)
		proofChunk, err := o.getProofChunkFromBlobResponse(blobResponse, chunk.Index)
		if err != nil {
			o.Logger.Errorf("Could not get proof from blobs: %v", err)
			return nil, err
		}
		proofChunks = append(proofChunks, proofChunk)
	}

	fullBytes := bytes.Join(proofChunks, nil)

	// Decode hex
	decodedProof := make([]byte, hex.DecodedLen(len(fullBytes)))
	_, err = hex.Decode(decodedProof, fullBytes)
	if err != nil {
		return nil, err
	}

	// Decode RLP
	buff := make([]byte, len(decodedProof))
	err = rlp.DecodeBytes(decodedProof, &buff)
	if err != nil {
		return nil, err
	}

	return buff, nil
}

func (o *Operator) getResponseFromBeaconRoot(beaconRoot []byte) (*BlobResponse, error) {
	beaconRootStr := hex.EncodeToString(beaconRoot)
	log.Println("Getting response from beacon root: ", beaconRootStr)

	resp, err := http.Get(o.Config.BlobsConfig.BeaconChainRpcUrl + "/eth/v1/beacon/blob_sidecars/0x" + beaconRootStr)
	if err != nil {
		return nil, err
	}
	if resp.StatusCode != http.StatusOK {
		return nil, errors.New("could not get response from beacon root")
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

		log.Println("Blob index: ", blobIndexInt)

		blobIndex := uint64(blobIndexInt)
		if blobIndex == index {
			//decodedBlob := make([]byte, hex.DecodedLen(len(blob.Blob)))
			//_, err = hex.Decode(decodedBlob, []byte(blob.Blob))
			// remove 0x prefix
			if blob.Blob[:2] == "0x" {
				blob.Blob = blob.Blob[2:]
			}
			//decodedBlob, err := hex.DecodeString(blob.Blob)
			//if err != nil {
			//	return nil, err
			//}

			return []byte(blob.Blob), nil
		}
	}
	return nil, errors.New("index not found in blob response")
}
