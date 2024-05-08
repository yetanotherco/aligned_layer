package sp1_test

import (
	"fmt"
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/sp1"
)

func TestFibonacciSp1ProofVerifies(t *testing.T) {
	fmt.Println(os.Getwd())
	f, err := os.Open("../../task_sender/test_example/sp1/sp1_fibonacci.proof")
	if err != nil {
		t.Errorf("could not open proof file")
	}

	proofBytes := make([]byte, sp1.MAX_PROOF_SIZE)
	nReadBytes, err := f.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !sp1.VerifySp1Proof(([sp1.MAX_PROOF_SIZE]byte)(proofBytes), uint(nReadBytes)) {
		t.Errorf("proof did not verify")
	}
}
