package tests

import (
	"context"
	"github.com/stretchr/testify/assert"
	"github.com/yetanotherco/aligned_layer/aggregator/pkg"
	"github.com/yetanotherco/aligned_layer/core/config"
	operator "github.com/yetanotherco/aligned_layer/operator/pkg"
	"os"
	"testing"
	"fmt"
	"net/http"
	"bufio"
	"regexp"
)

func TestIntegration(t *testing.T) {
	fmt.Println("Running integration test")
	err := os.Chdir("../")
	assert.Nil(t, err, "Could not change directory to project root")

	configFilePath := "config-files/config-test.yaml"

	// check anvil is running
	result, anvilPort := checkAnvilIsRunning(configFilePath)
	if!result {
		t.Fatalf("Expected Anvil to be running, in port %s but it was not.", anvilPort)
	}

	// start aggregator
	aggregator := buildAggregator(t, configFilePath)
	go func() {
		err := aggregator.ServeOperators()
		assert.Nil(t, err, "Could not start aggregator")
	}()

	// start operator
	op := buildOperator(t, configFilePath)
	go func() {
		err := op.Start(context.Background())
		assert.Nil(t, err, "Could not start operator")
	}()

}

func checkAnvilIsRunning(configFilePath string) (bool, string) {
	file, err := os.Open(configFilePath)
	if err!= nil {
		fmt.Println("Error opening file:", err)
		return false, ""
	}
	defer file.Close()
	var port string
	portRegex := regexp.MustCompile(`eth_rpc_url: "http://localhost:(\d+)"`)

	// Read the file line by line
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		match := portRegex.FindStringSubmatch(line)
		if len(match) > 1 {
			port = match[1]
			fmt.Printf("Port number: %s\n", port)
			break 
		}
	}
	if err := scanner.Err(); err!= nil {
		fmt.Fprintln(os.Stderr, "Error reading file:", err)
	}

	_, errs := http.Get("http://localhost:" + port)
	if errs!= nil {
		return false, port
	}
	
	return true, port
}

func buildAggregator(t *testing.T, configFile string) *pkg.Aggregator {
	fmt.Println("Building Aggregator")
	// time.Sleep(5 * time.Second)
	aggregatorConfig := config.NewAggregatorConfig(configFile)

	fmt.Println("Building Aggregator2")

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
