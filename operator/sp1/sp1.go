package sp1

/*
#cgo linux LDFLAGS: -L./lib -lsp1_verifier -ldl -lrt -lm
#cgo darwin LDFLAGS: -L./lib -lsp1_verifier

#include "lib/sp1.h"
*/
import "C"
import "unsafe"

const MAX_PROOF_SIZE = 1024 * 1024
const MAX_ELF_BUFFER_SIZE = 1024 * 1024

func VerifySp1Proof(proofBuffer [MAX_PROOF_SIZE]byte, proofLen uint, elfBuffer [MAX_ELF_BUFFER_SIZE]byte, elfLen uint) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	elfPtr := (*C.uchar)(unsafe.Pointer(&elfBuffer[0]))
	return (bool)(C.verify_sp1_proof_ffi(proofPtr, (C.uint)(proofLen), elfPtr, (C.uint)(elfLen)))
}
