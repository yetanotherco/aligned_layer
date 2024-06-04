package jolt_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/sp1"
)

const MaxProofSize = 2 * 1024 * 1024
const MaxInfoSize = 2 * 1024 * 1024

func TestFibonacciJoltProofVerifies(t *testing.T) {
	proofFile, err := os.Open("../../task_sender/test_examples/jolt/sha2chaingenerator/script/jolt.proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	infoFile, err := os.Open("../../task_sender/test_examples/jolt/sha2chaingenerator/script/jolt.info")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	infoBytes := make([]byte, MaxInfoSize)
	nReadInfoBytes, err := infoFile.Read(infoBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !sp1.VerifyJoltProof(proofBytes, uint32(nReadProofBytes), infoBytes, uint32(nReadInfoBytes)) {
		t.Errorf("proof did not verify")
	}
}
