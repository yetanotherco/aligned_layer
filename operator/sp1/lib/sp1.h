#include <stdbool.h>
#include <stdint.h>

int32_t verify_sp1_proof_ffi(unsigned char *proof_buffer, uint32_t proof_len,
                                unsigned char *public_inputs_buffer, uint32_t public_inputs_len,
                                unsigned char *elf_buffer, uint32_t elf_len);
