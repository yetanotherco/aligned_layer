#include <stdbool.h>
#include <stdint.h>

bool verify_halo2_ipa_proof_ffi(unsigned char *proof_bytes, uint32_t proof_len,
                                unsigned char *params_bytes, uint32_t params_len,
                                unsigned char *public_input_bytes, uint32_t public_input_len);
                                