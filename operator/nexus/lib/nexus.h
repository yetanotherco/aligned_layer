#include <stdbool.h>
#include <stdint.h>

bool verify_nexus_proof_ffi(unsigned char *proof_buffer, uint32_t proof_len,
                          unsigned char *params_buffer, uint32_t params_len,
                          unsigned char *key_buffer, uint32_t key_len);
