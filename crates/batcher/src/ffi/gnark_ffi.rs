use crate::ffi::list_ref::ListRef;

extern "C" {
    pub fn VerifyPlonkProofBLS12_381(
        proof: ListRef,
        public_input: ListRef,
        verification_key: ListRef,
    ) -> bool;
    pub fn VerifyPlonkProofBN254(
        proof: ListRef,
        public_input: ListRef,
        verification_key: ListRef,
    ) -> bool;
    pub fn VerifyGroth16ProofBN254(
        proof: ListRef,
        public_input: ListRef,
        verification_key: ListRef,
    ) -> bool;
}
