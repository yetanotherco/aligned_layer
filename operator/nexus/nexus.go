package nexus

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libnexus_verifier.so -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lnexus_verifier
#include "lib/nexus.h"
*/
import "C"
import "unsafe"

func VerifyNexusProof(proofBuffer []byte, paramsBuffer []byte) bool {
	proofPtr := (*C.uchar)(unsafe.SliceData(proofBuffer))
	paramsPtr := (*C.uchar)(unsafe.SliceData(paramsBuffer))

	return (bool)(C.verify_nexus_proof_ffi(proofPtr, (C.uint32_t)(len(proofBuffer)), paramsPtr, (C.uint32_t)(len(paramsBuffer))))
}
