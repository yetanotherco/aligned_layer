package connection_test

import (
	"context"
	"fmt"
	"log"
	"math/big"
	"os"
	"os/exec"
	"strconv"
	"syscall"
	"testing"
	"time"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	rpccalls "github.com/Layr-Labs/eigensdk-go/metrics/collectors/rpc_calls"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/stretchr/testify/assert"
	connection "github.com/yetanotherco/aligned_layer/core"
	"github.com/yetanotherco/aligned_layer/core/utils"
)

func DummyFunction(x uint64) (uint64, error) {
	fmt.Println("Starting Anvil on Port ")
	if x == 42 {
		return 0, connection.PermanentError{Inner: fmt.Errorf("Permanent error!")}
	} else if x < 42 {
		return 0, fmt.Errorf("Transient error!")
	}
	return x, nil
}

func TestRetryWithData(t *testing.T) {
	function := func() (*uint64, error) {
		x, err := DummyFunction(43)
		return &x, err
	}
	data, err := connection.RetryWithData(function, 1000, 2, 3)
	if err != nil {
		t.Errorf("Retry error!: %s", err)
	} else {
		fmt.Printf("DATA: %d\n", data)
	}
}

func TestRetry(t *testing.T) {
	function := func() error {
		_, err := DummyFunction(43)
		return err
	}
	err := connection.Retry(function, 1000, 2, 3)
	if err != nil {
		t.Errorf("Retry error!: %s", err)
	}
}

/*
Starts an anvil instance via the cli.
Assumes that anvil is installed but checks.
*/
func SetupAnvil(port uint16) (*exec.Cmd, *eth.InstrumentedClient, error) {

	path, err := exec.LookPath("anvil")
	if err != nil {
		fmt.Printf("Could not find `anvil` executable in `%s`\n", path)
	}

	port_str := strconv.Itoa(int(port))
	http_rpc_url := fmt.Sprintf("http://localhost:%d", port)

	// Create a command
	cmd := exec.Command("anvil", "--port", port_str, "--load-state", "../contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json", "--block-time", "7")
	cmd.SysProcAttr = &syscall.SysProcAttr{Setpgid: true}

	// Run the command
	err = cmd.Start()
	if err != nil {
		fmt.Printf("Error: %s\n", err)
	}

	// Delay needed for anvil to start
	time.Sleep(1 * time.Second)

	reg := prometheus.NewRegistry()
	rpcCallsCollector := rpccalls.NewCollector("ethRpc", reg)
	ethRpcClient, err := eth.NewInstrumentedClient(http_rpc_url, rpcCallsCollector)
	if err != nil {
		log.Fatal("Error initializing eth rpc client: ", err)
	}

	return cmd, ethRpcClient, nil
}

func TestAnvilSetupKill(t *testing.T) {
	// Start Anvil
	cmd, _, err := SetupAnvil(8545)
	if err != nil {
		log.Fatal("Error setting up Anvil: ", err)
	}

	// Get Anvil PID
	pid := cmd.Process.Pid
	p, err := os.FindProcess(pid)
	if err != nil {
		log.Fatal("Error finding Anvil Process: ", err)
	}
	err = p.Signal(syscall.Signal(0))
	assert.Nil(t, err, "Anvil Process Killed")

	// Kill Anvil
	if err := cmd.Process.Kill(); err != nil {
		fmt.Printf("Error killing process: %v\n", err)
		return
	}

	// Check that PID is not currently present in running processes.
	// FindProcess always succeeds so on Unix systems we call it below.
	p, err = os.FindProcess(pid)
	if err != nil {
		log.Fatal("Error finding Anvil Process: ", err)
	}
	// Ensure process has exited
	err = p.Signal(syscall.Signal(0))

	assert.Nil(t, err, "Anvil Process Killed")
}

// |--Aggreagator Retry Tests--|

// Waits for receipt from anvil node -> Will fail to get receipt
func TestWaitForTransactionReceiptRetryable(t *testing.T) {

	// Retry call Params
	to := common.BytesToAddress([]byte{0x11})
	tx := types.NewTx(&types.AccessListTx{
		ChainID:  big.NewInt(1337),
		Nonce:    1,
		GasPrice: big.NewInt(11111),
		Gas:      1111,
		To:       &to,
		Value:    big.NewInt(111),
		Data:     []byte{0x11, 0x11, 0x11},
	})

	ctx := context.WithoutCancel(context.Background())

	hash := tx.Hash()

	// Start anvil
	cmd, client, err := SetupAnvil(8545)
	if err != nil {
		fmt.Printf("Error setting up Anvil: %s\n", err)
	}

	// Assert Call succeeds why Anvil running
	_, err = utils.WaitForTransactionReceiptRetryable(*client, ctx, hash)
	assert.NotNil(t, err, "Error Waiting for Transaction with Anvil Running: %s\n", err)
	if err.Error() != "not found" {
		fmt.Printf("WaitForTransactionReceipt Emitted incorrect error: %s\n", err)
		return
	}

	// Kill Anvil
	if err := cmd.Process.Kill(); err != nil {
		fmt.Printf("Error killing process: %v\n", err)
		return
	}

	// Fails
	receipt, err := utils.WaitForTransactionReceiptRetryable(*client, ctx, hash)
	assert.Nil(t, receipt, "Receipt not empty")
	assert.NotEqual(t, err.Error(), "not found")

	// Start anvil
	_, client, err = SetupAnvil(8545)
	if err != nil {
		fmt.Printf("Error setting up Anvil: %s\n", err)
	}

	_, err = utils.WaitForTransactionReceiptRetryable(*client, ctx, hash)
	assert.NotNil(t, err, "Call to Anvil failed")
	if err.Error() != "not found" {
		fmt.Printf("WaitForTransactionReceipt Emitted incorrect error: %s\n", err)
	}
}

/*

func TestSendAggregatedResponseRetryable(t *testing.T) {
}

func TestInitializeNewTaskRetryable(t *testing.T) {
	//TODO: Instantiate Aggregator
}

// |--Server Retry Tests--|
func TestProcessNewSignatureRetryable(t *testing.T) {
		agg := NewAggregator()
		agg.ProcessNewSignatureRetryable()
}

// |--Subscriber Retry Tests--|

func TestSubscribeToNewTasksV3Retryable(t *testing.T) {
		newBatchChan := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3)

		baseConfig := core.NewBaseConfig("")
		avsSubscriber, err := chainio.NewAvsSubscriberFromConfig(baseConfig)
		if err != nil {
			return nil, err
		}

		agg.taskSubscriber, err = avsSubscriber.SubscribeToNewTasksV3Retryable(newBatchChan)
}

// |--AVS-Writer Retry Tests--|

func TestRespondToTaskV2(t *testing.T) {
}

func TestBatchesStateWriter(t *testing.T) {
}

func TestBalanceAt(t *testing.T) {
}

func TestBatchersBalances(t *testing.T) {
}

// |--AVS-Subscriber Retry Tests--|

func TestSubscribeToNewTasksV2(t *testing.T) {
}

func TestSubscribeToNewTasksV3(t *testing.T) {
}

func TestBlockNumber(t *testing.T) {
}

func TestFilterBatchV2(t *testing.T) {
}

func TestFilterBatchV3(t *testing.T) {
}

func TestBatchesStateSubscriber(t *testing.T) {
}

func TestSubscribeNewHead(t *testing.T) {
}
*/
