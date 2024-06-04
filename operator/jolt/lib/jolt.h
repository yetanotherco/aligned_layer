#include <stdbool.h>
#include <stdint.h>

bool verify_jolt_proof_ffi(unsigned char *proof_buffer, uint32_t proof_len,
                          unsigned char *info_buffer, uint32_t info_len);
