package halo2kzg_test

import (
	"fmt"
	"os"
	"testing"
	"encoding/binary"

	"github.com/yetanotherco/aligned_layer/operator/halo2kzg"
)

//TODO: remove prints
func TestHalo2KzgProofVerifies(t *testing.T) {
	proofFile, err := os.Open("./lib/proof.bin")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, halo2kzg.MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	defer proofFile.Close()

	paramsFile, err := os.Open("./lib/params.bin")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	paramsFileBytes := make([]byte, halo2kzg.MaxParamsSize)
	_, err = paramsFile.Read(paramsFileBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	defer paramsFile.Close()

	csLenBuffer := make([]byte, 4)
	vkLenBuffer := make([]byte, 4)
	kzgParamLenBuffer := make([]byte, 4)
	csBytes := make([]byte, halo2kzg.MaxConstraintSystemSize)
	vkBytes := make([]byte, halo2kzg.MaxVerifierKeySize)
	kzgParamsBytes := make([]byte, halo2kzg.MaxKzgParamsSize)

	// Deserialize lengths of values
	copy(csLenBuffer, paramsFileBytes[:4])
	copy(vkLenBuffer, paramsFileBytes[4:8])
	copy(kzgParamLenBuffer, paramsFileBytes[8:12])

	csLen :=  binary.LittleEndian.Uint32(csLenBuffer)
	fmt.Printf("csLen: %d\n", csLen)
	vkLen :=  binary.LittleEndian.Uint32(vkLenBuffer)
	fmt.Printf("vkLen: %d\n", vkLen)
	kzgParamsLen :=  binary.LittleEndian.Uint32(kzgParamLenBuffer)
	fmt.Printf("kzgParamsLen: %d\n", kzgParamsLen)

	// Select bytes
	csOffset := uint32(12)
	fmt.Printf("csOffset: %d\n", csOffset)
	copy(csBytes, paramsFileBytes[csOffset:(csOffset + csLen)])
	vkOffset := csOffset + csLen
	copy(vkBytes, paramsFileBytes[vkOffset:(vkOffset + vkLen)])
	kzgParamsOffset := vkOffset + vkLen
	copy(kzgParamsBytes, paramsFileBytes[kzgParamsOffset:])

	publicInputFile, err := os.Open("./lib/pub_input.bin")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	publicInputBytes := make([]byte, halo2kzg.MaxPublicInputSize)
	nReadPublicInputBytes, err := publicInputFile.Read(publicInputBytes)
	fmt.Printf("Public Input Len: %d\n", nReadPublicInputBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}

	if !halo2kzg.VerifyHalo2KzgProof(
		([halo2kzg.MaxProofSize]byte)(proofBytes), uint(nReadProofBytes), 
		([halo2kzg.MaxConstraintSystemSize]byte)(csBytes), uint(csLen),
		([halo2kzg.MaxVerifierKeySize]byte)(vkBytes), uint(vkLen),
		([halo2kzg.MaxKzgParamsSize]byte)(kzgParamsBytes), uint(kzgParamsLen),
		([halo2kzg.MaxPublicInputSize]byte)(publicInputBytes), uint(nReadPublicInputBytes),
	) {
		t.Errorf("proof did not verify")
	}
}