package halo2ipa

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libhalo2ipa_verifier.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lhalo2ipa_verifier

#include "lib/halo2ipa.h"
*/
import "C"
import "unsafe"

// MaxProofSize 4KB
const MaxProofSize =  8 * 1024;

// MaxProofSize 4KB
const MaxParamsSize =  8 * 1024;

// MaxConstraintSystemSize 2KB
const MaxConstraintSystemSize = 2 * 1024;

// MaxVerificationKeySize 1KB
const MaxVerifierKeySize = 1024;

// MaxKzgParamsSize 4KB
const MaxIpaParamsSize = 4 * 1024;

// MaxPublicInputSize 4KB
const MaxPublicInputSize = 4 * 1024;

//Merge all pointers into one array and send across interface
func VerifyHalo2IpaProof(
	proofBuffer [MaxProofSize]byte, proofLen uint, 
	csBuffer [MaxConstraintSystemSize]byte, csLen uint, 
	vkBuffer [MaxVerifierKeySize]byte, vkLen uint, 
	ipaParamBuffer [MaxIpaParamsSize]byte, ipaParamLen uint, 
	publicInputBuffer [MaxPublicInputSize]byte, publicInputLen uint,
) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	csPtr := (*C.uchar)(unsafe.Pointer(&csBuffer[0]))
	vkPtr := (*C.uchar)(unsafe.Pointer(&vkBuffer[0]))
	ipaParamPtr := (*C.uchar)(unsafe.Pointer(&ipaParamBuffer[0]))
	publicInputPtr := (*C.uchar)(unsafe.Pointer(&publicInputBuffer[0]))

	return (bool)(C.verify_halo2_ipa_proof_ffi(
		proofPtr, (C.ulonglong)(proofLen), 
		csPtr, (C.ulonglong)(csLen),
		vkPtr, (C.ulonglong)(vkLen),
		ipaParamPtr, (C.ulonglong)(ipaParamLen),
		publicInputPtr, (C.ulonglong)(publicInputLen)),
	)
}