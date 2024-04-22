package rpc_server

import (
	"aligned_layer/aggregator/internal/rpc_server/aggregator"
	"aligned_layer/common/pkg/config"
	"net/http"
	"net/rpc"
)

// Serve starts the rpc server
func Serve() {
	config := config.New()
	// Create a new Aggregator object
	server := aggregator.New(*config)
	// Registers a new RPC server
	err := rpc.Register(server)
	if err != nil {
		config.Logger.Fatal("Format of service TaskManager isn't correct. ", "err", err)
	}

	// Registers an HTTP handler for RPC messages
	rpc.HandleHTTP()

	// Start listening for requests on port 1234
	// Serve accepts incoming HTTP connections on the listener, creating
	// a new service goroutine for each. The service goroutines read requests
	// and then call handler to reply to them
	config.Logger.Info("Starting RPC server on address", "address", config.AggregatorAddress)
	err = http.ListenAndServe(config.AggregatorAddress, nil)
	if err != nil {
		config.Logger.Fatal("ListenAndServe", "err", err)
	}
}
