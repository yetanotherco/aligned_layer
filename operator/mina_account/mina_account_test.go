package mina_account_test

import (
	"fmt"
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/mina_account"
)

func TestMinaStateProofVerifies(t *testing.T) {
	fmt.Println(os.Getwd())
	proofFile, err := os.Open("../../batcher/aligned/test_files/mina/account_B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9.proof")
	if err != nil {
		t.Errorf("could not open mina account proof file")
	}

	proofBuffer := make([]byte, mina.MAX_PROOF_SIZE)
	proofLen, err := proofFile.Read(proofBuffer)
	if err != nil {
		t.Errorf("could not read bytes from mina account proof file")
	}

	pubInputFile, err := os.Open("../../batcher/aligned/test_files/mina/account_B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9.pub")
	if err != nil {
		t.Errorf("could not open mina account pub inputs file")
	}
	pubInputBuffer := make([]byte, mina.MAX_PUB_INPUT_SIZE)
	pubInputLen, err := pubInputFile.Read(pubInputBuffer)
	if err != nil {
		t.Errorf("could not read bytes from mina account pub inputs hash")
	}

	if !mina_account.VerifyAccountInclusion(([mina.MAX_PROOF_SIZE]byte)(proofBuffer), uint(proofLen), ([mina.MAX_PUB_INPUT_SIZE]byte)(pubInputBuffer), uint(pubInputLen)) {
		t.Errorf("proof did not verify")
	}
}
