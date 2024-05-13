package sp1_groth16

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libsp1_groth16_verifier.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lsp1_groth16_verifier

#include "lib/sp1_groth16.h"
*/
import "C"
import "unsafe"

// MaxProofSize 2 MB
const MaxProofSize = 2 * 1024 * 1024

// MaxElfBufferSize 1 MB
const MaxElfBufferSize = 1024 * 1024

func VerifySp1Groth16Proof(proofBuffer [MaxProofSize]byte, proofLen uint, elfBuffer [MaxElfBufferSize]byte, elfLen uint) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	elfPtr := (*C.uchar)(unsafe.Pointer(&elfBuffer[0]))
	return (bool)(C.verify_sp1_groth16_proof_ffi(proofPtr, (C.uint)(proofLen), elfPtr, (C.uint)(elfLen)))
}

// Should i create another function for groth16?
