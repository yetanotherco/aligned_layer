#include <stdexcept>

#include "verifier.h"
extern "C" {
    bool verify_circom_proof_ffi(unsigned char *proof, uint32_t proof_len,
                                unsigned char *key, uint32_t key_len,
                                unsigned char *public_inputs, uint32_t public_input_len) {
        
        char errorMessage[256];

        const int error = groth16_verify(proof.dataAsString().c_str(),
                                         public_inputs.dataAsString().c_str(),
                                         key.dataAsString().c_str(),
                                         errorMessage, sizeof(errorMessage));

        if (error == VERIFIER_VALID_PROOF) {

            std::cerr << "Result: Valid proof" << std::endl;
            return EXIT_SUCCESS;

        } else if (error == VERIFIER_INVALID_PROOF) {

            std::cerr << "Result: Invalid proof" << std::endl;
            return EXIT_FAILURE;

        } else {
            std::cerr << "Error: " << errorMessage << '\n';
            return EXIT_FAILURE;
        }

    }
}