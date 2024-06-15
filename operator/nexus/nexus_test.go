package nexus_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/nexus"
)

const MaxProofsize = 2 * 1024 * 1024
const MaxParamsSize = 2 * 1024 * 1024
const MaxInputSize = 2 * 1024 * 1024
const MaxElfSize = 2 * 1024 * 1024

func TestFibonacciNexusProofVerifies(t *testing.T) {
	proofFile, err := os.Open("../../task_sender/test_examples/nexus/fib/nexus.proof")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	
	inputFile, err := os.Open("../../task_sender/test_examples/nexus/fib/nexus.input")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	inputBytes := make([]byte, MaxInputSize)
	nReadInputBytes, err := paramsFile.Read(inputBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	paramsFile, err := os.Open("../../task_sender/test_examples/nexus/fib/nexus.params")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	paramsBytes := make([]byte, MaxParamsSize)
	nReadParamsBytes, err := paramsFile.Read(paramsBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	elfFile, err := os.Open("../../task_sender/test_examples/nexus/fib/nexus.elf")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	elfBytes := make([]byte, MaxElfSize)
	nReadElfBytes, err := elfFile.Read(elfBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !sp1.VerifyNexusProof(proofBytes, uint32(nReadProofBytes), paramsBytes, uint32(nReadParamsBytes), inputBytes, uint32(nReadInputBytes), elfBytes, uint32(nReadElfBytes)) {
		t.Errorf("proof did not verify")
	}
}
