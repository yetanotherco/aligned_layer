package pkg

import (
	"context"
	"net/http"
	"net/rpc"

	"github.com/yetanotherco/aligned_layer/core/types"
	"github.com/yetanotherco/aligned_layer/core/utils"
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

// / Aggregator Methods
// / This is the list of methods that the Aggregator exposes to the Operator
// / The Operator can call these methods to interact with the Aggregator
// / This methods are automatically registered by the RPC server
// / This takes a response an adds it to the internal. If reaching the quorum, it sends the aggregated signatures to ethereum
// Returns:
//   - 0: Success
//   - 1: Error
func (agg *Aggregator) ProcessOperatorSignedTaskResponse(signedTaskResponse *types.SignedTaskResponse, reply *uint8) error {

	agg.AggregatorConfig.BaseConfig.Logger.Info("New Task response", "taskResponse", signedTaskResponse)

	taskIndex := signedTaskResponse.TaskResponse.TaskIndex
	// Check if the task exists. If not, return error
	if _, ok := agg.OperatorTaskResponses[taskIndex]; !ok {
		// TODO: Check if the aggregator has missed the task
		agg.AggregatorConfig.BaseConfig.Logger.Error("Task does not exist", "taskIndex", signedTaskResponse.TaskResponse.TaskIndex)
		*reply = 1
		return nil
	}

	// TODO: Check if the task response is valid
	agg.taskResponsesMutex.Lock()
	taskResponses := agg.OperatorTaskResponses[taskIndex]
	taskResponses.taskResponses = append(
		agg.OperatorTaskResponses[signedTaskResponse.TaskResponse.TaskIndex].taskResponses,
		*signedTaskResponse)

	taskResponseDigest, err := utils.TaskResponseDigest(&signedTaskResponse.TaskResponse)
	if err != nil {
		return err
	}

	err = agg.blsAggregationService.ProcessNewSignature(
		context.Background(), taskIndex, taskResponseDigest,
		&signedTaskResponse.BlsSignature, signedTaskResponse.OperatorId,
	)
	if err != nil {
		agg.logger.Errorf("BLS aggregation service error: %s", err)
		return err
	}

	// Submit the task response to the contract when the number of responses is 2
	// TODO: Make this configurable (based on quorum %)
	// if !taskResponses.submittedToEthereum && len(taskResponses.taskResponses) >= 2 {
	// 	agg.AggregatorConfig.BaseConfig.Logger.Info("Submitting task response to contract", "taskIndex",
	// 		signedTaskResponse.TaskResponse, "proofIsValid", true)

	// 	task := agg.tasks[taskIndex]

	// 	_, err := agg.avsWriter.AvsContractBindings.ServiceManager.RespondToTask(agg.avsWriter.Signer.GetTxOpts(),
	// 		task, signedTaskResponse.TaskResponse)
	// 	if err != nil {
	// 		agg.taskResponsesMutex.Unlock()
	// 		*reply = 1
	// 		return err
	// 	}

	// 	taskResponses.submittedToEthereum = true
	// }

	agg.taskResponsesMutex.Unlock()
	*reply = 0

	return nil
}

// Dummy method to check if the server is running
// TODO: Remove this method in prod
func (agg *Aggregator) ServerRunning(_ *struct{}, reply *int64) error {
	*reply = 1
	return nil
}
