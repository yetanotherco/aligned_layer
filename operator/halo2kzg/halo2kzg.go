package halo2kzg

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libhalo2kzg_verifier.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lhalo2kzg_verifier
#include "lib/halo2kzg.h"
*/
import "C"
import "unsafe"

func VerifyHalo2KzgProof(
	proofBuffer []byte, proofLen uint32,
	paramsBuffer []byte, paramsLen uint32,
	publicInputBuffer []byte, publicInputLen uint32,
) bool {
	if len(proofBuffer) == 0 || len(paramsBuffer) == 0 || len(publicInputBuffer) == 0 {
		return false
	}

	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	paramsPtr := (*C.uchar)(unsafe.Pointer(&paramsBuffer[0]))
	publicInputPtr := (*C.uchar)(unsafe.Pointer(&publicInputBuffer[0]))

	return (bool)(C.verify_halo2_kzg_proof_ffi(
		proofPtr, (C.uint32_t)(proofLen),
		paramsPtr, (C.uint32_t)(paramsLen),
		publicInputPtr, (C.uint32_t)(publicInputLen),
	))
}
