package tests

import (
	"bufio"
	"context"
	"fmt"
	"log"
	"math/big"
	"net/http"
	"os"

	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/stretchr/testify/assert"

	"regexp"
	"strings"
	"testing"
)

func TestEventsReader(t *testing.T) {
	fmt.Println("Running integration test")
	err := os.Chdir("../")
	assert.Nil(t, err, "Could not change directory to project root")

	configFilePath := "config-files/config-test.yaml"

	result, anvilPort, alignedContractAddress := checkAnvilIsRunning(configFilePath)
	if !result {
		t.Fatalf("Expected Anvil to be running, in port %s but it was not.", anvilPort)
	}

	client, err := ethclient.Dial("http://localhost:" + anvilPort)
	if err != nil {
		log.Fatal(err)
	}

	// check if contract sent event
	query := ethereum.FilterQuery{
		FromBlock: big.NewInt(0),
		Addresses: []common.Address{
			common.HexToAddress(alignedContractAddress),
		},
	}
	logs, err := client.FilterLogs(context.Background(), query)
	if err != nil {
		log.Fatal(err)
	}

	abiFilePath := "/Users/urix/aligned_layer/tests/AlignedLayerServiceManager.json"
	contractAbi, err := getAlignedABI(abiFilePath)
	if err != nil {
		log.Fatal(err)
	}

	NewTaskCreatedEventSignature := contractAbi.Events["NewTaskCreated"].ID.Hex()
	TaskRespondedEventSignature := contractAbi.Events["TaskResponded"].ID.Hex()

	assert.NotEmpty(t, logs, "No New Events found")

	var taskCreatedEvents = 0
	var taskRespondedEvents = 0

	for _, vLog := range logs {
		switch vLog.Topics[0].Hex() {
		case NewTaskCreatedEventSignature:
			if taskCreatedEvents == 0 {
				assert.Equal(t, "0x0000000000000000000000000000000000000000000000000000000000000000", vLog.Topics[1].Hex(), "Expected NewTaskCreated event with taskId 0")
			} else if taskCreatedEvents == 1 {
				assert.Equal(t, "0x0000000000000000000000000000000000000000000000000000000000000001", vLog.Topics[1].Hex(), "Expected NewTaskCreated event with taskId 1")
			} else {
				assert.Fail(t, "Too many NewTaskCreated events")
			}
			taskCreatedEvents++

		case TaskRespondedEventSignature:
			if taskRespondedEvents == 0 {
				assert.Equal(t, "0x0000000000000000000000000000000000000000000000000000000000000000", vLog.Topics[1].Hex(), "Expected NewTaskCreated event with taskId 0")
			} else if taskRespondedEvents == 1 {
				assert.Equal(t, "0x0000000000000000000000000000000000000000000000000000000000000001", vLog.Topics[1].Hex(), "Expected NewTaskCreated event with taskId 1")
			} else {
				assert.Fail(t, "Too many TaskResponded events")
			}
			taskRespondedEvents++

		default:
			assert.Fail(t, "Unknown event")
			fmt.Println("Unknown event")
		}
	}
	assert.Equal(t, 2, taskCreatedEvents, "Expected 2 NewTaskCreated events")
	assert.Equal(t, 2, taskRespondedEvents, "Expected 2 TaskResponded events")
}

func getAlignedABI(abiFilePath string) (abi.ABI, error) {
	abiBytes, err := os.ReadFile(abiFilePath)
	if err != nil {
		log.Fatalf("Failed to read ABI file: %v", err)
	}

	// Convert the byte slice to a string
	abiString := string(abiBytes)

	// Parse the ABI string into an ABI object
	return abi.JSON(strings.NewReader(abiString))
}

func checkAnvilIsRunning(configFilePath string) (bool, string, string) {
	file, err := os.Open(configFilePath)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return false, "", ""
	}
	defer file.Close()
	var port string
	var avsServiceManagerAddress string
	portRegex := regexp.MustCompile(`eth_rpc_url: "http://localhost:(\d+)"`)
	avsServiceManagerAddressRegex := regexp.MustCompile(`avs_service_manager_address: (\w+)`)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		// Check for the port
		portMatch := portRegex.FindStringSubmatch(line)
		if len(portMatch) > 1 {
			port = portMatch[1]
		}
		// Check for the avs_service_manager_address
		avsServiceManagerAddressMatch := avsServiceManagerAddressRegex.FindStringSubmatch(line)
		if len(avsServiceManagerAddressMatch) > 1 {
			avsServiceManagerAddress = avsServiceManagerAddressMatch[1]
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "Error reading file:", err)
	}

	_, err = http.Get("http://localhost:" + port)
	if err != nil {
		return false, port, avsServiceManagerAddress
	}

	return true, port, avsServiceManagerAddress
}
