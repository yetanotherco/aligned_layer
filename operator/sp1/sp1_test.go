package sp1_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/sp1"
)

const ProofFilePath = "../../scripts/test_files/sp1/sp1_fibonacci_new.proof"

const ElfFilePath = "../../scripts/test_files/sp1/sp1_fibonacci_new.elf"

func TestFibonacciSp1ProofVerifies(t *testing.T) {
	proofBytes, err := os.ReadFile(ProofFilePath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	elfBytes, err := os.ReadFile(ElfFilePath)
	if err != nil {
		t.Errorf("could not open elf file: %s", err)
	}

	verified, err := sp1.VerifySp1Proof(proofBytes, elfBytes)
	if err != nil || !verified {
		t.Errorf("proof did not verify")
	}
}
