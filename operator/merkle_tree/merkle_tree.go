package merkle_tree

/*
#cgo linux LDFLAGS: -L./lib -llibmerkle_tree -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -llibmerkle_tree

#include "lib/merkle_tree.h"
*/
import "C"
import "unsafe"


func VerifyMerkleTreeBatch(batchBuffer []byte, batchLen uint, merkleRootBuffer [32]byte) bool {
	batchPtr := (*C.uchar)(unsafe.Pointer(&batchBuffer[0]))
	merkleRootPtr := (*C.uchar)(unsafe.Pointer(&merkleRootBuffer[0]))
	return (bool)(C.verify_merkle_tree_batch_ffi(batchPtr, (C.uint)(batchLen), merkleRootPtr))
}
