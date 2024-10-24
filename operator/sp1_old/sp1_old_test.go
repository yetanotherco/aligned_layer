package sp1_old_test

import (
	"os"
	"testing"

	sp1 "github.com/yetanotherco/aligned_layer/operator/sp1_old"
)

const ProofFilePath = "../../scripts/test_files/sp1/sp1_fibonacci_old.proof"

const ElfFilePath = "../../scripts/test_files/sp1/sp1_fibonacci_old.elf"

func TestFibonacciSp1ProofVerifies(t *testing.T) {
	proofBytes, err := os.ReadFile(ProofFilePath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	elfBytes, err := os.ReadFile(ElfFilePath)
	if err != nil {
		t.Errorf("could not open elf file: %s", err)
	}

	if !sp1.VerifySp1ProofOld(proofBytes, elfBytes) {
		t.Errorf("proof did not verify")
	}
}
