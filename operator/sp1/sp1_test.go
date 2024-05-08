package sp1_test

import (
	"fmt"
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/sp1"
)

func TestFibonacciSp1ProofVerifies(t *testing.T) {
	fmt.Println(os.Getwd())
	proofFile, err := os.Open("../../task_sender/test_examples/sp1/sp1_fibonacci.proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, sp1.MAX_PROOF_SIZE)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	elfFile, err := os.Open("../../task_sender/test_examples/sp1/riscv32im-succinct-zkvm-elf")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	elfBytes := make([]byte, sp1.MAX_ELF_BUFFER_SIZE)
	nReadElfBytes, err := elfFile.Read(elfBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !sp1.VerifySp1Proof(([sp1.MAX_PROOF_SIZE]byte)(proofBytes), uint(nReadProofBytes), ([sp1.MAX_ELF_BUFFER_SIZE]byte)(elfBytes), uint(nReadElfBytes)) {
		t.Errorf("proof did not verify")
	}
}
