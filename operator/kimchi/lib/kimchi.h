#include <stdbool.h>

bool verify_kimchi_proof_ffi(unsigned char *proof_buffer,
                             unsigned int proof_len,
                             unsigned char *pub_input_buffer,
                             unsigned int pub_input_len);
