package risc_zero_test

import (
	"github.com/yetanotherco/aligned_layer/operator/risc_zero"
	"os"
	"strconv"
	"strings"
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

	imageId := getImageIdsFromFile(t, "../../task_sender/test_examples/risc_zero/fibonacci_proof_generator/fibonacci_id.txt")

	if !risc_zero.VerifyRiscZeroReceipt(([risc_zero.MaxReceiptSize]byte)(receiptBytes), uint(nReadReceiptBytes), ([risc_zero.MaxImageIdSize]uint32)(imageId)) {
		t.Errorf("proof did not verify")
	}
}

func getImageIdsFromFile(t *testing.T, filename string) []uint32 {
	data, err := os.ReadFile(filename)
	if err != nil {
		t.Errorf("could not open image id file: %s", err)
	}

	content := strings.TrimSpace(string(data))

	content = strings.TrimPrefix(content, "[")
	content = strings.TrimSuffix(content, "]")

	stringNumbers := strings.Split(content, ",")

	var imageId []uint32

	for _, strNum := range stringNumbers {
		strNum = strings.TrimSpace(strNum)

		num, err := strconv.ParseUint(strNum, 10, 32)
		if err != nil {
			t.Errorf("could not parse image id: %s", err)
		}
		imageId = append(imageId, uint32(num))
	}

	return imageId
}
