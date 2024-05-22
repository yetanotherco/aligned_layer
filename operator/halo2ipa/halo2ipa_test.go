package halo2ipa_test

import (
	"os"
	"testing"
	"encoding/binary"
	"github.com/yetanotherco/aligned_layer/operator/halo2ipa"
)

const ProofFilePath = "../../task_sender/test_examples/halo2_ipa/proof.bin";

const PublicInputPath = "../../task_sender/test_examples/halo2_ipa/pub_input.bin";

const ParamsFilePath = "../../task_sender/test_examples/halo2_ipa/params.bin";

func TestHalo2IpaProofVerifies(t *testing.T) {
	proofFile, err := os.Open(ProofFilePath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, halo2ipa.MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	defer proofFile.Close()

	paramsFile, err := os.Open(ParamsFilePath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	paramsFileBytes := make([]byte, halo2ipa.MaxParamsSize)
	_, err = paramsFile.Read(paramsFileBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	defer paramsFile.Close()

	csLenBuffer := make([]byte, 4)
	vkLenBuffer := make([]byte, 4)
	ipaParamLenBuffer := make([]byte, 4)
	csBytes := make([]byte, halo2ipa.MaxConstraintSystemSize)
	vkBytes := make([]byte, halo2ipa.MaxVerifierKeySize)
	ipaParamsBytes := make([]byte, halo2ipa.MaxIpaParamsSize)

	// Deserialize lengths of values
	copy(csLenBuffer, paramsFileBytes[:4])
	copy(vkLenBuffer, paramsFileBytes[4:8])
	copy(ipaParamLenBuffer, paramsFileBytes[8:12])

	csLen :=  binary.LittleEndian.Uint32(csLenBuffer)
	vkLen :=  binary.LittleEndian.Uint32(vkLenBuffer)
	ipaParamsLen :=  binary.LittleEndian.Uint32(ipaParamLenBuffer)

	// Select bytes
	csOffset := uint32(12)
	copy(csBytes, paramsFileBytes[csOffset:(csOffset + csLen)])
	vkOffset := csOffset + csLen
	copy(vkBytes, paramsFileBytes[vkOffset:(vkOffset + vkLen)])
	ipaParamsOffset := vkOffset + vkLen
	copy(ipaParamsBytes, paramsFileBytes[ipaParamsOffset:])

	publicInputFile, err := os.Open(PublicInputPath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	publicInputBytes := make([]byte, halo2ipa.MaxPublicInputSize)
	nReadPublicInputBytes, err := publicInputFile.Read(publicInputBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !halo2ipa.VerifyHalo2IpaProof(
		([halo2ipa.MaxProofSize]byte)(proofBytes), uint(nReadProofBytes), 
		([halo2ipa.MaxConstraintSystemSize]byte)(csBytes), uint(csLen),
		([halo2ipa.MaxVerifierKeySize]byte)(vkBytes), uint(vkLen),
		([halo2ipa.MaxIpaParamsSize]byte)(ipaParamsBytes), uint(ipaParamsLen),
		([halo2ipa.MaxPublicInputSize]byte)(publicInputBytes), uint(nReadPublicInputBytes),
	) {
		t.Errorf("proof did not verify")
	}
}