package jolt_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/jolt"
)

const MaxProofSize = 4 * 1024 * 1024
const MaxElfSize = 2 * 1024 * 1024
const MaxCommitmentSize = 2 * 1024 * 1024

func TestFibonacciJoltProofVerifies(t *testing.T) {
	proofFile, err := os.Open("../../task_sender/test_examples/jolt/fib_e2e/jolt.proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	elfFile, err := os.Open("../../task_sender/test_examples/jolt/fib_e2e/jolt.elf")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	elfBytes := make([]byte, MaxElfSize)
	nReadElfBytes, err := elfFile.Read(elfBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	commitmentFile, err := os.Open("../../task_sender/test_examples/jolt/fib_e2e/jolt.commitment")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	commitmentBytes := make([]byte, MaxCommitmentSize)
	nReadCommitmentBytes, err := commitmentFile.Read(commitmentBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !jolt.VerifyJoltProof(proofBytes, uint32(nReadProofBytes), elfBytes, uint32(nReadElfBytes), commitmentBytes, uint32(nReadCommitmentBytes)) {
		t.Errorf("proof did not verify")
	}
}

func TestSha3JoltProofVerifies(t *testing.T) {
	proofFile, err := os.Open("../../task_sender/test_examples/jolt/sha3_e2e/jolt.proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	elfFile, err := os.Open("../../task_sender/test_examples/jolt/sha3_e2e/jolt.elf")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	elfBytes := make([]byte, MaxElfSize)
	nReadElfBytes, err := elfFile.Read(elfBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	commitmentFile, err := os.Open("../../task_sender/test_examples/jolt/sha3_e2e/jolt.commitment")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	commitmentBytes := make([]byte, MaxCommitmentSize)
	nReadCommitmentBytes, err := commitmentFile.Read(commitmentBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !jolt.VerifyJoltProof(proofBytes, uint32(nReadProofBytes), elfBytes, uint32(nReadElfBytes), commitmentBytes, uint32(nReadCommitmentBytes)) {
		t.Errorf("proof did not verify")
	}
}
