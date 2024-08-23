package halo2ipa

/*
#cgo linux LDFLAGS: ${SRCDIR}/lib/libhalo2ipa_verifier.a -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition
#cgo darwin LDFLAGS: -L./lib -lhalo2ipa_verifier

#include "lib/halo2ipa.h"
*/
import "C"
import "unsafe"

func VerifyHalo2IpaProof(
	proofBuffer []byte, proofLen uint32,
	paramsBuffer []byte, paramsLen uint32,
	publicInputBuffer []byte, publicInputLen uint32,
) bool {
	/*
		For Halo2 the `paramsBuffer` contains the serialized cs, vk, and params with there respective sizes serialized as u32 values (4 bytes) => 3 * 4 bytes = 12 followed by the concatenated variable length buffers:
		We therefore require that the `paramsBuffer` is greater than 12 bytes and treat the case that buffer lengths and buffers themselves are 0 size as false.
		[ cs_len | vk_len | vk_params_len | cs_bytes | vk_bytes | vk_params_bytes ].
	*/
	if len(proofBuffer) == 0 || len(paramsBuffer) <= 12 || len(publicInputBuffer) == 0 {
		return false
	}

	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	paramsPtr := (*C.uchar)(unsafe.Pointer(&paramsBuffer[0]))
	publicInputPtr := (*C.uchar)(unsafe.Pointer(&publicInputBuffer[0]))

	return (bool)(C.verify_halo2_ipa_proof_ffi(
		proofPtr, (C.uint32_t)(proofLen),
		paramsPtr, (C.uint32_t)(paramsLen),
		publicInputPtr, (C.uint32_t)(publicInputLen),
	))
}
