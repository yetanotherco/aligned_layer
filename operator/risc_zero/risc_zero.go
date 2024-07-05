package risc_zero

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/librisc_zero_verifier_ffi.so -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lrisc_zero_verifier_ffi

#include "lib/risc_zero.h"
*/
import "C"
import (
	"unsafe"
)

func VerifyRiscZeroReceipt(receiptBuffer []byte, receiptLen uint32, imageIdBuffer []byte, imageIdLen uint32) bool {
	receiptPtr := (*C.uchar)(unsafe.Pointer(&receiptBuffer[0]))
	imageIdPtr := (*C.uchar)(unsafe.Pointer(&imageIdBuffer[0]))
	return (bool)(C.verify_risc_zero_receipt_ffi(receiptPtr, (C.uint32_t)(receiptLen), imageIdPtr, (C.uint32_t)(imageIdLen)))
}
