package pkg

import (
	"github.com/yetanotherco/aligned_layer/core/types"
	"net/http"
	"net/rpc"
)

func (aggregator *Aggregator) Serve() error {
	// Registers a new RPC server
	err := rpc.Register(aggregator)
	if err != nil {
		return err
	}

	// Registers an HTTP handler for RPC messages
	rpc.HandleHTTP()

	// Start listening for requests on aggregator address
	// Serve accepts incoming HTTP connections on the listener, creating
	// a new service goroutine for each. The service goroutines read requests
	// and then call handler to reply to them
	aggregator.AggregatorConfig.BaseConfig.Logger.Info("Starting RPC server on address", "address",
		aggregator.AggregatorConfig.Aggregator.AggregatorServerIpPortAddress)
	err = http.ListenAndServe(aggregator.AggregatorConfig.Aggregator.AggregatorServerIpPortAddress, nil)
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
func (aggregator *Aggregator) SubmitTaskResponse(taskResponse *types.SignedTaskResponse, reply *uint8) error {
	aggregator.AggregatorConfig.BaseConfig.Logger.Info("New Task response", "taskResponse", taskResponse)

	// Check if the task exists. If not, return error
	if _, ok := aggregator.taskResponses[taskResponse.TaskIndex]; !ok {
		*reply = 1
		return nil
	}

	// TODO: Mutex?
	aggregator.taskResponses[taskResponse.TaskIndex] = append(aggregator.taskResponses[taskResponse.TaskIndex],
		*taskResponse)

	*reply = 0

	return nil
}

// Dummy method to check if the server is running
// TODO: Remove this method in prod
func (aggregator *Aggregator) ServerRunning(_ *struct{}, reply *int64) error {
	*reply = 1
	return nil
}
