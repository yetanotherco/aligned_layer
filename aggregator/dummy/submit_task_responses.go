package main

import (
	"github.com/yetanotherco/aligned_layer/core/types"
	"log"
	"net/rpc"
	"os"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
)

// TODO: Remove this once we have a functioning operator

func main() {
	// Address to this variable will be sent to the RPC server
	// Type of reply should be same as that specified on server
	log.Println("Booting dummy operator ...")

	log.Println("Sending valid task response to aggregator, expecting response 0")
	var reply uint8
	args := types.SignedTaskResponse{
		TaskIndex:    0,
		BlsSignature: *bls.NewZeroSignature(),
		OperatorId:   eigentypes.Bytes32{},
	}

	aggregatorAddress := os.Getenv("AGGREGATOR_ADDRESS")
	if aggregatorAddress == "" {
		log.Println("AGGREGATOR_ADDRESS environment variable not set, using default")
		aggregatorAddress = "localhost:8090"
	}

	// DialHTTP connects to an HTTP RPC server at the specified network
	client, err := rpc.DialHTTP("tcp", aggregatorAddress)
	if err != nil {
		log.Fatal("Client connection error: ", err)
	}

	// Sending the arguments and reply variable address to the server as well
	err = client.Call("Aggregator.SubmitTaskResponse", args, &reply)
	if err != nil {
		log.Fatal("Client invocation error: ", err)
	}

	// Print the reply from the server
	log.Printf("response: %d", reply)
}
