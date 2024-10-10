package risc_zero_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/risc_zero"
)

func TestFibonacciRiscZeroProofVerifies(t *testing.T) {
	innerReceiptBytes, err := os.ReadFile("../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	imageIdBytes, err := os.ReadFile("../../scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id.bin")
	if err != nil {
		t.Errorf("could not open image id file: %s", err)
	}

	publicInputBytes, err := os.ReadFile("../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.pub")
	if err != nil {
		t.Errorf("could not open public input file: %s", err)
	}
	verified, err := risc_zero.VerifyRiscZeroReceipt(innerReceiptBytes, imageIdBytes, publicInputBytes)
	if err != nil || !verified {
		t.Errorf("proof did not verify")
	}
}
