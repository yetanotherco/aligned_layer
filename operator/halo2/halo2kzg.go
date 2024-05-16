package halo2kzg

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libhalo2_verifier.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lhalo2_verifier

#include "lib/halo2.h"
*/
import "C"
import "unsafe"

// MaxProofSize 2MB
const MaxProofSize = 2 * 1024 * 1024;

// MaxVerificationKeySize 1 KB
const MaxVerificationKeySize = 4 * 1024;

// MaxVerificationKeySize 4 MB
const MaxKZGParamsSize = 4 * 1024 * 1024;

// MaxVerificationKeySize 4 MB
const MaxPublicInputSize = 4 * 1024 * 1024;

func VerifyHalo2KZGProof(
	proofBuffer [MaxProofSize]byte, proofLen uint, 
	vkBuffer [MaxVerifierKeySize]byte, vkLen uint, 
	kzgParamBuffer [MaxKZGParamsSize]byte, kzgParamLen uint, 
	publicInputBuffer [MaxPublicInputSize]byte, publicInputLen uint
) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	vkPtr := (*C.uchar)(unsafe.Pointer(&verificationKeyBuffer[0]))
	kzgParamPtr := (C.uchar)(unsafe.Pointer(&kzgParamBuffer[0]))
	publicInputBuffer := (C.uchar)(unsafe.Pointer(&publicInputBuffer[0]))
	return (bool)(C.verify_halo2_kzg_proof_ffi(proofPtr, (C.uint)(proofLen), vkPtr, (C.uint)(vkLen), kzgParamBuffer, (C.uint)(kzgParamLen), publicInputBuffer, (C.uint)(publicInputLen)))
}