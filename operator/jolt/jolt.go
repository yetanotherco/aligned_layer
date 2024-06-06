package jolt

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libsp1_verifier.so -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lsp1_verifier

#include "lib/sp1.h"
*/
import "C"
import "unsafe"

func VerifyJoltProof(proofBuffer []byte, proofLen uint32, elfBuffer []byte, elfLen uint32, commitmentBuffer []byte, commitmentLen uint32) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	elfPtr := (*C.uchar)(unsafe.Pointer(&elfBuffer[0]))
	commitmentPtr := (*C.uchar)(unsafe.Pointer(&commitmentBuffer[0]))

	return (bool)(C.verify_jolt_proof_ffi(proofPtr, (C.uint32_t)(proofLen), elfPtr, (C.uint32_t)(elfBuffer), commitmentPtr, (C.uint32_t)(commitmentLen)))
}
