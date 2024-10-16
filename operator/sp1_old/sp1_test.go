package sp1_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/sp1"
)

const ProofFilePath = "../../scripts/test_files/sp1_old/sp1_fibonacci.proof"

const ElfFilePath = "../../scripts/test_files/sp1_old/sp1_fibonacci.elf"

func TestFibonacciSp1ProofVerifies(t *testing.T) {
	proofBytes, err := os.ReadFile(ProofFilePath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	elfBytes, err := os.ReadFile(ElfFilePath)
	if err != nil {
		t.Errorf("could not open elf file: %s", err)
	}

	if !sp1.VerifySp1Proof(proofBytes, elfBytes) {
		t.Errorf("proof did not verify")
	}
}
