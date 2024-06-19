package nexus_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/nexus"
)

const MaxProofSize = 64 * 1024 * 1024
const MaxParamsSize = 64 * 1024 * 1024

func TestFibonacciNexusProofVerifies(t *testing.T) {
	proofFile, err := os.Open("../../task_sender/test_examples/nexus/fib/nexus-proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	
	paramsFile, err := os.Open("../../task_sender/test_examples/nexus/fib/target/nexus-cache/nexus-public-seq-16.zst")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	paramsBytes := make([]byte, MaxParamsSize)
	nReadParamsBytes, err := paramsFile.Read(paramsBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !nexus.VerifyNexusProof(proofBytes, uint32(nReadProofBytes), paramsBytes, uint32(nReadParamsBytes)) {
		t.Errorf("proof did not verify")
	}
}
