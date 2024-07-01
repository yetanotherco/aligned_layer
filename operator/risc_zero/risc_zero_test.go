package risc_zero_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/risc_zero"
)

const MaxProofSize = 2 * 1024 * 1024
const MaxImageIdSize = 32

func TestFibonacciRiscZeroProofVerifies(t *testing.T) {
	receiptFile, err := os.Open("../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	receiptBytes := make([]byte, MaxProofSize)
	nReadReceiptBytes, err := receiptFile.Read(receiptBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	imageIdFile, err := os.Open("../../scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id.bin")
	if err != nil {
		t.Errorf("could not open image id file: %s", err)
	}
	imageIdBytes := make([]byte, MaxImageIdSize)
	nReadImageIdBytes, err := imageIdFile.Read(imageIdBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !risc_zero.VerifyRiscZeroReceipt(receiptBytes, uint32(nReadReceiptBytes), imageIdBytes, uint32(nReadImageIdBytes)) {
		t.Errorf("proof did not verify")
	}
}
