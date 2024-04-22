package rpc_server

import (
	"aligned_layer/aggregator/internal/rpc_server/aggregator"
	"aligned_layer/common/pkg/config"
	"github.com/joho/godotenv"
	"log"
	"net/http"
	"net/rpc"
)

// Serve starts the rpc server
func Serve() {
	err := godotenv.Load(".env")
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	aggregatorConfig := config.New()
	server := aggregator.New(*aggregatorConfig)
	// Registers a new RPC server
	err = rpc.Register(server)
	if err != nil {
		aggregatorConfig.Logger.Fatal("Error registering aggregator server ", "err", err)
	}

	// Registers an HTTP handler for RPC messages
	rpc.HandleHTTP()

	// Start listening for requests on aggregator address
	// Serve accepts incoming HTTP connections on the listener, creating
	// a new service goroutine for each. The service goroutines read requests
	// and then call handler to reply to them
	aggregatorConfig.Logger.Info("Starting RPC server on address", "address", aggregatorConfig.AggregatorAddress)
	err = http.ListenAndServe(aggregatorConfig.AggregatorAddress, nil)
	if err != nil {
		aggregatorConfig.Logger.Fatal("Error on ListenAndServe", "err", err)
	}
}
