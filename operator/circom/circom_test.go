package circom_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/circom"
)

func TestCircomProofVerifies(t *testing.T) {
	proofBytes, err := os.ReadFile("../../scripts/test_files/circom/proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	keyBytes, err := os.ReadFile("../../scripts/test_files/circom/key")
	if err != nil {
		t.Errorf("could not open image id file: %s", err)
	}

	publicInputBytes, err := os.ReadFile("../../scripts/test_files/circom/public_inputs")
	if err != nil {
		t.Errorf("could not open public input file: %s", err)
	}

	if !circom.VerifyCircomProof([8192]byte(proofBytes), uint32(len(proofBytes)), [1024]byte(keyBytes), uint32(len(keyBytes)), publicInputBytes, uint32(len(publicInputBytes))) {
		t.Errorf("proof did not verify")
	}
}
