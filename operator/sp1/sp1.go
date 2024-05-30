package sp1

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libsp1_verifier.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lsp1_verifier

#include "lib/sp1.h"
*/
import "C"
import "unsafe"

func VerifySp1Proof(proofBuffer []byte, proofLen uint, elfBuffer []byte, elfLen uint) bool {
	println("VerifySp1Proof")
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	elfPtr := (*C.uchar)(unsafe.Pointer(&elfBuffer[0]))

	println("Calling C.verify_sp1_proof_ffi")
	return (bool)(C.verify_sp1_proof_ffi(proofPtr, (C.uint)(proofLen), elfPtr, (C.uint)(elfLen)))
}
