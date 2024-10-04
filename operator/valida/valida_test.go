package valida_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/valida"
)

func TestCatValidaProofVerifies(t *testing.T) {
	proofBytes, err := os.ReadFile("../../scripts/test_files/valida/cat/cat.proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	programCodeBytes, err := os.ReadFile("../../scripts/test_files/valida/cat/cat.bin")
	if err != nil {
		t.Errorf("could not open program code file: %s", err)
	}

	if !valida.VerifyValidaProof(proofBytes, programCodeBytes) {
		t.Errorf("proof did not verify")
	}
}
