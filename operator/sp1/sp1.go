package sp1

import "C"
import "unsafe"

/*
#cgo linux LDFLAGS: -L./lib -lsp1_verifier -ldl -lrt -lm
#cgo darwin LDFLAGS: -L./lib -lsp1_verifier
#include "lib/sp1.h"
*/
import "C"

const MAX_PROOF_SIZE = 1024 * 1024

func VerifySp1Proof(proofBuffer [MAX_PROOF_SIZE]byte, proofLen uint) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	return (bool)(C.verify_sp1_proof_ffi(proofPtr, (C.uint)(proofLen)))
	//C.hello()
}
