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

	imageId := []uint32{3090655438, 2953112184, 965953788, 2757110989, 1044116726, 4262054234, 2330742163, 3902204400}

	if !risc_zero.VerifyRiscZeroReceipt(([risc_zero.MaxReceiptSize]byte)(receiptBytes), uint(nReadReceiptBytes), ([risc_zero.MaxImageIdSize]uint32)(imageId)) {
		t.Errorf("proof did not verify")
	}
}
