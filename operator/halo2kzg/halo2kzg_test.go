package halo2kzg_test

import (
	"io"
	"fmt"
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/halo2kzg"
)

func TestHalo2KzgProofVerifies(t *testing.T) {
	proofFile, err := os.Open("./lib/plonk_proof.bin")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	proofBytes := make([]byte, halo2kzg.MaxProofSize)
	nReadProofBytes, err := proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	fmt.Println("Length of proof input bytes:", nReadProofBytes)

	paramsFile, err := os.Open("./lib/verification_key.bin")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	vkBytes := make([]byte, halo2kzg.MaxVerifierKeySize)
	kzgParamsBytes := make([]byte, halo2kzg.MaxKzgParamsSize)
	nReadVkBytes, err := paramsFile.Read(vkBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	// Seek to new file position
	_, err = paramsFile.Seek(halo2kzg.MaxVerifierKeySize, io.SeekStart)
	if err != nil {
		fmt.Println("Error seeking file:", err)
		return
	}
	nReadKzgParamsBytes, err := paramsFile.Read(kzgParamsBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	fmt.Println("Length of vk bytes:", nReadVkBytes)
	fmt.Println("Length of kzg params bytes:", nReadKzgParamsBytes)

	publicInputFile, err := os.Open("./lib/pub_input.bin")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	publicInputBytes := make([]byte, halo2kzg.MaxPublicInputSize)
	nReadPublicInputBytes, err := publicInputFile.Read(publicInputBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	fmt.Println("Length of public input bytes:", nReadPublicInputBytes)

	if !halo2kzg.VerifyHalo2KzgProof(
		([halo2kzg.MaxProofSize]byte)(proofBytes), uint(nReadProofBytes), 
		([halo2kzg.MaxVerifierKeySize]byte)(vkBytes), uint(nReadVkBytes),
		([halo2kzg.MaxKzgParamsSize]byte)(kzgParamsBytes), uint(nReadKzgParamsBytes),
		([halo2kzg.MaxPublicInputSize]byte)(publicInputBytes), uint(nReadPublicInputBytes),
	) {
		t.Errorf("proof did not verify")
	}
}