package operator_test

import (
	"bytes"
	"os"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/backend/witness"
)

const MaxProofSize = 2 * 1024 * 1024
const MaxElfSize = 2 * 1024 * 1024

func TestBreakingGnark(t *testing.T) {
	// proofFile, err := os.Open("../../scripts/test_files/sp1/sp1_fibonacci.proof")
	// if err != nil {
	// 	t.Errorf("could not open proof file: %s", err)
	// }
	// proofBytes := make([]byte, MaxProofSize)
	// nReadProofBytes, err := proofFile.Read(proofBytes)
	// if err != nil {
	// 	t.Errorf("could not read bytes from file")
	// }

	// elfFile, err := os.Open("../../scripts/test_files/sp1/sp1_fibonacci.elf")
	// if err != nil {
	// 	t.Errorf("could not open proof file: %s", err)
	// }

	// elfBytes := make([]byte, MaxElfSize)
	// nReadElfBytes, err := elfFile.Read(elfBytes)
	// if err != nil {
	// 	t.Errorf("could not read bytes from file")
	// }

	//
	curve := ecc.BN254

	// PROOF

	proofFile, err := os.Open("../../scripts/test_files/ineq_93042012_groth16.proof")
	proofBytes := make([]byte, MaxProofSize)
	_, err = proofFile.Read(proofBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	proofReader := bytes.NewReader(proofBytes)
	proof := groth16.NewProof(ecc.BN254)
	if _, err := proof.ReadFrom(proofReader); err != nil {
		t.Errorf("could not read bytes from file")
	}

	// PUB INPUT
	pubInputFile, err := os.Open("../../scripts/test_files/ineq_93042012_groth16.pub")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	pubInputBytes := make([]byte, MaxProofSize)
	_, err = pubInputFile.Read(pubInputBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	pubInputReader := bytes.NewReader(pubInputBytes)
	pubInput, err := witness.New(curve.ScalarField())
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	if _, err = pubInput.ReadFrom(pubInputReader); err != nil {
		t.Errorf("could not read bytes from file")
	}

	// VERIFICATION KEY
	verificationKeyFile, err := os.Open("../../scripts/test_files/ineq_93042012_groth16.vk")
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}
	verificationKeyBytes := make([]byte, MaxProofSize)
	_, err = verificationKeyFile.Read(verificationKeyBytes)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	verificationKeyReader := bytes.NewReader(pubInputBytes)
	verificationKey := groth16.NewVerifyingKey(curve)
	if err != nil {
		t.Errorf("could not read bytes from file")
	}
	if _, err = verificationKey.ReadFrom(verificationKeyReader); err != nil {
		t.Errorf("could not read bytes from file")
	}

	// VERIFY
	err = groth16.Verify(proof, verificationKey, pubInput)
	if err != nil {
		panic("GROTH16 proof not verified")
	}
}
