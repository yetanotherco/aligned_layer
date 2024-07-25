#include <stdbool.h>
#include <stdint.h>

bool verify_circom_proof_ffi(unsigned char *proof_bytes, uint32_t proof_len,
                                unsigned char *verifier_params_bytes, uint32_t vk_len,
                                unsigned char *public_input_bytes, uint32_t public_input_len);