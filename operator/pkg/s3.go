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

	contentLength := resp.ContentLength
	if contentLength > o.Config.Operator.MaxBatchSize {
		return nil, fmt.Errorf("proof size %d exceeds max batch size %d",
			contentLength, o.Config.Operator.MaxBatchSize)
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

	// Use io.LimitReader to limit the size of the response body
	// This is to prevent the operator from downloading a larger than expected file
	// + 1 is added to the contentLength to check if the response body is larger than expected
	reader := io.LimitedReader{R: resp.Body, N: contentLength + 1}
	batchBytes, err := io.ReadAll(&reader)
	if err != nil {
		return nil, err
	}

	// Check if the response body is larger than expected
	if reader.N <= 0 {
		return nil, fmt.Errorf("batch size exceeds max batch size %d", o.Config.Operator.MaxBatchSize)
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
