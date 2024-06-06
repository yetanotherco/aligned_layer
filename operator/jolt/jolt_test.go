package jolt_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/sp1"
)

const MaxProofSize = 2 * 1024 * 1024
const MaxElfSize = 2 * 1024 * 1024
const MaxCommitmentSize = 2 * 1024 * 1024

func TestFibonacciJoltProofVerifies(t *testing.T) {
	proofFile, err := os.Open("../../task_sender/test_examples/jolt/sha2chaingenerator/jolt.proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	elfBytes, err := os.Open("../../task_sender/test_examples/jolt/sha2chaingenerator/jolt.elf")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	elfBytes := make([]byte, MaxInfoSize)
	nReadElfBytes, err := infoFile.Read(elfBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	commitmentBytes, err := os.Open("../../task_sender/test_examples/jolt/sha2chaingenerator/jolt.commitments")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	commitmentBytes := make([]byte, MaxInfoSize)
	nReadCommitmentBytes, err := infoFile.Read(commitmentBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !sp1.VerifyJoltProof(proofBytes, uint32(nReadProofBytes), elfBytes, uint32(nReadElfBytes), commitmentBytes, uint32(nReadCommitmentBytes)) {
		t.Errorf("proof did not verify")
	}
}
