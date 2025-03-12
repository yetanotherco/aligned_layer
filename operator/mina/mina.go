package mina

/*
#cgo darwin LDFLAGS: -L./lib -lmina_state_verifier_ffi
#cgo linux LDFLAGS: ${SRCDIR}/lib/libmina_state_verifier_ffi.so -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition

#include "lib/mina_verifier.h"
*/
import "C"
import (
	"fmt"
	"time"
	"unsafe"
)

func timer() func() {
	start := time.Now()
	return func() {
		fmt.Printf("Mina block verification took %v\n", time.Since(start))
	}
}

func VerifyMinaState(proofBuffer []byte, proofLen uint, pubInputBuffer []byte, pubInputLen uint) bool {
	defer timer()()

	if len(proofBuffer) == 0 || len(pubInputBuffer) == 0 {
		return false
	}

	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	pubInputPtr := (*C.uchar)(unsafe.Pointer(&pubInputBuffer[0]))
	return (bool)(C.verify_mina_state_ffi(proofPtr, (C.uint)(proofLen), pubInputPtr, (C.uint)(pubInputLen)))
}
