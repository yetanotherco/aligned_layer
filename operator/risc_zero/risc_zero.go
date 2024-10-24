package risc_zero

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/librisc_zero_verifier_ffi.so -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lrisc_zero_verifier

#include "lib/risc_zero.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func VerifyRiscZeroReceipt(innerReceiptBuffer []byte, imageIdBuffer []byte, publicInputBuffer []byte) (isVerified bool, err error) {
	// Here we define the return value on failure
	isVerified = false
	err = nil

	if len(innerReceiptBuffer) == 0 || len(imageIdBuffer) == 0 {
		return isVerified, err
	}

	// This will catch any go panic
	defer func() {
		rec := recover()
		if rec != nil {
			err = fmt.Errorf("Panic was caught while verifying risc0 proof: %s", rec)
		}
	}()

	receiptPtr := (*C.uchar)(unsafe.Pointer(&innerReceiptBuffer[0]))
	imageIdPtr := (*C.uchar)(unsafe.Pointer(&imageIdBuffer[0]))

	r := (C.int32_t)(0)

	if len(publicInputBuffer) == 0 { // allow empty public input
		r = (C.int32_t)(C.verify_risc_zero_receipt_ffi(receiptPtr, (C.uint32_t)(len(innerReceiptBuffer)), imageIdPtr, (C.uint32_t)(len(imageIdBuffer)), nil, (C.uint32_t)(0)))
	} else {
		publicInputPtr := (*C.uchar)(unsafe.Pointer(&publicInputBuffer[0]))
		r = (C.int32_t)(C.verify_risc_zero_receipt_ffi(receiptPtr, (C.uint32_t)(len(innerReceiptBuffer)), imageIdPtr, (C.uint32_t)(len(imageIdBuffer)), publicInputPtr, (C.uint32_t)(len(publicInputBuffer))))
	}

	if r == -1 {
		err = fmt.Errorf("Panic happened on FFI while verifying risc0 proof")
		return isVerified, err
	}

	isVerified = (r == 1)

	return isVerified, err
}
