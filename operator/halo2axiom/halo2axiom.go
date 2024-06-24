package halo2axiom

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libhalo2axiom_verifier.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lhalo2axiom_verifier
#include "lib/halo2axiom.h"
*/
import "C"
import "unsafe"

// MaxProofSize 4KB
const MaxProofSize =  8 * 1024;

// MaxProofSize 4KB
const MaxParamsSize =  8 * 1024;

// MaxVerificationKeySize 1KB
const MaxVerifierKeySize = 1024;

// MaxKzgParamsSize 4KB
const MaxKzgParamsSize = 4 * 1024;

// MaxPublicInputSize 4KB
const MaxPublicInputSize = 4 * 1024;

func VerifyHalo2AxiomProof(
	proofBuffer [MaxProofSize]byte, proofLen_u32 uint32, 
	vkBuffer [MaxVerifierKeySize]byte, vkLen_u32 uint32, 
	kzgParamBuffer [MaxKzgParamsSize]byte, kzgParamLen_u32 uint32, 
	publicInputBuffer [MaxPublicInputSize]byte, publicInputLen_u32 uint32,
) bool {
	// Cast data pointers to C-types
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	vkPtr := (*C.uchar)(unsafe.Pointer(&vkBuffer[0]))
	kzgParamPtr := (*C.uchar)(unsafe.Pointer(&kzgParamBuffer[0]))
	publicInputPtr := (*C.uchar)(unsafe.Pointer(&publicInputBuffer[0]))

	// Cast data lengths to C-Types
	proofLen := (C.uint32_t)(proofLen_u32)
	vkLen := (C.uint32_t)(vkLen_u32)
	kzgParamLen := (C.uint32_t)(kzgParamLen_u32)
	publicInputLen := (C.uint32_t)(publicInputLen_u32) 

	return (bool)(C.verify_halo2_axiom_proof_ffi(
		proofPtr, proofLen, 
		vkPtr, vkLen,
		kzgParamPtr, kzgParamLen,
		publicInputPtr, publicInputLen),
	)
}