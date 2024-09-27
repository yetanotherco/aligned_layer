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

type Telemetry struct {
	client  *http.Client
	baseURL *url.URL
	logger  logging.Logger
}

func NewTelemetry(serverAddress string, logger logging.Logger) *Telemetry {
	client := &http.Client{}

	baseURL := &url.URL{
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

// Initializes a new trace for the given batchMerkleRoot.
// User must call FinishTrace() to complete the trace.
func (t *Telemetry) InitNewTrace(batchMerkleRoot [32]byte) {
	merkleRootString := hex.EncodeToString(batchMerkleRoot[:])
	body := TraceMessage{
		MerkleRoot: fmt.Sprintf("0x%s", merkleRootString),
	}
	encodedBody, err := json.Marshal(body)
	if err != nil {
		t.logger.Error("[Telemetry] Error marshalling JSON: %v", err)
		return
	}
	t.logger.Info("[Telemetry] Sending init task trace.", "merkle_root", body.MerkleRoot)
	if err := t.sendAndReceiveResponse("/api/initTaskTrace", encodedBody); err != nil {

		t.logger.Error("[Telemetry] Error sending init task trace: %v", err)
	}
}

// Logs an operator response.
// The response is added as an event in the corresponding trace.
func (t *Telemetry) LogOperatorResponse(batchMerkleRoot [32]byte, operatorId [32]byte) {
	merkleRootString := hex.EncodeToString(batchMerkleRoot[:])
	operatorIdString := hex.EncodeToString(operatorId[:])
	body := OperatorResponseMessage{
		MerkleRoot: fmt.Sprintf("0x%s", merkleRootString),
		OperatorId: fmt.Sprintf("0x%s", operatorIdString),
	}
	encodedBody, err := json.Marshal(body)
	if err != nil {
		t.logger.Error("[Telemetry] Error marshalling JSON: %v", err)
		return
	}
	t.logger.Info("[Telemetry] Sending operator response.", "merkle_root", body.MerkleRoot, "operator_resonse", body.OperatorId)
	if err := t.sendAndReceiveResponse("/api/operatorResponse", encodedBody); err != nil {

		t.logger.Error("[Telemetry] Error sending operator response: %v", err)
	}
}

// Logs quorum reached.
// Emits an event in the corresponding trace.
func (t *Telemetry) LogQuorumReached(batchMerkleRoot [32]byte) {
	merkleRootString := hex.EncodeToString(batchMerkleRoot[:])
	body := QuorumReachedMessage{
		MerkleRoot: fmt.Sprintf("0x%s", merkleRootString),
	}
	encodedBody, err := json.Marshal(body)
	if err != nil {
		t.logger.Error("[Telemetry] Error marshalling JSON: %v", err)
		return
	}
	t.logger.Info("[Telemetry] Logging quorum reached", "merkle_root", body.MerkleRoot)
	if err := t.sendAndReceiveResponse("/api/quorumReached", encodedBody); err != nil {

		t.logger.Error("[Telemetry] Error sending QuorumReached: %v", err)
	}
}

// Finishes the trace for the given merkle root and frees resources
// In order to wait for all operators responses, even if the quorum is reached, this function has a delayed execution
func (t *Telemetry) FinishTrace(batchMerkleRoot [32]byte) {
	go func() {
		time.Sleep(10 * time.Second)
		merkleRootString := hex.EncodeToString(batchMerkleRoot[:])
		body := TraceMessage{
			MerkleRoot: fmt.Sprintf("0x%s", merkleRootString),
		}
		encodedBody, err := json.Marshal(body)
		if err != nil {
			t.logger.Error("[Telemetry] Error marshalling JSON: %v", err)
			return
		}
		t.logger.Info("[Telemetry] Sending finish task trace.", "merkle_root", body.MerkleRoot)

		if err := t.sendAndReceiveResponse("/api/finishTaskTrace", encodedBody); err != nil {

			t.logger.Error("[Telemetry] Error finishing trace: %v", err)
		}
	}()
}

// Sends a POST request and processes the response.
func (t *Telemetry) sendAndReceiveResponse(endpoint string, body []byte) error {

	url := t.baseURL.ResolveReference(&url.URL{Path: endpoint})
	resp, err := t.client.Post(url.String(), "application/json", bytes.NewBuffer(body))
	if err != nil {
		return fmt.Errorf("error making POST request: %w", err)
	}
	defer resp.Body.Close()

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return fmt.Errorf("error reading response body: %w", err)
	}

	t.logger.Info("[Telemetry] Response Status", "status", resp.Status, "response_body", string(respBody))

	return nil
}
