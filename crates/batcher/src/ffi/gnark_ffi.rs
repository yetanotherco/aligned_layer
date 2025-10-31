use crate::ffi::list_ref::ListRef;

extern "C" {
    pub fn VerifyGnarkPlonkProofBLS12_381(
        proof: ListRef,
        public_input: ListRef,
        verification_key: ListRef,
    ) -> bool;
    pub fn VerifyGnarkPlonkProofBN254(
        proof: ListRef,
        public_input: ListRef,
        verification_key: ListRef,
    ) -> bool;
    pub fn VerifyGnarkGroth16ProofBN254(
        proof: ListRef,
        public_input: ListRef,
        verification_key: ListRef,
    ) -> bool;
}
