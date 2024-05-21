package halo2kzg

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libhalo2kzg_verifier.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lhalo2kzg_verifier

#include "lib/halo2kzg.h"
*/
import "C"
import "fmt"
import "unsafe"

// MaxProofSize 2MB
const MaxProofSize = 2048;

// MaxVerificationKeySize 4 MB
const MaxParamsSize = 4 * 1024;

// MaxConstraintSystemSize 1 KB
const MaxConstraintSystemSize = 791;

// MaxVerificationKeySize 1 KB
const MaxVerifierKeySize = 518;

// MaxVerificationKeySize 4 MB
const MaxKzgParamsSize = 2308;

// MaxPublicInputSize 4 MB
const MaxPublicInputSize = 1024 * 1024;

//Merge all pointers into one array and send across interface
func VerifyHalo2KzgProof(
	proofBuffer [MaxProofSize]byte, proofLen uint, 
	csBuffer [MaxConstraintSystemSize]byte, csLen uint, 
	vkBuffer [MaxVerifierKeySize]byte, vkLen uint, 
	kzgParamBuffer [MaxKzgParamsSize]byte, kzgParamLen uint, 
	publicInputBuffer [MaxPublicInputSize]byte, publicInputLen uint,
) bool {
	fmt.Printf("Public Input Len: %d\n", publicInputLen)
	(C.print_pub_len((C.uint)(publicInputLen)))
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	csPtr := (*C.uchar)(unsafe.Pointer(&csBuffer[0]))
	vkPtr := (*C.uchar)(unsafe.Pointer(&vkBuffer[0]))
	kzgParamPtr := (*C.uchar)(unsafe.Pointer(&kzgParamBuffer[0]))
	publicInputPtr := (*C.uchar)(unsafe.Pointer(&publicInputBuffer[0]))

	/*
	publicInputPtr = proofPtr;	
	publicInputLenForReal := uint(1)
	*/

	return (bool)(C.verify_halo2_kzg_proof_ffi(
		proofPtr, (C.uint)(proofLen), 
		csPtr, (C.uint)(csLen),
		vkPtr, (C.uint)(vkLen),
		kzgParamPtr, (C.uint)(kzgParamLen),
		// 4 or 8 bytes
		publicInputPtr, (C.uint)(publicInputLen)),
	)
}