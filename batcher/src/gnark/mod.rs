use crate::types::ProvingSystemId;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct DataView {
    ptr: *const (),
    len: usize,
}

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct SliceRef(DataView);

impl From<Vec<u8>> for SliceRef {
    fn from(v: Vec<u8>) -> Self {
        Self::from(v.as_slice())
    }
}

impl From<&Vec<u8>> for SliceRef {
    fn from(v: &Vec<u8>) -> Self {
        Self::from(v.as_slice())
    }
}

impl From<&[u8]> for SliceRef {
    fn from(v: &[u8]) -> Self {
        let len = v.len();
        let ptr = v.as_ptr().cast();
        SliceRef(DataView{ptr, len})
    }
}

pub fn verify_gnark(
    proving_system: &ProvingSystemId,
    proof: &[u8],
    public_input: &[u8],
    verification_key: &[u8],
) -> bool {
    let proof = SliceRef::from(proof);
    let public_input = SliceRef::from(public_input);
    let verification_key = SliceRef::from(verification_key);

    match proving_system {
        ProvingSystemId::GnarkPlonkBn254 => unsafe {
            VerifyPlonkProofBN254(&proof, &public_input, &verification_key)
        },
        ProvingSystemId::GnarkPlonkBls12_381 => unsafe {
            VerifyPlonkProofBLS12_381(&proof, &public_input, &verification_key)
        },
        ProvingSystemId::Groth16Bn254 => unsafe {
            VerifyGroth16ProofBN254(&proof, &public_input, &verification_key)
        },
        _ => panic!("Unsupported proving system"),
    }
}

extern "C" {
    pub fn VerifyPlonkProofBLS12_381(
        proof: &SliceRef,
        public_input: &SliceRef,
        verification_key: &SliceRef,
    ) -> bool;
    pub fn VerifyPlonkProofBN254(
        proof: &SliceRef,
        public_input: &SliceRef,
        verification_key: &SliceRef,
    ) -> bool;
    pub fn VerifyGroth16ProofBN254(
        proof: &SliceRef,
        public_input: &SliceRef,
        verification_key: &SliceRef,
    ) -> bool;
}
