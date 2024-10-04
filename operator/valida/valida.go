package valida

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libvalida_verifier_ffi.so -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lvalida_verifier_ffi

#include "lib/valida.h"
*/
import "C"
import (
	"unsafe"
)

func VerifyValidaProof(proofBuffer []byte, programBuffer []byte) bool {
	if len(proofBuffer) == 0 || len(programBuffer) == 0 {
		return false
	}

	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	programPtr := (*C.uchar)(unsafe.Pointer(&programBuffer[0]))

	return (bool)(C.verify_valida_proof_ffi(proofPtr, (C.uint32_t)(len(proofBuffer)), programPtr, (C.uint32_t)(len(programBuffer))))
}
