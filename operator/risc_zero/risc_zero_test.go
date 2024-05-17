package risc_zero_test

import (
	"github.com/yetanotherco/aligned_layer/operator/risc_zero"
	"os"
	"testing"
)

func TestFibonacciRiscZeroProofVerifies(t *testing.T) {
	receiptFile, err := os.Open("../../task_sender/test_examples/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	receiptBytes := make([]byte, risc_zero.MaxReceiptSize)
	nReadReceiptBytes, err := receiptFile.Read(receiptBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	imageId := []uint32{2168696514, 4069298130, 1005557306, 3274294743, 1735077096, 3539040653, 808254153, 306297660}

	if !risc_zero.VerifyRiscZeroReceipt(([risc_zero.MaxReceiptSize]byte)(receiptBytes), uint(nReadReceiptBytes), ([risc_zero.MaxImageIdSize]uint32)(imageId)) {
		t.Errorf("proof did not verify")
	}
}
