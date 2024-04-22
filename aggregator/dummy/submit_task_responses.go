package main

import (
	"aligned_layer/common/pkg/types"
	"github.com/joho/godotenv"
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

	err := godotenv.Load(".env")
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	log.Println("Sending valid task response to aggregator, expecting response 0")
	var reply uint8
	args := types.SignedTaskResponse{
		TaskResponse: "TaskResponse",
		BlsSignature: *bls.NewZeroSignature(),
		OperatorId:   eigentypes.Bytes32{},
	}

	aggregatorAddress := os.Getenv("AGGREGATOR_ADDRESS")
	if aggregatorAddress == "" {
		log.Println("AGGREGATOR_ADDRESS environment variable not set, using default")
		aggregatorAddress = "localhost:1234"
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

	log.Println("Sending invalid task response to aggregator, expecting response 1")
	args = types.SignedTaskResponse{
		TaskResponse: "",
		BlsSignature: *bls.NewZeroSignature(),
		OperatorId:   eigentypes.Bytes32{},
	}

	// Sending the arguments and reply variable address to the server as well
	err = client.Call("Aggregator.SubmitTaskResponse", args, &reply)
	if err != nil {
		log.Fatal("Client invocation error: ", err)
	}

	// Print the reply from the server
	log.Printf("response: %d", reply)

}
