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

	csBuffer := make([]byte, 4)
	vkLenBuffer := make([]byte, 4)
	kzgParamLenBuffer := make([]byte, 4)
	csBytes := make([]byte, halo2kzg.MaxCSSize)
	vkBytes := make([]byte, halo2kzg.MaxVerifierKeySize)
	kzgParamsBytes := make([]byte, halo2kzg.MaxKzgParamsSize)

	// Deserialize lengths of values
	copy(csBuffer, paramsFileBytes[:4])
	copy(vkLenBuffer, paramsFileBytes[4:8])
	copy(kzgParamLenBuffer, paramsFileBytes[8:12])

	csLen :=  binary.LittleEndian.Uint32(csBuffer)
	vkLen :=  binary.LittleEndian.Uint32(vkLenBuffer)
	kzgParamsLen :=  binary.LittleEndian.Uint32(kzgParamLenBuffer)

	csOffset := uint32(12)
	copy(csBytes, paramsFileBytes[vkOffset:(csOffset + csLen)])

	// Select Vk Bytes
	vkOffset := csOffset + csLen
	copy(vkBytes, paramsFileBytes[vkOffset:(vkOffset + vkLen)])

	// Select KZG Bytes
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
		([halo2kzg.MaxCSSize]byte)(csBytes), uint32(csLen),
		([halo2kzg.MaxVerifierKeySize]byte)(vkBytes), uint32(vkLen),
		([halo2kzg.MaxKzgParamsSize]byte)(kzgParamsBytes), uint32(kzgParamsLen),
		([halo2kzg.MaxPublicInputSize]byte)(publicInputBytes), uint32(nReadPublicInputBytes),
	) {
		t.Errorf("proof did not verify")
	}
}