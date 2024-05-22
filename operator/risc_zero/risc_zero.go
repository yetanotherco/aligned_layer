package risc_zero

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/librisc_zero_verifier_ffi.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: ${SRCDIR}/lib/librisc_zero_verifier_ffi.dylib

#include "lib/risc_zero.h"
*/
import "C"
import "unsafe"

const (
	MaxReceiptSize = 215523
)

func VerifyRiscZeroReceipt(receiptBuffer [MaxReceiptSize]byte, receiptLen uint, imageIdBuffer [8]uint32) bool {
	receiptPtr := (*C.uchar)(unsafe.Pointer(&receiptBuffer[0]))
	imageIdPtr := (*C.uint)(unsafe.Pointer(&imageIdBuffer[0]))
	return (bool)(C.verify_risc_zero_receipt_ffi(receiptPtr, (C.uint)(receiptLen), imageIdPtr))
}
