package rpc_server

import (
	"aligned_layer/aggregator/internal/rpc_server/aggregator"
	"aligned_layer/common/pkg/config"
	"aligned_layer/common/pkg/types"
	"log"
	"os"
	"testing"
)

// TODO: Change when actual task response is received

// TestSubmitTaskResponse tests the SubmitTaskResponse method
// Dummy function body, returns 0 if task response string is not empty
// Expected output: nil, 0

func TestSubmitTaskResponse(testing *testing.T) {
	err := os.Setenv("AGGREGATOR_ADDRESS", "localhost:1234")
	if err != nil {
		log.Fatal("Error setting AGGREGATOR_ADDRESS")
	}

	aggregatorConfig := config.New()
	server := aggregator.New(*aggregatorConfig)
	taskResponse := &types.SignedTaskResponse{
		TaskResponse: "TaskResponse",
	}

	var reply uint8
	err = server.SubmitTaskResponse(taskResponse, &reply)
	if err != nil {
		testing.Errorf("Expected nil, got %v", err)
	}
	if reply != 0 {
		testing.Errorf("Expected 0, got %v", reply)
	}
}

// TestSubmitTaskResponse tests the SubmitTaskResponse method
// Dummy function body, returns 0 if task response string is not empty
// Expected output: nil, 0
func TestSubmitTaskResponseFailEmpty(testing *testing.T) {
	config := config.New()
	server := aggregator.New(*config)
	taskResponse := &types.SignedTaskResponse{
		TaskResponse: "",
	}

	var reply uint8
	err := server.SubmitTaskResponse(taskResponse, &reply)
	if err != nil {
		testing.Errorf("Expected nil, got %v", err)
	}
	if reply != 1 {
		testing.Errorf("Expected 1, got %v", reply)
	}
}
