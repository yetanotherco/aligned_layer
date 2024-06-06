#include <stdbool.h>
#include <stdint.h>

bool verify_jolt_proof_ffi(unsigned char *proof_buffer, uint32_t proof_len,
                          unsigned char *elf_buffer, uint32_t elf_len,
                          unsigned char *commitment_input, uint32_t commitment_input_len);
