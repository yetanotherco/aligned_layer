package merkle_tree

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libmerkle_tree.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: ${SRCDIR}/lib/libmerkle_tree.dylib

#include "lib/merkle_tree.h"
*/
import "C"
import "unsafe"
import "fmt"

func VerifyMerkleTreeBatch(batchBuffer []byte, merkleRootBuffer [32]byte) (isVerified bool, err error) {
	// Here we define the return value on failure
	isVerified = false
	err = nil
	if len(batchBuffer) == 0 {
		return isVerified, err
	}

	// This will catch any go panic
	defer func() {
		rec := recover()
		if rec != nil {
			err = fmt.Errorf("Panic was caught while verifying merkle tree batch: %s", rec)
		}
	}()

	batchPtr := (*C.uchar)(unsafe.Pointer(&batchBuffer[0]))
	merkleRootPtr := (*C.uchar)(unsafe.Pointer(&merkleRootBuffer[0]))

	r := (C.int32_t)(C.verify_merkle_tree_batch_ffi(batchPtr, (C.uint)(len(batchBuffer)), merkleRootPtr))

	if r == -1 {
		err = fmt.Errorf("Panic happened on FFI while verifying merkle tree batch")
		return isVerified, err
	}

	isVerified = (r == 1)

	return isVerified, err
}
