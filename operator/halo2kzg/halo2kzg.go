package halo2kzg

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libhalo2kzg_verifier.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lhalo2kzg_verifier

#include "lib/halo2kzg.h"
*/
import "C"
import "unsafe"

// MaxProofSize 2MB
const MaxProofSize = 2048;

// MaxVerificationKeySize 1 KB
const MaxVerifierKeySize = 518;

// MaxVerificationKeySize 4 MB
const MaxKzgParamsSize = 2308;

// MaxVerificationKeySize 4 MB
const MaxPublicInputSize = 4 * 1024 * 1024;

func VerifyHalo2KzgProof(
	proofBuffer [MaxProofSize]byte, proofLen uint, 
	vkBuffer [MaxVerifierKeySize]byte, vkLen uint, 
	kzgParamBuffer [MaxKzgParamsSize]byte, kzgParamLen uint, 
	publicInputBuffer [MaxPublicInputSize]byte, publicInputLen uint,
) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	vkPtr := (*C.uchar)(unsafe.Pointer(&vkBuffer[0]))
	kzgParamPtr := (*C.uchar)(unsafe.Pointer(&kzgParamBuffer[0]))
	publicInputPtr := (*C.uchar)(unsafe.Pointer(&publicInputBuffer[0]))
	return (bool)(C.verify_halo2_kzg_proof_ffi(
		proofPtr, (C.uint)(proofLen), 
		vkPtr, (C.uint)(vkLen),
		kzgParamPtr, (C.uint)(kzgParamLen),
		publicInputPtr, (C.uint)(publicInputLen)),
	)
}