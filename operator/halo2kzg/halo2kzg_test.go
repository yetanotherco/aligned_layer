package halo2kzg_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/halo2kzg"
)

const ProofFilePath = "../../scripts/test_files/halo2_kzg/proof.bin"

const PublicInputPath = "../../scripts/test_files/halo2_kzg/pub_input.bin"

const ParamsFilePath = "../../scripts/test_files/halo2_kzg/params.bin"

func TestHalo2KzgProofVerifies(t *testing.T) {
	proofBytes, err := os.ReadFile(ProofFilePath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	paramsBytes, err := os.ReadFile(ParamsFilePath)
	if err != nil {
		t.Errorf("could not open params file: %s", err)
	}

	publicInputBytes, err := os.ReadFile(PublicInputPath)
	if err != nil {
		t.Errorf("could not open public input file: %s", err)
	}

	if !halo2kzg.VerifyHalo2KzgProof(
		([]byte)(proofBytes),
		([]byte)(paramsBytes),
		([]byte)(publicInputBytes),
	) {
		t.Errorf("proof did not verify")
	}
}
