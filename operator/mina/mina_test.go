package mina_test

import (
	"fmt"
	"os"
	"testing"

	"github.com/lambdaclass/aligned_layer/operator/mina"
)

func TestMinaStateProofVerifies(t *testing.T) {
	fmt.Println(os.Getwd())
	proofFile, err := os.Open("../../batcher/aligned/test_files/mina/protocol_state.proof")
	if err != nil {
		t.Errorf("could not open mina state proof file")
	}

	proofBuffer := make([]byte, mina.MAX_PROOF_SIZE)
	proofLen, err := proofFile.Read(proofBuffer)
	if err != nil {
		t.Errorf("could not read bytes from mina state proof file")
	}

	pubInputFile, err := os.Open("../../batcher/aligned/test_files/mina/protocol_state.pub")
	if err != nil {
		t.Errorf("could not open mina state hash file")
	}
	pubInputBuffer := make([]byte, mina.MAX_PUB_INPUT_SIZE)
	pubInputLen, err := pubInputFile.Read(pubInputBuffer)
	if err != nil {
		t.Errorf("could not read bytes from mina state hash")
	}

	if !mina.VerifyProtocolStateProof(([mina.MAX_PROOF_SIZE]byte)(proofBuffer), uint(proofLen), ([mina.MAX_PUB_INPUT_SIZE]byte)(pubInputBuffer), uint(pubInputLen)) {
		t.Errorf("proof did not verify")
	}
}
