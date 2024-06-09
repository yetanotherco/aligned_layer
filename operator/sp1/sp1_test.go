package sp1_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/sp1"
)

const MaxProofSize = 2 * 1024 * 1024
const MaxElfSize = 2 * 1024 * 1024

func TestFibonacciSp1ProofVerifies(t *testing.T) {
	proofFile, err := os.Open("../../task_sender/test_examples/sp1/sp1_fibonacci.proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	elfFile, err := os.Open("../../task_sender/test_examples/sp1/elf")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	elfBytes := make([]byte, MaxElfSize)
	nReadElfBytes, err := elfFile.Read(elfBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !sp1.VerifySp1Proof(proofBytes, uint32(nReadProofBytes), elfBytes, uint32(nReadElfBytes)) {
		t.Errorf("proof did not verify")
	}
}
