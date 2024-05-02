package tests

import (
	"bufio"
	"context"
	"fmt"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/stretchr/testify/assert"
	"github.com/yetanotherco/aligned_layer/aggregator/pkg"
	"github.com/yetanotherco/aligned_layer/core/config"
	operator "github.com/yetanotherco/aligned_layer/operator/pkg"
	"log"
	"math/big"
	"net/http"
	"os"

	"os/exec"
	"bytes"

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
	fmt.Println("alignedContractAddress: " + alignedContractAddress)

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

	// start operator
	op := buildOperator(t, configFilePath)
	go func() {
		err := op.Start(context.Background())
		assert.Nil(t, err, "Could not start operator")
	}()

	// send task
	var out bytes.Buffer
	cmd := exec.Command("make", "send-plonk-proof")
	cmd.Stdout = &out
	err = cmd.Run()
	if err != nil {
		log.Fatalf("cmd.Run() failed with %s\n", err)
	}
	fmt.Println("Task sent successfully")

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
	// contractAbi, err := abi.JSON(strings.NewReader(string(store.StoreABI)))
	a, err := getAlignedABI()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(a)

	fmt.Println("value")
	fmt.Print(logs)
	assert.NotEmpty(t, logs, "No NewTaskCreated Events found")
	// for _, vLog := range logs {
	// 	event := struct {
	// 		Key   [32]byte
	// 		Value [32]byte
	// 	}{}
	// 	value, err := contractAbi.Unpack("NewTaskCreated", vLog.Data)
	// 	if err != nil {
	// 		log.Fatal(err)
	// 	}

	// 	fmt.Println("value")
	// 	fmt.Println(value)

	// 	assert.NotEmpty(t, value)

	// 	fmt.Println(string(event.Key[:]))   // foo
	// 	fmt.Println(string(event.Value[:])) // bar
	// }

	//check if aggregator send to operator

	//check if aggregator

}

func getAlignedABI() (abi.ABI, error) {
	abiFilePath := "/Users/urix/aligned_layer/tests/AlignedLAyerServiceManager.json"

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
	operatorConfig := config.NewOperatorConfig(configFile)

	opereator, err := operator.NewOperatorFromConfig(*operatorConfig)
	assert.Nil(t, err)

	return opereator
}
