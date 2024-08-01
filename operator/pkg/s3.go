package operator

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"

	"github.com/yetanotherco/aligned_layer/operator/merkle_tree"
)

func (o *Operator) getBatchFromS3(batchURL string, expectedMerkleRoot [32]byte) ([]VerificationData, error) {
	o.Logger.Infof("Getting batch from S3..., batchURL: %s", batchURL)
	resp, err := http.Head(batchURL)
	if err != nil {
		return nil, err
	}

	// Check if the response is OK
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("error getting Proof Head from S3: %s", resp.Status)
	}

	if resp.ContentLength > o.Config.Operator.MaxBatchSize {
		return nil, fmt.Errorf("proof size %d exceeds max batch size %d",
			resp.ContentLength, o.Config.Operator.MaxBatchSize)
	}

	resp, err = http.Get(batchURL)
	if err != nil {
		return nil, err
	}
	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			fmt.Println("error closing body: ", err)
		}
	}(resp.Body)

	batchBytes, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	// Checks if downloaded merkle root is the same as the expected one
	o.Logger.Infof("Verifying batch merkle tree...")
	merkle_root_check := merkle_tree.VerifyMerkleTreeBatch(batchBytes, uint(len(batchBytes)), expectedMerkleRoot)
	if !merkle_root_check {
		return nil, fmt.Errorf("merkle root check failed")
	}
	o.Logger.Infof("Batch merkle tree verified")

	var batch []VerificationData

	err = json.Unmarshal(batchBytes, &batch)
	if err != nil {
		return nil, err
	}

	return batch, nil
}
