package pkg

import (
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
	agg.AggregatorConfig.BaseConfig.Logger.Info("Starting RPC server on address", "address",
		agg.AggregatorConfig.Aggregator.ServerIpPortAddress)
	err = http.ListenAndServe(agg.AggregatorConfig.Aggregator.ServerIpPortAddress, nil)
	if err != nil {
		return err
	}

	return nil
}

/// Aggregator Methods
/// This is the list of methods that the Aggregator exposes to the Operator
/// The Operator can call these methods to interact with the Aggregator
/// This methods are automatically registered by the RPC server

// Receives a signed task response from an operator
// Returns:
//   - 0: Success
//   - 1: Error
func (agg *Aggregator) SubmitTaskResponse(taskResponse *types.SignedTaskResponse, reply *uint8) error {
	agg.AggregatorConfig.BaseConfig.Logger.Info("New Task response", "taskResponse", taskResponse)

	taskIndex := taskResponse.TaskResponse.TaskIndex
	// Check if the task exists. If not, get the task from the contract, and store it in the tasks map
	// If the task does not exist, return an error
	if _, ok := agg.taskResponses[taskIndex]; !ok {
		task, err := agg.avsReader.GetNewTaskCreated(taskIndex)
		if err != nil {
			agg.AggregatorConfig.BaseConfig.Logger.Error("Task does not exist", "taskIndex", taskResponse.TaskResponse.TaskIndex)
			*reply = 1
			return fmt.Errorf("task %d does not exist", taskIndex)
		}
		agg.AddNewTask(taskIndex, task.Task)
	}

	// TODO: Check if the task response is valid
	agg.taskResponsesMutex.Lock()

	taskResponses := agg.taskResponses[taskIndex]

	taskResponses.taskResponses = append(
		agg.taskResponses[taskIndex].taskResponses,
		*taskResponse)

	// Submit the task response to the contract when the number of responses is 2
	// TODO: Make this configurable (based on quorum %)
	if !taskResponses.submittedToEthereum && len(taskResponses.taskResponses) >= 2 {
		agg.AggregatorConfig.BaseConfig.Logger.Info("Submitting task response to contract", "taskIndex",
			taskResponse.TaskResponse, "proofIsValid", true)

		_, err := agg.avsWriter.AvsContractBindings.ServiceManager.RespondToTask(agg.avsWriter.Signer.GetTxOpts(),
			taskResponse.TaskResponse.TaskIndex, true)
		if err != nil {
			agg.taskResponsesMutex.Unlock()
			*reply = 1
			return err
		}

		taskResponses.submittedToEthereum = true
	}

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
