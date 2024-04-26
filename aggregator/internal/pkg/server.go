package pkg

import (
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/yetanotherco/aligned_layer/core/types"
	"net/http"
	"net/rpc"
)

func (agg *Aggregator) Serve() error {
	// Registers a new RPC server
	err := rpc.Register(agg)
	if err != nil {
		return err
	}

	// Registers an HTTP handler for RPC messages
	rpc.HandleHTTP()

	// Start listening for requests on aggregator address
	// Serve accepts incoming HTTP connections on the listener, creating
	// a new service goroutine for each. The service goroutines read requests
	// and then call handler to reply to them
	agg.AggregatorConfig.BaseConfig.Logger.Info("Starting RPC server on address", "address",
		agg.AggregatorConfig.Aggregator.AggregatorServerIpPortAddress)
	err = http.ListenAndServe(agg.AggregatorConfig.Aggregator.AggregatorServerIpPortAddress, nil)
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

	// Check if the task exists. If not, return error
	if _, ok := agg.taskResponses[taskResponse.TaskIndex]; !ok {
		*reply = 1
		return nil
	}

	// TODO: Check if the task response is valid

	// TODO: Mutex?
	agg.taskResponses[taskResponse.TaskIndex] = append(agg.taskResponses[taskResponse.TaskIndex],
		*taskResponse)

	// Submit the task response to the contract when the number of responses is 2
	// TODO: Make this configurable (based on quorum %)
	// TODO: Check if response has already been submitted
	if len(agg.taskResponses[taskResponse.TaskIndex]) >= 2 {
		agg.AggregatorConfig.BaseConfig.Logger.Info("Submitting task response to contract", "taskIndex",
			taskResponse.TaskIndex, "proofIsValid", true)

		_, err := agg.avsWriter.AvsContractBindings.ServiceManager.RespondToTask(&bind.TransactOpts{},
			taskResponse.TaskIndex, true)
		if err != nil {
			*reply = 1
			return err
		}
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
