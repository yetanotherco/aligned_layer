package circom

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libcircom_verifier.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lcircom_verifier

#include "lib/circom.h"
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

// MaxPublicInputSize 4KB
const MaxPublicInputSize = 4 * 1024

// Merge all pointers into one array and send across interface
func VerifyCircomProof(
	proofBuffer [MaxProofSize]byte, proofLen_u32 uint32,
	vkBuffer [MaxVerifierKeySize]byte, vkLen_u32 uint32,
	publicInputBuffer [MaxPublicInputSize]byte, publicInputLen_u32 uint32,
) bool {
	// Cast data pointers to C-types
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	vkPtr := (*C.uchar)(unsafe.Pointer(&vkBuffer[0]))
	publicInputPtr := (*C.uchar)(unsafe.Pointer(&publicInputBuffer[0]))

	// Cast data length to C-types
	proofLen := (C.uint32_t)(proofLen_u32)
	vkLen := (C.uint32_t)(vkLen_u32)
	publicInputLen := (C.uint32_t)(publicInputLen_u32)

	return (bool)(C.verify_circom_proof_ffi(
		proofPtr, proofLen,
		vkPtr, vkLen,
		publicInputPtr, publicInputLen),
	)
}
