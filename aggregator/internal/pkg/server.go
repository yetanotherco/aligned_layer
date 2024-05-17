package pkg

import (
	"context"
	"fmt"
	"net/http"
	"net/rpc"

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
	if err != nil {
		return err
	}

	return nil
}

// Aggregator Methods
// This is the list of methods that the Aggregator exposes to the Operator
// The Operator can call these methods to interact with the Aggregator
// This methods are automatically registered by the RPC server
// This takes a response an adds it to the internal. If reaching the quorum, it sends the aggregated signatures to ethereum
// Returns:
//   - 0: Success
//   - 1: Error
func (agg *Aggregator) ProcessOperatorSignedTaskResponse(signedTaskResponse *types.SignedTaskResponse, reply *uint8) error {

	agg.AggregatorConfig.BaseConfig.Logger.Info("New task response", "taskResponse", signedTaskResponse)

	if _, ok := agg.OperatorTaskResponses[signedTaskResponse.BatchMerkleRoot]; !ok {
		return fmt.Errorf("task with batch merkle root %d does not exist", signedTaskResponse.BatchMerkleRoot)
	}

	// TODO: Check if the task response is valid
	agg.taskResponsesMutex.Lock()
	taskResponses := agg.OperatorTaskResponses[signedTaskResponse.BatchMerkleRoot]
	taskResponses.taskResponses = append(
		agg.OperatorTaskResponses[signedTaskResponse.BatchMerkleRoot].taskResponses,
		*signedTaskResponse)
	agg.taskResponsesMutex.Unlock()

	agg.taskCounterMutex.Lock()
	taskIndex := agg.taskCounter
	agg.taskCounterMutex.Unlock()

	err := agg.blsAggregationService.ProcessNewSignature(
		context.Background(), taskIndex, signedTaskResponse.BatchMerkleRoot,
		&signedTaskResponse.BlsSignature, signedTaskResponse.OperatorId,
	)
	if err != nil {
		agg.logger.Errorf("BLS aggregation service error: %s", err)
		*reply = 1
		return err
	}

	*reply = 0

	return nil
}

// Dummy method to check if the server is running
// TODO: Remove this method in prod
func (agg *Aggregator) ServerRunning(_ *struct{}, reply *int64) error {
	*reply = 1
	return nil
}
