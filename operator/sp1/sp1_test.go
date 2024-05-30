package sp1_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/sp1"
)

const MaxProofSize = 2 * 1024 * 1024
const MaxElfSize = 2 * 1024 * 1024

func TestFibonacciSp1ProofVerifies(t *testing.T) {
	t.Log("TestFibonacciSp1ProofVerifies")
	proofFile, err := os.Open("../../task_sender/test_examples/sp1/fibonacci_proof_generator/script/sp1_fibonacci.proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	t.Log("Opened proof file")
	proofBytes := make([]byte, MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	t.Log("Read proof bytes")

	elfFile, err := os.Open("../../task_sender/test_examples/sp1/fibonacci_proof_generator/program/elf/riscv32im-succinct-zkvm-elf")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	t.Log("Opened elf file")

	elfBytes := make([]byte, MaxElfSize)
	nReadElfBytes, err := elfFile.Read(elfBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	t.Log("Read elf bytes")

	if !sp1.VerifySp1Proof(proofBytes, uint(nReadProofBytes), elfBytes, uint(nReadElfBytes)) {
		t.Errorf("proof did not verify")
	}

	t.Log("Proof verified")
}
