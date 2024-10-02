package main

/*
#include <stdlib.h>
#include <stdint.h>

typedef struct ListRef {
  const uint8_t *ptr;
  uintptr_t len;
} ListRef;


*/
import "C"

import (
	"bytes"
	"log"
	"unsafe"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/backend/plonk"
	"github.com/consensys/gnark/backend/witness"
)

func listRefToBytes(listRef C.ListRef) []byte {

	if listRef.len == 0 {
		return []byte{}
	}

	return C.GoBytes(unsafe.Pointer(listRef.ptr), C.int(listRef.len))
}

func main() {}

//export VerifyPlonkProofBLS12_381
func VerifyPlonkProofBLS12_381(proofBytes C.ListRef, pubInputBytes C.ListRef, verificationKeyBytes C.ListRef) bool {
	return verifyPlonkProof(proofBytes, pubInputBytes, verificationKeyBytes, ecc.BLS12_381)
}

//export VerifyPlonkProofBN254
func VerifyPlonkProofBN254(proofBytes C.ListRef, pubInputBytes C.ListRef, verificationKeyBytes C.ListRef) bool {
	return verifyPlonkProof(proofBytes, pubInputBytes, verificationKeyBytes, ecc.BN254)
}

//export VerifyGroth16ProofBN254
func VerifyGroth16ProofBN254(proofBytes C.ListRef, pubInputBytes C.ListRef, verificationKeyBytes C.ListRef) bool {
	return verifyGroth16Proof(proofBytes, pubInputBytes, verificationKeyBytes, ecc.BN254)
}

// verifyPlonkProof contains the common proof verification logic.
func verifyPlonkProof(proofBytesRef C.ListRef, pubInputBytesRef C.ListRef, verificationKeyBytesRef C.ListRef, curve ecc.ID) bool {
	proofBytes := listRefToBytes(proofBytesRef)
	pubInputBytes := listRefToBytes(pubInputBytesRef)
	verificationKeyBytes := listRefToBytes(verificationKeyBytesRef)

	proofReader := bytes.NewReader(proofBytes)
	proof := plonk.NewProof(curve)
	if _, err := proof.ReadFrom(proofReader); err != nil {
		log.Printf("Could not deserialize proof: %v", err)
		return false
	}

	pubInputReader := bytes.NewReader(pubInputBytes)
	pubInput, err := witness.New(curve.ScalarField())
	if err != nil {
		log.Printf("Error instantiating witness: %v", err)
		return false
	}
	if _, err = pubInput.ReadFrom(pubInputReader); err != nil {
		log.Printf("Could not read PLONK public input: %v", err)
		return false
	}

	verificationKeyReader := bytes.NewReader(verificationKeyBytes)
	verificationKey := plonk.NewVerifyingKey(curve)
	if _, err = verificationKey.ReadFrom(verificationKeyReader); err != nil {
		log.Printf("Could not read PLONK verifying key from bytes: %v", err)
		return false
	}

	err = plonk.Verify(proof, verificationKey, pubInput)
	return err == nil
}

// verifyGroth16Proof contains the common proof verification logic.
func verifyGroth16Proof(proofBytesRef C.ListRef, pubInputBytesRef C.ListRef, verificationKeyBytesRef C.ListRef, curve ecc.ID) bool {
	proofBytes := listRefToBytes(proofBytesRef)
	pubInputBytes := listRefToBytes(pubInputBytesRef)
	verificationKeyBytes := listRefToBytes(verificationKeyBytesRef)

	proofReader := bytes.NewReader(proofBytes)
	proof := groth16.NewProof(curve)
	if _, err := proof.ReadFrom(proofReader); err != nil {
		log.Printf("Could not deserialize proof: %v", err)
		return false
	}

	pubInputReader := bytes.NewReader(pubInputBytes)
	pubInput, err := witness.New(curve.ScalarField())
	if err != nil {
		log.Printf("Error instantiating witness: %v", err)
		return false
	}
	if _, err = pubInput.ReadFrom(pubInputReader); err != nil {
		log.Printf("Could not read Groth16 public input: %v", err)
		return false
	}

	verificationKeyReader := bytes.NewReader(verificationKeyBytes)
	verificationKey := groth16.NewVerifyingKey(curve)
	if _, err = verificationKey.ReadFrom(verificationKeyReader); err != nil {
		log.Printf("Could not read Groth16 verifying key from bytes: %v", err)
		return false
	}

	err = groth16.Verify(proof, verificationKey, pubInput)
	return err == nil
}
