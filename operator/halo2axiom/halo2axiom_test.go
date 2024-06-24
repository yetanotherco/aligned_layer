package halo2axiom_test

import (
	"os"
	"testing"
	"encoding/binary"
	"github.com/yetanotherco/aligned_layer/operator/halo2kzg"
)

const ProofFilePath = "../../task_sender/test_examples/halo2_axiom/proof.bin";

const PublicInputPath = "../../task_sender/test_examples/halo2_axiom/pub_input.bin";

const ParamsFilePath = "../../task_sender/test_examples/halo2_axiom/params.bin";

func TestHalo2KzgProofVerifies(t *testing.T) {
	proofFile, err := os.Open(ProofFilePath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, halo2kzg.MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	defer proofFile.Close()

	paramsFile, err := os.Open(ParamsFilePath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	paramsFileBytes := make([]byte, halo2kzg.MaxParamsSize)
	_, err = paramsFile.Read(paramsFileBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	defer paramsFile.Close()

	vkLenBuffer := make([]byte, 4)
	kzgParamLenBuffer := make([]byte, 4)
	vkBytes := make([]byte, halo2kzg.MaxVerifierKeySize)
	kzgParamsBytes := make([]byte, halo2kzg.MaxKzgParamsSize)

	// Deserialize lengths of values
	copy(vkLenBuffer, paramsFileBytes[:4])
	copy(kzgParamLenBuffer, paramsFileBytes[4:8])

	vkLen :=  binary.LittleEndian.Uint32(vkLenBuffer)
	kzgParamsLen :=  binary.LittleEndian.Uint32(kzgParamLenBuffer)

	// Select bytes
	vkOffset := uint32(8)
	copy(vkBytes, paramsFileBytes[vkOffset:(vkOffset + vkLen)])
	kzgParamsOffset := vkOffset + vkLen
	copy(kzgParamsBytes, paramsFileBytes[kzgParamsOffset:])

	publicInputFile, err := os.Open(PublicInputPath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	publicInputBytes := make([]byte, halo2kzg.MaxPublicInputSize)
	nReadPublicInputBytes, err := publicInputFile.Read(publicInputBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !halo2kzg.VerifyHalo2AxiomProof(
		([halo2kzg.MaxProofSize]byte)(proofBytes), uint32(nReadProofBytes), 
		([halo2kzg.MaxVerifierKeySize]byte)(vkBytes), uint32(vkLen),
		([halo2kzg.MaxKzgParamsSize]byte)(kzgParamsBytes), uint32(kzgParamsLen),
		([halo2kzg.MaxPublicInputSize]byte)(publicInputBytes), uint32(nReadPublicInputBytes),
	) {
		t.Errorf("proof did not verify")
	}
}