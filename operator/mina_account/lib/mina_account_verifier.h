#include <stdbool.h>
#include <stdint.h>

int32_t verify_account_inclusion_ffi(unsigned char *proof_buffer,
                                     uint32_t proof_len,
                                     unsigned char *public_input_buffer,
                                     uint32_t public_input_len);
