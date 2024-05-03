package tests

import (
	"bufio"
	"context"
	"fmt"
	"log"
	"math/big"
	"net/http"
	"os"
	// "time"

	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/stretchr/testify/assert"
	"github.com/yetanotherco/aligned_layer/aggregator/pkg"
	"github.com/yetanotherco/aligned_layer/core/config"
	operator "github.com/yetanotherco/aligned_layer/operator/pkg"

	"bytes"
	"os/exec"

	"regexp"
	"strings"
	"testing"
)

func TestIntegration(t *testing.T) {
	fmt.Println("Running integration test")
	err := os.Chdir("../")
	assert.Nil(t, err, "Could not change directory to project root")

	configFilePath := "config-files/config-test.yaml"

	// check anvil is running
	result, anvilPort, alignedContractAddress := checkAnvilIsRunning(configFilePath)
	if !result {
		t.Fatalf("Expected Anvil to be running, in port %s but it was not.", anvilPort)
	}
	// fmt.Println("alignedContractAddress: " + alignedContractAddress)

	// Setup RPC client
	client, err := ethclient.Dial("http://localhost:" + anvilPort)
	if err != nil {
		log.Fatal(err)
	}

	// start aggregator
	aggregator := buildAggregator(t, configFilePath)
	go func() {
		err := aggregator.ServeOperators()
		assert.Nil(t, err, "Could not start aggregator")
	}()
	fmt.Println("Aggregator started")

	// register operator
	var out bytes.Buffer
	cmd := exec.Command("make", "operator-full-registration") //this is running a bit wonky?
	cmd.Stdout = &out
	err = cmd.Run()
	if err != nil {
		log.Fatalf("cmd.Run() failed with %s\n", err)
	}
	fmt.Println("Operator registered")
	fmt.Println("Output:", out.String())

	// start operator
	op := buildOperator(t, configFilePath)
	go func() {
		err := op.Start(context.Background())
		assert.Nil(t, err, "Could not start operator")
	}()
	fmt.Println("Operator started")

	// send task
	cmd = exec.Command("make", "send-plonk_bls12_381-proof")
	cmd.Stdout = &out
	err = cmd.Run()
	if err != nil {
		log.Fatalf("cmd.Run() failed with %s\n", err)
	}
	fmt.Println("Task sent successfully")

	// send task #2
	// var out bytes.Buffer
	cmd = exec.Command("make", "send-plonk_bn254-proof")
	// cmd.Stdout = &out
	err = cmd.Run()
	if err != nil {
		log.Fatalf("cmd.Run() failed with %s\n", err)
	}
	fmt.Println("Task #2 sent successfully")

	// check if contract sent event
	query := ethereum.FilterQuery{
		FromBlock: big.NewInt(0),
		// ToBlock:   big.NewInt(2394201),
		Addresses: []common.Address{
			common.HexToAddress(alignedContractAddress),
		},
	}
	logs, err := client.FilterLogs(context.Background(), query)
	if err != nil {
		log.Fatal(err)
	}

	abiFilePath := "/Users/urix/aligned_layer/tests/AlignedLAyerServiceManager.json"
	contractAbi, err := getAlignedABI(abiFilePath)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(contractAbi)

	// fmt.Println("value")
	// fmt.Print(logs)
	assert.NotEmpty(t, logs, "No New Events found")
	fmt.Println("Events found")
	fmt.Println("logs:")
	fmt.Println(logs)

	for _, vLog := range logs {
		value, err := contractAbi.Unpack("NewTaskCreated", vLog.Data)
		if err != nil {
			fmt.Println("err")
			log.Fatal(err)
		} else {
			fmt.Println("value")
			fmt.Println(value)
		}

		// fmt.Println("topics")
		// var topics [4]string
		// for i := range vLog.Topics {
		// 	fmt.Println("topic")
		// 	topics[i] = vLog.Topics[i].Hex()
		// 	fmt.Println(topics[i])
		// }
		// if id == 0 {
		// 	assert.Equal(t, topics[1], 0x0, "Expected NewTaskCreated event with taskId 0")
		// } else if id == 1 {
		// 	assert.Equal(t, topics[1], 0x1, "Expected NewTaskCreated event with taskId 1")
		// }
	}

	assert.Equal(t, "0x0000000000000000000000000000000000000000000000000000000000000000", logs[0].Topics[1].Hex(), "Expected NewTaskCreated event with taskId 0")
	assert.Equal(t, "0x0000000000000000000000000000000000000000000000000000000000000001", logs[1].Topics[1].Hex(), "Expected NewTaskCreated event with taskId 0")

	// 	assert.NotEmpty(t, value)
	// 	fmt.Println("TaskCreated Event found? ::")
	// 	fmt.Println(t)

	// 	// fmt.Println(string(event.Key[:]))   // foo
	// 	// fmt.Println(string(event.Value[:])) // bar
	// }

	//check if aggregator send to operator

	//check if aggregator

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

func buildAggregator(t *testing.T, configFile string) *pkg.Aggregator {
	aggregatorConfig := config.NewAggregatorConfig(configFile)

	aggregator, err := pkg.NewAggregator(*aggregatorConfig)
	assert.Nil(t, err)

	return aggregator
}

func buildOperator(t *testing.T, configFile string) *operator.Operator {
	//TODO missing register Operator to Aggregator
	operatorConfig := config.NewOperatorConfig(configFile)

	opereator, err := operator.NewOperatorFromConfig(*operatorConfig)
	assert.Nil(t, err)

	return opereator
}
