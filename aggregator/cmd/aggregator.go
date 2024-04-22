package main

import (
	"aligned_layer/aggregator/internal/rpc_server"
	"fmt"
)

func main() {
	fmt.Println("Booting aggregator ...")

	// Create a new RPC server
	rpc_server.Serve()
}
