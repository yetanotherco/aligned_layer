package rpc_server

import (
	"github.com/yetanotherco/aligned_layer/aggregator/internal/rpc_server/aggregator"
	"github.com/yetanotherco/aligned_layer/core/config"
	"net/http"
	"net/rpc"
)

// Serve starts the rpc server
func Serve(aggregatorConfig *config.AggregatorConfig) error {
	server := aggregator.New(*aggregatorConfig)
	// Registers a new RPC server
	err := rpc.Register(server)
	if err != nil {
		return err
	}

	// Registers an HTTP handler for RPC messages
	rpc.HandleHTTP()

	// Start listening for requests on aggregator address
	// Serve accepts incoming HTTP connections on the listener, creating
	// a new service goroutine for each. The service goroutines read requests
	// and then call handler to reply to them
	aggregatorConfig.BaseConfig.Logger.Info("Starting RPC server on address", "address", aggregatorConfig.Aggregator.AggregatorServerIpPortAddress)
	err = http.ListenAndServe(aggregatorConfig.Aggregator.AggregatorServerIpPortAddress, nil)
	if err != nil {
		return err
	}

	return nil
}
