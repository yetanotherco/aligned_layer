package mina

/*
#cgo darwin LDFLAGS: -L./lib -lmina_state_verifier
#cgo linux LDFLAGS: -L./lib -lmina_state_verifier -ldl -lrt -lm

#include "lib/mina_verifier.h"
*/
import "C"
import (
	"unsafe"
)

// TODO(xqft): check proof size
const MAX_PROOF_SIZE = 16 * 1024
const MAX_PUB_INPUT_SIZE = 1024

func VerifyProtocolStateProof(proofBuffer [MAX_PROOF_SIZE]byte, proofLen uint, pubInputBuffer [MAX_PUB_INPUT_SIZE]byte, pubInputLen uint) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	pubInputPtr := (*C.uchar)(unsafe.Pointer(&pubInputBuffer[0]))
	return (bool)(C.verify_protocol_state_proof_ffi(proofPtr, (C.uint)(proofLen), pubInputPtr, (C.uint)(pubInputLen)))
}
