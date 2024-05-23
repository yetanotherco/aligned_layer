#include <stdbool.h>
#include <stdint.h>

bool verify_sp1_proof_ffi(unsigned char *proof_buffer, uint64_t proof_len,
                          unsigned char *elf_buffer, uint64_t elf_len);
