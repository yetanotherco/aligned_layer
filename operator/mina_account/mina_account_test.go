package mina_account_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/mina_account"
)

const ProofFilePath = "../../scripts/test_files/mina_account/mina_account.proof"

const PubInputFilePath = "../../scripts/test_files/mina_account/mina_account.pub"

func TestMinaStateProofVerifies(t *testing.T) {
	proofBytes, err := os.ReadFile(ProofFilePath)
	if err != nil {
		t.Errorf("could not open mina account proof file")
	}

	pubInputBytes, err := os.ReadFile(PubInputFilePath)
	if err != nil {
		t.Errorf("could not open mina account pub input file")
	}

	verified, err := mina_account.VerifyAccountInclusion(proofBytes, pubInputBytes)
	if err != nil || !verified {
		t.Errorf("proof did not verify")
	}
}

func TestEmptyMinaStateProofDoesNotVerify(t *testing.T) {
	proofBytes, err := os.ReadFile(ProofFilePath)
	if err != nil {
		t.Errorf("could not open mina state proof file")
	}
	emptyProofBuffer := make([]byte, len(proofBytes))

	pubInputBytes, err := os.ReadFile(PubInputFilePath)
	if err != nil {
		t.Errorf("could not open mina state pub input file")
	}

	verified, err := mina_account.VerifyAccountInclusion(emptyProofBuffer, pubInputBytes)
	if err != nil {
		t.Errorf("verification failed with error")
	}
	if verified {
		t.Errorf("proof should not verify")
	}
}

func TestMinaStateProofWithEmptyPubInputDoesNotVerify(t *testing.T) {
	proofBytes, err := os.ReadFile(ProofFilePath)
	if err != nil {
		t.Errorf("could not open mina state proof file")
	}

	pubInputBytes, err := os.ReadFile(PubInputFilePath)
	if err != nil {
		t.Errorf("could not open mina state pub input file")
	}
	emptyPubInputBuffer := make([]byte, len(pubInputBytes))

	verified, err := mina_account.VerifyAccountInclusion(proofBytes, emptyPubInputBuffer)
	if err != nil {
		t.Errorf("verification failed with error")
	}
	if verified {
		t.Errorf("proof should not verify")
	}
}
