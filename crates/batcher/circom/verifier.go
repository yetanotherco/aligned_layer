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
	"github.com/yetanotherco/go-circom-prover-verifier/parsers"
	"github.com/yetanotherco/go-circom-prover-verifier/verifier"
	"unsafe"
)

func listRefToBytes(listRef C.ListRef) []byte {
	if listRef.len == 0 {
		return []byte{}
	}

	return C.GoBytes(unsafe.Pointer(listRef.ptr), C.int(listRef.len))
}

func main() {}

//export VerifyCircomGroth16ProofBN128
func VerifyCircomGroth16ProofBN128(proofBytesRef C.ListRef, pubInputBytesRef C.ListRef, verificationKeyBytesRef C.ListRef) bool {
	proofBytes := listRefToBytes(proofBytesRef)
	pubInputBytes := listRefToBytes(proofBytesRef)
	verificationKeyBytes := listRefToBytes(proofBytesRef)

	proof, err := parsers.ParseProof(proofBytes)
	if err != nil {
		return false
	}
	public, err := parsers.ParsePublicSignals(pubInputBytes)
	if err != nil {
		return false
	}
	vk, err := parsers.ParseVk(verificationKeyBytes)
	if err != nil {
		return false
	}

	return verifier.Verify(vk, proof, public)
}
