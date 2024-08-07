package halo2kzg

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libhalo2kzg_verifier.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lhalo2kzg_verifier
#include "lib/halo2kzg.h"
*/
import "C"
import "unsafe"

// MaxProofSize 4KB
const MaxProofSize = 8 * 1024

// MaxProofSize 4KB
const MaxParamsSize = 8 * 1024

// MaxConstraintSystemSize 2KB
const MaxConstraintSystemSize = 2 * 1024

// MaxVerificationKeySize 1KB
const MaxVerifierKeySize = 1024

// MaxKzgParamsSize 4KB
const MaxKzgParamsSize = 4 * 1024

// MaxPublicInputSize 4KB
const MaxPublicInputSize = 4 * 1024

func VerifyHalo2KzgProof(
	proofBuffer [MaxProofSize]byte, proofLen_u32 uint32,
	csBuffer [MaxConstraintSystemSize]byte, csLen_u32 uint32,
	vkBuffer [MaxVerifierKeySize]byte, vkLen_u32 uint32,
	kzgParamBuffer [MaxKzgParamsSize]byte, kzgParamLen_u32 uint32,
	publicInputBuffer [MaxPublicInputSize]byte, publicInputLen_u32 uint32,
) bool {
	if len(proofBuffer) == 0 || len(csBuffer) == 0 || len(vkBuffer) == 0 || len(kzgParamBuffer) == 0 || len(publicInputBuffer) == 0 {
		return false
	}

	// Cast data pointers to C-types
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	csPtr := (*C.uchar)(unsafe.Pointer(&csBuffer[0]))
	vkPtr := (*C.uchar)(unsafe.Pointer(&vkBuffer[0]))
	kzgParamPtr := (*C.uchar)(unsafe.Pointer(&kzgParamBuffer[0]))
	publicInputPtr := (*C.uchar)(unsafe.Pointer(&publicInputBuffer[0]))

	// Cast data lengths to C-Types
	proofLen := (C.uint32_t)(proofLen_u32)
	csLen := (C.uint32_t)(csLen_u32)
	vkLen := (C.uint32_t)(vkLen_u32)
	kzgParamLen := (C.uint32_t)(kzgParamLen_u32)
	publicInputLen := (C.uint32_t)(publicInputLen_u32)

	return (bool)(C.verify_halo2_kzg_proof_ffi(
		proofPtr, proofLen,
		csPtr, csLen,
		vkPtr, vkLen,
		kzgParamPtr, kzgParamLen,
		publicInputPtr, publicInputLen),
	)
}
