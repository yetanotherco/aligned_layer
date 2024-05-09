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

type TaskResponse struct {
	TaskIndex      uint32 "json:\"taskIndex\""
	ProofIsCorrect bool   "json:\"proofIsCorrect\""
}

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
	assert.NotEmpty(t, logs, "No New Events found")

	abiFilePath := "tests/AlignedLayerServiceManager.json"
	contractAbi, err := getAlignedABI(abiFilePath)
	if err != nil {
		log.Fatal(err)
	}
	NewTaskCreatedEventSignature := contractAbi.Events["NewTaskCreated"].ID
	TaskRespondedEventSignature := contractAbi.Events["TaskResponded"].ID

	var taskCreatedEvents = 0
	var taskRespondedEvents = 0

	for _, vLog := range logs {
		switch vLog.Topics[0] {
		case NewTaskCreatedEventSignature:
			taskCreated, _ := contractAbi.Unpack("NewTaskCreated", vLog.Data) // Couldn't cast this to a TaskResponse struct defined outside
			task := taskCreated[0].(struct {
				ProvingSystemId uint16 `json:"provingSystemId"`

				DAPayload struct {
					Solution            uint8   `json:"solution"`
					ProofAssociatedData []uint8 `json:"proof_associated_data"`
					Index               uint64  `json:"index"`
				} `json:"DAPayload"`

				PubInput                   []uint8  `json:"pubInput"`
				VerificationKey            []uint8  `json:"verificationKey"`
				TaskCreatedBlock           uint32   `json:"taskCreatedBlock"`
				QuorumNumbers              []uint8  `json:"quorumNumbers"`
				QuorumThresholdPercentages []uint8  `json:"quorumThresholdPercentages"`
				Fee                        *big.Int `json:"fee"`
			})

			// If TaskIndex is added to Task struct, we can cast this event's Data to a Task struct to read it's TaskId
			// like it is done in TaskRespondedEventSignature event
			if taskCreatedEvents == 0 {
				assert.Equal(t, "0x0000000000000000000000000000000000000000000000000000000000000000", vLog.Topics[1].Hex(), "Expected NewTaskCreated event with taskId 0")
				assert.Equal(t, uint16(0), task.ProvingSystemId, "Expected NewTaskCreated event with provingSystemId 0")
				assert.Equal(t, uint8(98), task.QuorumThresholdPercentages[0], "Expected NewTaskCreated event with quorumThresholdPercentages 98")
				assert.Equal(t, big.NewInt(1), task.Fee, "Expected NewTaskCreated event with fee 1")
				assert.Equal(t, uint8(0), task.DAPayload.Solution, "Expected Solution to be Calldata")
			} else if taskCreatedEvents == 1 {
				assert.Equal(t, "0x0000000000000000000000000000000000000000000000000000000000000001", vLog.Topics[1].Hex(), "Expected NewTaskCreated event with taskId 1")
				assert.Equal(t, uint16(1), task.ProvingSystemId, "Expected NewTaskCreated event with provingSystemId 0")
				assert.Equal(t, uint8(100), task.QuorumThresholdPercentages[0], "Expected NewTaskCreated event with quorumThresholdPercentages 98")
				assert.Equal(t, big.NewInt(1), task.Fee, "Expected NewTaskCreated event with fee 1")
				assert.Equal(t, uint8(0), task.DAPayload.Solution, "Expected Solution to be Calldata")
			} else {
				assert.Fail(t, "Too many NewTaskCreated events")
			}
			taskCreatedEvents++

		case TaskRespondedEventSignature:
			taskResponded, _ := contractAbi.Unpack("TaskResponded", vLog.Data) // Couldn't cast this to a TaskResponse struct defined outside
			task := taskResponded[0].(struct {
				TaskIndex      uint32 "json:\"taskIndex\""
				ProofIsCorrect bool   "json:\"proofIsCorrect\""
			})
			assert.Equal(t, uint32(taskRespondedEvents), task.TaskIndex, "Expected NewTaskCreated event with taskId")
			assert.Equal(t, true, task.ProofIsCorrect, "Expected TaskResponse bool true")
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
