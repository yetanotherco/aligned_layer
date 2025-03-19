package pkg

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"time"

	"github.com/Layr-Labs/eigensdk-go/logging"
)

type TraceMessage struct {
	MerkleRoot string `json:"merkle_root"`
}

type OperatorResponseMessage struct {
	MerkleRoot string `json:"merkle_root"`
	OperatorId string `json:"operator_id"`
}
type QuorumReachedMessage struct {
	MerkleRoot string `json:"merkle_root"`
}

type TaskErrorMessage struct {
	MerkleRoot string `json:"merkle_root"`
	TaskError  string `json:"error"`
}

type TaskSetGasPriceMessage struct {
	MerkleRoot string `json:"merkle_root"`
	GasPrice   string `json:"gas_price"`
}

type TaskSentToEthereumMessage struct {
	MerkleRoot        string `json:"merkle_root"`
	TxHash            string `json:"tx_hash"`
	EffectiveGasPrice string `json:"effective_gas_price"`
}

type Telemetry struct {
	client  http.Client
	baseURL url.URL
	logger  logging.Logger
}

func NewTelemetry(serverAddress string, logger logging.Logger) *Telemetry {
	client := http.Client{}

	baseURL := url.URL{
		Scheme: "http",
		Host:   serverAddress,
	}
	logger.Info("[Telemetry] Starting Telemetry client.", "server_address",
		serverAddress)

	return &Telemetry{
		client:  client,
		baseURL: baseURL,
		logger:  logger,
	}
}

func (t *Telemetry) InitNewTrace(batchMerkleRoot [32]byte) {
	body := TraceMessage{
		MerkleRoot: fmt.Sprintf("0x%s", hex.EncodeToString(batchMerkleRoot[:])),
	}
	if err := t.sendTelemetryMessage("/api/initTaskTrace", body); err != nil {
		t.logger.Warn("[Telemetry] Error in InitNewTrace", "error", err)
	}
}

func (t *Telemetry) LogOperatorResponse(batchMerkleRoot [32]byte, operatorId [32]byte) {
	body := OperatorResponseMessage{
		MerkleRoot: fmt.Sprintf("0x%s", hex.EncodeToString(batchMerkleRoot[:])),
		OperatorId: fmt.Sprintf("0x%s", hex.EncodeToString(operatorId[:])),
	}
	if err := t.sendTelemetryMessage("/api/operatorResponse", body); err != nil {
		t.logger.Warn("[Telemetry] Error in LogOperatorResponse", "error", err)
	}
}

func (t *Telemetry) LogQuorumReached(batchMerkleRoot [32]byte) {
	body := QuorumReachedMessage{
		MerkleRoot: fmt.Sprintf("0x%s", hex.EncodeToString(batchMerkleRoot[:])),
	}
	if err := t.sendTelemetryMessage("/api/quorumReached", body); err != nil {
		t.logger.Warn("[Telemetry] Error in LogQuorumReached", "error", err)
	}
}

func (t *Telemetry) LogTaskError(batchMerkleRoot [32]byte, taskError error) {
	body := TaskErrorMessage{
		MerkleRoot: fmt.Sprintf("0x%s", hex.EncodeToString(batchMerkleRoot[:])),
		TaskError:  taskError.Error(),
	}
	if err := t.sendTelemetryMessage("/api/taskError", body); err != nil {
		t.logger.Warn("[Telemetry] Error in LogTaskError", "error", err)
	}
}

func (t *Telemetry) TaskSetGasPrice(batchMerkleRoot [32]byte, gasPrice string) {
	body := TaskSetGasPriceMessage{
		MerkleRoot: fmt.Sprintf("0x%s", hex.EncodeToString(batchMerkleRoot[:])),
		GasPrice:   gasPrice,
	}
	if err := t.sendTelemetryMessage("/api/aggregatorTaskSetGasPrice", body); err != nil {
		t.logger.Warn("[Telemetry] Error in LogOperatorResponse", "error", err)
	}
}

func (t *Telemetry) TaskSentToEthereum(batchMerkleRoot [32]byte, txHash string, effectiveGasPrice string) {
	body := TaskSentToEthereumMessage{
		MerkleRoot:        fmt.Sprintf("0x%s", hex.EncodeToString(batchMerkleRoot[:])),
		TxHash:            txHash,
		EffectiveGasPrice: effectiveGasPrice,
	}
	if err := t.sendTelemetryMessage("/api/aggregatorTaskSent", body); err != nil {
		t.logger.Warn("[Telemetry] Error in TaskSentToEthereum", "error", err)
	}
}

func (t *Telemetry) FinishTrace(batchMerkleRoot [32]byte) {
	// In order to wait for all operator responses, even if the quorum is reached, this function has a delayed execution
	go func() {
		time.Sleep(10 * time.Second)
		body := TraceMessage{
			MerkleRoot: fmt.Sprintf("0x%s", hex.EncodeToString(batchMerkleRoot[:])),
		}
		if err := t.sendTelemetryMessage("/api/finishTaskTrace", body); err != nil {
			t.logger.Warn("[Telemetry] Error in FinishTrace", "error", err)
		}
	}()
}

func (t *Telemetry) sendTelemetryMessage(endpoint string, message interface{}) error {
	encodedBody, err := json.Marshal(message)
	if err != nil {
		t.logger.Warn("[Telemetry] Error marshalling JSON", "error", err)
		return fmt.Errorf("error marshalling JSON: %w", err)
	}

	t.logger.Info("[Telemetry] Sending message.", "endpoint", endpoint, "message", message)

	fullURL := t.baseURL.ResolveReference(&url.URL{Path: endpoint})

	resp, err := t.client.Post(fullURL.String(), "application/json", bytes.NewBuffer(encodedBody))
	if err != nil {
		t.logger.Warn("[Telemetry] Error sending POST request", "error", err)
		return fmt.Errorf("error making POST request: %w", err)
	}
	defer resp.Body.Close()

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		t.logger.Warn("[Telemetry] Error reading response body", "error", err)
		return fmt.Errorf("error reading response body: %w", err)
	}

	t.logger.Info("[Telemetry] Response received", "status", resp.Status, "response_body", string(respBody))

	return nil
}
