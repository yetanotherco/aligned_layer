package halo2ipa_test

import (
	"encoding/binary"
	"github.com/lambdaclass/aligned_layer/operator/halo2ipa"
	"os"
	"testing"
)

const ProofFilePath = "../../scripts/test_files/halo2_ipa/proof.bin"

const PublicInputPath = "../../scripts/test_files/halo2_ipa/pub_input.bin"

const ParamsFilePath = "../../scripts/test_files/halo2_ipa/params.bin"

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

	csLen := binary.LittleEndian.Uint32(csLenBuffer)
	vkLen := binary.LittleEndian.Uint32(vkLenBuffer)
	ipaParamsLen := binary.LittleEndian.Uint32(ipaParamLenBuffer)

	// Select bytes
	csOffset := uint32(12)
	copy(csBytes, paramsFileBytes[csOffset:(csOffset+csLen)])
	vkOffset := csOffset + csLen
	copy(vkBytes, paramsFileBytes[vkOffset:(vkOffset+vkLen)])
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
		([halo2ipa.MaxProofSize]byte)(proofBytes), uint32(nReadProofBytes),
		([halo2ipa.MaxConstraintSystemSize]byte)(csBytes), uint32(csLen),
		([halo2ipa.MaxVerifierKeySize]byte)(vkBytes), uint32(vkLen),
		([halo2ipa.MaxIpaParamsSize]byte)(ipaParamsBytes), uint32(ipaParamsLen),
		([halo2ipa.MaxPublicInputSize]byte)(publicInputBytes), uint32(nReadPublicInputBytes),
	) {
		t.Errorf("proof did not verify")
	}
}
