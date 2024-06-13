package jolt

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libjolt_verifier.so -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -ljolt_verifier

#include "lib/jolt.h"
*/
import "C"
import "unsafe"

func VerifyJoltProof(proofBuffer []byte, proofLen uint32, elfBuffer []byte, elfLen uint32) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	elfPtr := (*C.uchar)(unsafe.Pointer(&elfBuffer[0]))

	return (bool)(C.verify_jolt_proof_ffi(proofPtr, (C.uint32_t)(proofLen), elfPtr, (C.uint32_t)(elfLen)))
}
