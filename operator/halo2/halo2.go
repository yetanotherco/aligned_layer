package halo2

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libhalo2_verifier.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lhalo2_verifier

#include "lib/halo2.h"
*/
import "C"
import "unsafe"


func VerifyHalo2KZGProof(proofBuffer [MaxProofSize]byte, proofLen uint, verificationKeyBytes []byte, ) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	vkPtr := (*C.uchar)(unsafe.Pointer(&verificationKeyBytes[0]))
	return (bool)(C.verify_halo2_kzg_proof_ffi(proofPtr, (C.uint)(proofLen), ))
}