#include <stdbool.h>

bool verify_protocol_state_proof_ffi(unsigned char *proof_buffer,
                                     unsigned int proof_len,
                                     unsigned char *public_input_buffer,
                                     unsigned int public_input_len);