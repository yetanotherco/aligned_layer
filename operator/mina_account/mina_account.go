package mina_account

/*
#cgo darwin LDFLAGS: -L./lib -lmina_account_verifier_ffi
#cgo linux LDFLAGS: ${SRCDIR}/lib/libmina_account_verifier_ffi.so -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition

#include "lib/mina_account_verifier.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func VerifyAccountInclusion(proofBuffer []byte, pubInputBuffer []byte) (isVerified bool, err error) {
	// Here we define the return value on failure
	isVerified = false
	err = nil
	if len(proofBuffer) == 0 || len(pubInputBuffer) == 0 {
		return isVerified, err
	}

	// This will catch any go panic
	defer func() {
		rec := recover()
		if rec != nil {
			err = fmt.Errorf("panic was caught while verifying sp1 proof: %s", rec)
		}
	}()

	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	pubInputPtr := (*C.uchar)(unsafe.Pointer(&pubInputBuffer[0]))
	r := (C.int32_t)(C.verify_account_inclusion_ffi(proofPtr, (C.uint32_t)(len(proofBuffer)), pubInputPtr, (C.uint32_t)(len(pubInputBuffer))))

	if r == -1 {
		err = fmt.Errorf("panic happened on FFI while verifying Mina account proof")
		return isVerified, err
	}

	isVerified = (r == 1)

	return isVerified, err
}
