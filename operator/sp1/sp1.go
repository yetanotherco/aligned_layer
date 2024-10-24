package sp1

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libsp1_verifier_ffi.so -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lsp1_verifier_ffi

#include "lib/sp1.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func VerifySp1Proof(proofBuffer []byte, elfBuffer []byte) (isVerified bool, err error) {
	// Here we define the return value on failure
	isVerified = false
	err = nil
	if len(proofBuffer) == 0 || len(elfBuffer) == 0 {
		return isVerified, err
	}

	// This will catch any go panic
	defer func() {
		rec := recover()
		if rec != nil {
			err = fmt.Errorf("Panic was caught while verifying sp1 proof: %s", rec)
		}
	}()

	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	elfPtr := (*C.uchar)(unsafe.Pointer(&elfBuffer[0]))

	r := (C.int32_t)(C.verify_sp1_proof_ffi(proofPtr, (C.uint32_t)(len(proofBuffer)), elfPtr, (C.uint32_t)(len(elfBuffer))))

	if r == -1 {
		err = fmt.Errorf("Panic happened on FFI while verifying sp1 proof")
		return isVerified, err
	}

	isVerified = (r == 1)

	return isVerified, err
}
