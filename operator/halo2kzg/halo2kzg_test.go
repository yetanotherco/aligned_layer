package halo2kzg_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/halo2kzg"
)

// MaxProofSize 4KB
const MaxProofSize = 8 * 1024

// MaxProofSize 4KB
const MaxParamsSize = 8 * 1024

// MaxPublicInputSize 4KB
const MaxPublicInputSize = 4 * 1024

const ProofFilePath = "../../scripts/test_files/halo2_kzg/proof.bin"

const PublicInputPath = "../../scripts/test_files/halo2_kzg/pub_input.bin"

const ParamsFilePath = "../../scripts/test_files/halo2_kzg/params.bin"

func TestHalo2KzgProofVerifies(t *testing.T) {
	proofFile, err := os.Open(ProofFilePath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	defer proofFile.Close()

	paramsFile, err := os.Open(ParamsFilePath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	paramsBytes := make([]byte, MaxParamsSize)
	nReadParamsBytes, err := paramsFile.Read(paramsBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	defer paramsFile.Close()

	publicInputFile, err := os.Open(PublicInputPath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	publicInputBytes := make([]byte, MaxPublicInputSize)
	nReadPublicInputBytes, err := publicInputFile.Read(publicInputBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !halo2kzg.VerifyHalo2KzgProof(
		([]byte)(proofBytes), uint32(nReadProofBytes),
		([]byte)(paramsBytes), uint32(nReadParamsBytes),
		([]byte)(publicInputBytes), uint32(nReadPublicInputBytes),
	) {
		t.Errorf("proof did not verify")
	}
}
