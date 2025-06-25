use crate::ffi::list_ref::ListRef;

extern "C" {
    pub fn VerifyCircomGroth16ProofBN128(
        proof: ListRef,
        public_input: ListRef,
        verification_key: ListRef,
    ) -> bool;
}
