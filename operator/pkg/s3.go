package operator

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"time"

	"github.com/ugorji/go/codec"
	"github.com/yetanotherco/aligned_layer/operator/merkle_tree"
)

func (o *Operator) getBatchFromDataService(ctx context.Context, batchURL string, expectedMerkleRoot [32]byte, maxRetries int, retryDelay time.Duration) ([]VerificationData, error) {
	o.Logger.Infof("Getting batch from data service, batchURL: %s", batchURL)

	var resp *http.Response
	var err error
	var req *http.Request

	for attempt := 0; attempt < maxRetries; attempt++ {
		if attempt > 0 {
			o.Logger.Infof("Waiting for %s before retrying data fetch (attempt %d of %d)", retryDelay, attempt+1, maxRetries)
			select {
			case <-time.After(retryDelay):
				// Wait before retrying
			case <-ctx.Done():
				return nil, ctx.Err()
			}
			retryDelay *= 2 // Exponential backoff. Ex: 5s, 10s, 20s
		}

		req, err = http.NewRequestWithContext(ctx, "GET", batchURL, nil)
		if err != nil {
			return nil, err
		}

		resp, err = http.DefaultClient.Do(req)
		if err == nil && resp.StatusCode == http.StatusOK {
			break // Successful request, exit retry loop
		}

		if resp != nil {
			err := resp.Body.Close()
			if err != nil {
				return nil, err
			}
		}

		o.Logger.Warnf("Error fetching batch from data service - (attempt %d): %v", attempt+1, err)
	}

	if err != nil {
		return nil, err
	}

	// At this point, the HTTP request was successfull.

	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			fmt.Println("error closing body: ", err)
		}
	}(resp.Body)

	// Check if the response is OK
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("error getting batch from data service: %s", resp.Status)
	}

	contentLength := resp.ContentLength
	if contentLength > o.Config.Operator.MaxBatchSize {
		return nil, fmt.Errorf("proof size %d exceeds max batch size %d",
			contentLength, o.Config.Operator.MaxBatchSize)
	}

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
	merkle_root_check, err := merkle_tree.VerifyMerkleTreeBatch(batchBytes, expectedMerkleRoot)
	if err != nil || !merkle_root_check {
		return nil, fmt.Errorf("Error while verifying merkle tree batch")
	}
	o.Logger.Infof("Batch merkle tree verified")

	var batch []VerificationData

	decoder, err := createDecoderMode()
	if err != nil {
		return nil, fmt.Errorf("error creating CBOR decoder: %s", err)
	}
	err = decoder.Unmarshal(batchBytes, &batch)

	if err != nil {
		o.Logger.Infof("Error decoding batch as CBOR: %s. Trying JSON decoding...", err)
		// try json
		decoder := codec.NewDecoderBytes(batchBytes, new(codec.JsonHandle))
		err = decoder.Decode(&batch)
		if err != nil {
			return nil, err
		}
	}

	return batch, nil
}
