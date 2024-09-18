package sp1_test

import (
	"io"
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/sp1"
)

const ProofFile = "../../scripts/test_files/sp1/sp1_fibonacci.proof"
const ElfFile = "../../scripts/test_files/sp1/sp1_fibonacci.elf"

func TestFibonacciSp1ProofVerifies(t *testing.T) {
	proofFile, err := os.Open(ProofFile)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes, err := io.ReadAll(proofFile)
	if err != nil {
		t.Fatalf("Error reading batch file: %v", err)
	}

	elfFile, err := os.Open(ElfFile)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	elfBytes, err := io.ReadAll(elfFile)
	if err != nil {
		t.Fatalf("Error reading batch file: %v", err)
	}

	if !sp1.VerifySp1Proof(proofBytes, uint32(len(proofBytes)), elfBytes, uint32(len(elfBytes))) {
		t.Errorf("proof did not verify")
	}
}
