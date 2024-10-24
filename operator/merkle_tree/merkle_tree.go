package merkle_tree

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libmerkle_tree.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: ${SRCDIR}/lib/libmerkle_tree.dylib

#include "lib/merkle_tree.h"
*/
import "C"
import "unsafe"

func VerifyMerkleTreeBatch(batchBuffer []byte, merkleRootBuffer [32]byte) bool {
	if len(batchBuffer) == 0 {
		return false
	}

	batchPtr := (*C.uchar)(unsafe.Pointer(&batchBuffer[0]))
	merkleRootPtr := (*C.uchar)(unsafe.Pointer(&merkleRootBuffer[0]))
	return (bool)(C.verify_merkle_tree_batch_ffi(batchPtr, (C.uint)(len(batchBuffer)), merkleRootPtr))
}
