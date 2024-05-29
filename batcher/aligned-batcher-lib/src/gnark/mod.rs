use crate::ProvingSystemId;

#[repr(C)]
pub struct GoSlice {
    data: *const u8,
    len: usize,
    cap: usize,
}

impl From<Vec<u8>> for GoSlice {
    fn from(v: Vec<u8>) -> Self {
        Self::from(&v)
    }
}

impl From<&Vec<u8>> for GoSlice {
    fn from(v: &Vec<u8>) -> Self {
        Self::from(v.as_slice())
    }
}

impl From<&[u8]> for GoSlice {
    fn from(v: &[u8]) -> Self {
        let len = v.len();
        let cap = v.len();
        let data = v.as_ptr();
        GoSlice { data, len, cap }
    }
}

pub fn verify_gnark(
    proving_system: &ProvingSystemId,
    proof: &[u8],
    public_input: &[u8],
    verification_key: &[u8],
) -> bool {
    let proof = GoSlice::from(proof);
    let public_input = GoSlice::from(public_input);
    let verification_key = GoSlice::from(verification_key);

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
        proof: &GoSlice,
        public_input: &GoSlice,
        verification_key: &GoSlice,
    ) -> bool;
    pub fn VerifyPlonkProofBN254(
        proof: &GoSlice,
        public_input: &GoSlice,
        verification_key: &GoSlice,
    ) -> bool;
    pub fn VerifyGroth16ProofBN254(
        proof: &GoSlice,
        public_input: &GoSlice,
        verification_key: &GoSlice,
    ) -> bool;
}
