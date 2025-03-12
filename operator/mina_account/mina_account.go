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

func timer() func() {
	start := time.Now()
	return func() {
		fmt.Printf("Mina account verification took %v\n", time.Since(start))
	}
}

func VerifyAccountInclusion(proofBuffer []byte, proofLen uint, pubInputBuffer []byte, pubInputLen uint) bool {
	defer timer()()
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	pubInputPtr := (*C.uchar)(unsafe.Pointer(&pubInputBuffer[0]))
	return (bool)(C.verify_account_inclusion_ffi(proofPtr, (C.uint)(proofLen), pubInputPtr, (C.uint)(pubInputLen)))
}
