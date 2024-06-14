package risc_zero

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/librisc_zero_verifier_ffi.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: ${SRCDIR}/lib/librisc_zero_verifier_ffi.dylib

#include "lib/risc_zero.h"
*/
import "C"
import "unsafe"

func VerifyRiscZeroReceipt(receiptBuffer []byte, receiptLen uint32, imageIdBuffer [8]uint32) bool {
	receiptPtr := (*C.uchar)(unsafe.Pointer(&receiptBuffer[0]))
	imageIdPtr := (*C.uint32_t)(unsafe.Pointer(&imageIdBuffer[0]))
	return (bool)(C.verify_risc_zero_receipt_ffi(receiptPtr, (C.uint32_t)(receiptLen), imageIdPtr))
}
