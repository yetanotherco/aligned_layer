package mina_account

/*
#cgo darwin LDFLAGS: -L./lib -lmina_account_verifier_ffi
#cgo linux LDFLAGS: ${SRCDIR}/lib/libmina_account_verifier_ffi.so -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition

#include "lib/mina_account_verifier.h"
*/
import "C"
import (
	"fmt"
	"time"
	"unsafe"
)

// TODO(xqft): check proof size
const MAX_PROOF_SIZE = 16 * 1024
const MAX_PUB_INPUT_SIZE = 6 * 1024

func timer() func() {
	start := time.Now()
	return func() {
		fmt.Printf("Mina account verification took %v\n", time.Since(start))
	}
}

func VerifyAccountInclusion(proofBuffer [MAX_PROOF_SIZE]byte, proofLen uint, pubInputBuffer [MAX_PUB_INPUT_SIZE]byte, pubInputLen uint) bool {
	defer timer()()
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	pubInputPtr := (*C.uchar)(unsafe.Pointer(&pubInputBuffer[0]))
	return (bool)(C.verify_account_inclusion_ffi(proofPtr, (C.uint)(proofLen), pubInputPtr, (C.uint)(pubInputLen)))
}
