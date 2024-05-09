package sp1

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libsp1_verifier.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lsp1_verifier

#include "lib/sp1.h"
*/
import "C"
import "unsafe"

const MaxProofSize = 2 * 1024 * 1024
const MaxElfBufferSize = 1024 * 1024

func VerifySp1Proof(proofBuffer [MaxProofSize]byte, proofLen uint, elfBuffer [MaxElfBufferSize]byte, elfLen uint) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	elfPtr := (*C.uchar)(unsafe.Pointer(&elfBuffer[0]))
	return (bool)(C.verify_sp1_proof_ffi(proofPtr, (C.uint)(proofLen), elfPtr, (C.uint)(elfLen)))
}
