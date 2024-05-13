package operator

import (
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"errors"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"io"
	"net/http"
	"strconv"
	"strings"
)

type BlobResponse struct {
	Data []struct {
		Index string `json:"index"`
		Blob  string `json:"blob"`
	} `json:"data"`
}

func (o *Operator) getProofByChunksFromBlobs(newTaskCreatedLog *servicemanager.ContractAlignedLayerServiceManagerNewTaskCreated) ([]byte, error) {
	var proofChunks []string

	// TODO: For now we assume that all the blobs are in the same beacon root, so for example we use the first one
	blobResponse, err := o.getResponseFromBeaconRoot(newTaskCreatedLog.Task.DAPayload.Chunks[0].ProofAssociatedData)
	if err != nil {
		o.Logger.Errorf("Could not get response from block root hash: %v", err)
		return nil, err
	}

	o.Logger.Infof("Got %d blobs from beacon root %s", len(blobResponse.Data), newTaskCreatedLog.Task.DAPayload.Chunks[0].ProofAssociatedData)
	for _, chunk := range newTaskCreatedLog.Task.DAPayload.Chunks {
		o.Logger.Infof("Getting proof chunk for blob %v...", chunk.Index)
		proofChunk, err := o.getProofChunkFromBlobResponse(blobResponse, chunk.Index)
		if err != nil {
			o.Logger.Errorf("Could not get proof from blobs: %v", err)
			return nil, err
		}
		proofChunks = append(proofChunks, *proofChunk)
	}

	fullString := strings.Join(proofChunks, "")

	fullBytes, err := hex.DecodeString(fullString)
	if err != nil {
		return nil, err
	}

	// Decode hex
	decodedProof := make([]byte, hex.DecodedLen(len(fullBytes)))
	decodedBytes, err := hex.Decode(decodedProof, fullBytes)

	// Proof is always 128 KB padded with zeros, ignore trailing zeros error
	if err != nil && !errors.Is(err, hex.InvalidByteError(0)) {
		return nil, err
	}

	if decodedBytes == 0 {
		return nil, errors.New("failed to decode proof")
	}

	decodedProof = decodedProof[:decodedBytes]

	// Get RLP length
	firstByte := decodedProof[0]

	if firstByte < 184 || firstByte > 191 {
		return nil, errors.New("invalid proof rlp encoding")
	}

	// RLP string more than 55 bytes
	lenLen := uint64(firstByte - 183)

	// Need to pad with zeros to decode length
	lenBytes := make([]byte, 8)
	copy(lenBytes[8-lenLen:], decodedProof[1:1+lenLen])
	dataLen := binary.BigEndian.Uint64(lenBytes)

	// Proof starts after length bytes & ends after data bytes
	proofStart := lenLen + 1
	proofEnd := lenLen + dataLen + 1

	// Trim to actual data bytes
	return decodedProof[proofStart:proofEnd], nil
}

func (o *Operator) getResponseFromBeaconRoot(beaconRoot []byte) (*BlobResponse, error) {
	beaconRootStr := hex.EncodeToString(beaconRoot)
	o.Logger.Infof("Getting response from beacon root: %s", beaconRootStr)

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

func (o *Operator) getProofChunkFromBlobResponse(blobResponse *BlobResponse, index uint64) (*string, error) {
	for _, blob := range blobResponse.Data {
		blobIndexInt, err := strconv.Atoi(blob.Index)
		if err != nil {
			return nil, err
		}

		blobIndex := uint64(blobIndexInt)
		if blobIndex == index {
			if blob.Blob[:2] == "0x" {
				blob.Blob = blob.Blob[2:]
			}
			return &blob.Blob, nil
		}
	}
	return nil, errors.New("index not found in blob response")
}
