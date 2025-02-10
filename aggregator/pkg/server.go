package pkg

import (
	"context"
	"encoding/hex"
	"errors"
	"fmt"
	"net/http"
	"net/rpc"
	"time"

	retry "github.com/yetanotherco/aligned_layer/core"
	"github.com/yetanotherco/aligned_layer/core/types"
)

func (agg *Aggregator) ServeOperators() error {
	// Registers a new RPC server
	err := rpc.Register(agg)
	if err != nil {
		return err
	}

	// Registers an HTTP handler for RPC messages
	rpc.HandleHTTP()

	// Start listening for requests on aggregator address
	// ServeOperators accepts incoming HTTP connections on the listener, creating
	// a new service goroutine for each. The service goroutines read requests
	// and then call handler to reply to them
	agg.logger.Info("Starting RPC server on address", "address",
		agg.AggregatorConfig.Aggregator.ServerIpPortAddress)

	err = http.ListenAndServe(agg.AggregatorConfig.Aggregator.ServerIpPortAddress, nil)

	return err
}

// Aggregator Methods
// This is the list of methods that the Aggregator exposes to the Operator
// The Operator can call these methods to interact with the Aggregator
// This methods are automatically registered by the RPC server
// This takes a response an adds it to the internal. If reaching the quorum, it sends the aggregated signatures to ethereum
// Returns:
//   - 0: Success
//   - 1: Error
func (agg *Aggregator) ProcessOperatorSignedTaskResponseV2(signedTaskResponse *types.SignedTaskResponse, reply *uint8) error {
	agg.AggregatorConfig.BaseConfig.Logger.Info("New task response",
		"BatchMerkleRoot", "0x"+hex.EncodeToString(signedTaskResponse.BatchMerkleRoot[:]),
		"SenderAddress", "0x"+hex.EncodeToString(signedTaskResponse.SenderAddress[:]),
		"BatchIdentifierHash", "0x"+hex.EncodeToString(signedTaskResponse.BatchIdentifierHash[:]),
		"operatorId", hex.EncodeToString(signedTaskResponse.OperatorId[:]))

	if signedTaskResponse.BlsSignature.G1Point == nil {
		agg.logger.Warn("invalid operator response with nil signature",
			"BatchMerkleRoot", "0x"+hex.EncodeToString(signedTaskResponse.BatchMerkleRoot[:]),
			"SenderAddress", "0x"+hex.EncodeToString(signedTaskResponse.SenderAddress[:]),
			"BatchIdentifierHash", "0x"+hex.EncodeToString(signedTaskResponse.BatchIdentifierHash[:]),
			"operatorId", hex.EncodeToString(signedTaskResponse.OperatorId[:]))
		*reply = 1
		return errors.New("invalid response: nil signature")
	}

	taskIndex := uint32(0)

	// The Aggregator may receive the Task Identifier after the operators.
	// If that's the case, we won't know about the task at this point
	// so we make GetTaskIndex retryable, waiting for some seconds,
	// before trying to fetch the task again from the map.
	taskIndex, err := agg.GetTaskIndexRetryable(signedTaskResponse.BatchIdentifierHash, retry.NetworkRetryParams())

	if err != nil {
		agg.logger.Warn("Task not found in the internal map, operator signature will be lost. Batch may not reach quorum")
		*reply = 1
		return nil
	}
	agg.telemetry.LogOperatorResponse(signedTaskResponse.BatchMerkleRoot, signedTaskResponse.OperatorId)

	// Don't wait infinitely if it can't answer
	// Create a context with a timeout of 5 seconds
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel() // Ensure the cancel function is called to release resources

	// Create a channel to signal when the task is done
	done := make(chan struct{})

	agg.logger.Info("Starting bls signature process")
	go func() {
		err := agg.blsAggregationService.ProcessNewSignature(
			context.Background(), taskIndex, signedTaskResponse.BatchIdentifierHash,
			&signedTaskResponse.BlsSignature, signedTaskResponse.OperatorId,
		)

		if err != nil {
			agg.logger.Warnf("BLS aggregation service error: %s", err)
			// todo shouldn't we here close the channel with a reply = 1?
		} else {
			agg.logger.Info("BLS process succeeded")
		}

		close(done)
	}()

	*reply = 1
	// Wait for either the context to be done or the task to complete
	select {
	case <-ctx.Done():
		// The context's deadline was exceeded or it was canceled
		agg.logger.Info("Bls process timed out, operator signature will be lost. Batch may not reach quorum")
	case <-done:
		// The task completed successfully
		agg.logger.Info("Bls context finished correctly")
		*reply = 0
	}

	return nil
}

// Dummy method to check if the server is running
// TODO: Remove this method in prod
func (agg *Aggregator) ServerRunning(_ *struct{}, reply *int64) error {
	*reply = 1
	return nil
}

/*
Checks Internal mapping for Signed Task Response, returns its TaskIndex.
- All errors are considered Transient Errors
- Retry times (3 retries): 1 sec, 2 sec, 4 sec
TODO: We should refactor the retry duration considering extending it to a larger time or number of retries, at least somewhere between 1 and 2 blocks
*/
func (agg *Aggregator) GetTaskIndexRetryable(batchIdentifierHash [32]byte, config *retry.RetryParams) (uint32, error) {
	getTaskIndex_func := func() (uint32, error) {
		agg.taskMutex.Lock()
		taskIndex, ok := agg.batchesIdxByIdentifierHash[batchIdentifierHash]
		agg.taskMutex.Unlock()
		if !ok {
			return taskIndex, fmt.Errorf("Task not found in the internal map")
		} else {
			return taskIndex, nil
		}
	}

	return retry.RetryWithData(getTaskIndex_func, config)
}
