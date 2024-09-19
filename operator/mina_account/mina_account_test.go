package mina_account_test

import (
	"fmt"
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/mina_account"
)

func TestMinaStateProofVerifies(t *testing.T) {
	fmt.Println(os.Getwd())
	proofFile, err := os.Open("../../scripts/test_files/mina_account/mina_account.proof")
	if err != nil {
		t.Errorf("could not open mina account proof file")
	}

	proofBuffer := make([]byte, mina_account.MAX_PROOF_SIZE)
	proofLen, err := proofFile.Read(proofBuffer)
	if err != nil {
		t.Errorf("could not read bytes from mina account proof file")
	}

	pubInputFile, err := os.Open("../../scripts/test_files/mina_account/mina_account.pub")
	if err != nil {
		t.Errorf("could not open mina account pub inputs file")
	}
	pubInputBuffer := make([]byte, mina_account.MAX_PUB_INPUT_SIZE)
	pubInputLen, err := pubInputFile.Read(pubInputBuffer)
	if err != nil {
		t.Errorf("could not read bytes from mina account pub inputs hash")
	}

	if !mina_account.VerifyAccountInclusion(([mina_account.MAX_PROOF_SIZE]byte)(proofBuffer), uint(proofLen), ([mina_account.MAX_PUB_INPUT_SIZE]byte)(pubInputBuffer), uint(pubInputLen)) {
		t.Errorf("proof did not verify")
	}
}

func TestEmptyMinaStateProofDoesNotVerify(t *testing.T) {
	fmt.Println(os.Getwd())
	proofBuffer := make([]byte, mina_account.MAX_PROOF_SIZE)

	pubInputFile, err := os.Open("../../scripts/test_files/mina_account/mina_account.pub")
	if err != nil {
		t.Errorf("could not open mina account pub inputs file")
	}
	pubInputBuffer := make([]byte, mina_account.MAX_PUB_INPUT_SIZE)
	pubInputLen, err := pubInputFile.Read(pubInputBuffer)
	if err != nil {
		t.Errorf("could not read bytes from mina account pub inputs hash")
	}

	if mina_account.VerifyAccountInclusion(([mina_account.MAX_PROOF_SIZE]byte)(proofBuffer), mina_account.MAX_PROOF_SIZE, ([mina_account.MAX_PUB_INPUT_SIZE]byte)(pubInputBuffer), uint(pubInputLen)) {
		t.Errorf("Empty proof should not verify but it did")
	}
}

func TestMinaStateProofWithEmptyPubInputDoesNotVerify(t *testing.T) {
	fmt.Println(os.Getwd())
	proofFile, err := os.Open("../../scripts/test_files/mina_account/mina_account.proof")
	if err != nil {
		t.Errorf("could not open mina account proof file")
	}

	proofBuffer := make([]byte, mina_account.MAX_PROOF_SIZE)
	proofLen, err := proofFile.Read(proofBuffer)
	if err != nil {
		t.Errorf("could not read bytes from mina account proof file")
	}

	pubInputBuffer := make([]byte, mina_account.MAX_PUB_INPUT_SIZE)

	if mina_account.VerifyAccountInclusion(([mina_account.MAX_PROOF_SIZE]byte)(proofBuffer), uint(proofLen), ([mina_account.MAX_PUB_INPUT_SIZE]byte)(pubInputBuffer), mina_account.MAX_PUB_INPUT_SIZE) {
		t.Errorf("proof with empty public input should not verify but id did")
	}
}
