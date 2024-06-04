package jolt

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libsp1_verifier.so -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lsp1_verifier

#include "lib/sp1.h"
*/
import "C"
import "unsafe"

func VerifyJoltProof(proofBuffer []byte, proofLen uint32, infoBuffer []byte, infoLen uint32) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	infoPtr := (*C.uchar)(unsafe.Pointer(&infoBuffer[0]))

	return (bool)(C.verify_jolt_proof_ffi(proofPtr, (C.uint32_t)(proofLen), infoPtr, (C.uint32_t)(infoLen)))
}
