package sp1

import "C"

// #include "hello.h"

import "C"

const MAX_PROOF_SIZE = 1024 * 1024

func VerifySp1Proof(proofBuffer [MAX_PROOF_SIZE]byte, proofLen uint) bool {
	//proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	//return (bool)(C.verify_sp1_proof_ffi(proofPtr, (C.uint)(proofLen)))
	C.hello()
	return true
}
