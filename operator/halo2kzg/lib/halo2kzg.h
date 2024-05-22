#include <stdbool.h>
#include <stdint.h>

bool verify_halo2_kzg_proof_ffi(unsigned char *proof_bytes, uint64_t proof_len,
                                unsigned char *constraint_system_bytes, uint64_t cs_len,
                                unsigned char *verifier_params_bytes, uint64_t vk_len,
                                unsigned char *kzg_params_bytes, uint64_t kzg_params_len,
                                unsigned char *public_input_bytes, uint64_t public_input_len);