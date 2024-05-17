#include <stdbool.h>

bool verify_halo2_kzg_proof_ffi(unsigned char *proof_bytes, unsigned int proof_len,
                                unsigned char *verifier_params_bytes, unsigned int vk_len,
                                unsigned char *kzg_params_bytes, unsigned int kzg_params_len,
                                unsigned char *public_input_bytes, unsigned int public_input_len);