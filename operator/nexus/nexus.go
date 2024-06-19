package nexus

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libsnexus_verifier.so -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lnexus_verifier

#include "lib/nexus.h"
*/
import "C"
import "unsafe"

func VerifyNexusProof(proofBuffer []byte, proofLen uint32, paramsBuffer []byte, paramsLen uint32) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	paramsPtr := (*C.uchar)(unsafe.Pointer(&paramsBuffer[0]))

	return (bool)(C.verify_nexus_proof_ffi(proofPtr, (C.uint32_t)(proofLen), paramsPtr, (C.uint32_t)(paramsLen)))
}
