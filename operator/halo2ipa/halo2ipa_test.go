package halo2ipa_test

import (
	"io"
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/halo2ipa"
)

const ProofFilePath = "../../scripts/test_files/halo2_ipa/proof.bin"

const PublicInputPath = "../../scripts/test_files/halo2_ipa/pub_input.bin"

const ParamsFilePath = "../../scripts/test_files/halo2_ipa/params.bin"

func TestHalo2IpaProofVerifies(t *testing.T) {
	proofFile, err := os.Open(ProofFilePath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes, err := io.ReadAll(proofFile)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	defer proofFile.Close()

	paramsFile, err := os.Open(ParamsFilePath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	paramsBytes, err := io.ReadAll(paramsFile)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	defer paramsFile.Close()

	publicInputFile, err := os.Open(PublicInputPath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	publicInputBytes, err := io.ReadAll(publicInputFile)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !halo2ipa.VerifyHalo2IpaProof(
		([]byte)(proofBytes), uint32(len(proofBytes)),
		([]byte)(paramsBytes), uint32(len(paramsBytes)),
		([]byte)(publicInputBytes), uint32(len(publicInputBytes)),
	) {
		t.Errorf("proof did not verify")
	}
}
