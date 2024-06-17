package nexus_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/nexus"
)

const MaxProofSize = 2 * 1024 * 1024
const MaxParamsSize = 2 * 1024 * 1024
const MaxKeySize = 2 * 1024 * 1024

func TestFibonacciNexusProofVerifies(t *testing.T) {
	proofFile, err := os.Open("../../task_sender/test_examples/nexus/fib/nexus.proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	
	paramsFile, err := os.Open("../../task_sender/test_examples/nexus/fib/nexus.params")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	paramsBytes := make([]byte, MaxParamsSize)
	nReadParamsBytes, err := paramsFile.Read(paramsBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	keyFile, err := os.Open("../../task_sender/test_examples/nexus/fib/nexus.key")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	keyBytes := make([]byte, MaxKeySize)
	nReadKeyBytes, err := keyFile.Read(keyBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !nexus.VerifyNexusProof(proofBytes, uint32(nReadProofBytes), paramsBytes, uint32(nReadParamsBytes), keyBytes, uint32(nReadKeyBytes)) {
		t.Errorf("proof did not verify")
	}
}
