package operator

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"

	"github.com/ugorji/go/codec"
	"github.com/yetanotherco/aligned_layer/operator/merkle_tree"
)

const (
	MaxBatchURLs = 5
)

// getBatchFromDataServiceWithMultipleURLs tries multiple comma-separated URLs until first successful response
func (o *Operator) getBatchFromDataServiceWithMultipleURLs(ctx context.Context, batchURLs string, expectedMerkleRoot [32]byte, maxRetries int, retryDelay time.Duration) ([]VerificationData, error) {
    // Parse comma-separated URLs and limit to max 5
	urls := parseBatchURLs(batchURLs)
	o.Logger.Infof("Getting batch from data service with %d URLs: %v", len(urls), urls)

	var errors []string

	// Try each URL until first successful response
	for i, batchURL := range urls {
		o.Logger.Infof("Trying URL %d of %d: %s", i+1, len(urls), batchURL)

		batch, err := o.getBatchFromDataService(ctx, batchURL, expectedMerkleRoot, maxRetries, retryDelay)
		if err != nil {
			o.Logger.Warnf("Failed to get batch from URL %s: %v", batchURL, err)
			errors = append(errors, fmt.Sprintf("URL %s: %v", batchURL, err))
		} else {
			o.Logger.Infof("Successfully retrieved batch from URL: %s", batchURL)
			return batch, nil
		}
	}

	// All URLs failed
	return nil, fmt.Errorf("failed to get batch from all URLs, errors: %s", strings.Join(errors, "; "))
}

// parseBatchURLs parses comma-separated URLs and limits to max 5
func parseBatchURLs(batchURLs string) []string {
	urls := make([]string, 0)
	for _, url := range strings.Split(batchURLs, ",") {
		trimmedURL := strings.TrimSpace(url)
		if trimmedURL != "" {
			urls = append(urls, trimmedURL)
			if len(urls) > MaxBatchURLs {
				break
			}
		}
	}

	return urls
}

func (o *Operator) getBatchFromDataService(ctx context.Context, batchURL string, expectedMerkleRoot [32]byte, maxRetries int, retryDelay time.Duration) ([]VerificationData, error) {
	o.Logger.Infof("Getting batch from data service, batchURL: %s", batchURL)

	var resp *http.Response
	var err error
	var req *http.Request

	// Create HTTP client with response header timeout to prevent hanging on silent servers
	transport := http.DefaultTransport.(*http.Transport).Clone()
	transport.ResponseHeaderTimeout = 15 * time.Second
	client := &http.Client{
		Transport: transport,
	}

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

		resp, err = client.Do(req)
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
		return nil, fmt.Errorf("error while verifying merkle tree batch")
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
