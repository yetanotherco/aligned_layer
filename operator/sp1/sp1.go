package sp1

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libsp1_verifier.so -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lsp1_verifier

#include "lib/sp1.h"
*/
import "C"
import "unsafe"

func VerifySp1Proof(proofBuffer []byte, elfBuffer []byte) bool {
	if len(proofBuffer) == 0 || len(elfBuffer) == 0 {
		return false
	}

	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	elfPtr := (*C.uchar)(unsafe.Pointer(&elfBuffer[0]))

	return (bool)(C.verify_sp1_proof_ffi(proofPtr, (C.uint32_t)(len(proofBuffer)), elfPtr, (C.uint32_t)(len(elfBuffer))))
}
