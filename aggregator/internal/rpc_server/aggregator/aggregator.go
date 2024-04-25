package aggregator

import (
	"github.com/yetanotherco/aligned_layer/core/config"
	"github.com/yetanotherco/aligned_layer/core/types"

	"github.com/Layr-Labs/eigensdk-go/logging"
)

type Aggregator struct {
	logger logging.Logger
}

func New(config config.AggregatorConfig) *Aggregator {
	return &Aggregator{
		logger: config.BaseConfig.Logger,
	}
}

/// Aggregator Methods
/// This is the list of methods that the Aggregator exposes to the Operator
/// The Operator can call these methods to interact with the Aggregator
/// This methods are automatically registered by the RPC server

// Receives a signed task response from an operator
// Returns:
//   - 0: Success
//   - 1: Error
func (a *Aggregator) SubmitTaskResponse(taskResponse *types.SignedTaskResponse, reply *uint8) error {
	a.logger.Info("New Task response", "taskResponse", taskResponse)

	// dummy function body, returns 0 if task response string is not empty
	if taskResponse.TaskResponse != "" {
		*reply = 0
	} else {
		*reply = 1
	}

	return nil
}

// Dummy method to check if the server is running
// TODO: Remove this method in prod
func (a *Aggregator) ServerRunning(_ *struct{}, reply *int64) error {
	*reply = 1
	return nil
}
