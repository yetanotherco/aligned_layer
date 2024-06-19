use aligned_batcher_lib::types::ProvingSystemId;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ListRef {
    data: *const u8,
    len: usize,
}

impl From<Vec<u8>> for ListRef {
    fn from(v: Vec<u8>) -> Self {
        Self::from(v.as_slice())
    }
}

impl From<&Vec<u8>> for ListRef {
    fn from(v: &Vec<u8>) -> Self {
        Self::from(v.as_slice())
    }
}

impl From<&[u8]> for ListRef {
    fn from(v: &[u8]) -> Self {
        let len = v.len();
        let data = v.as_ptr().cast();
        ListRef { data, len }
    }
}

pub fn verify_gnark(
    proving_system: &ProvingSystemId,
    proof: &Vec<u8>,
    public_input: &Vec<u8>,
    verification_key: &Vec<u8>,
) -> bool {
    let proof = proof.into();
    let public_input = public_input.into();
    let verification_key = verification_key.into();

    match proving_system {
        ProvingSystemId::GnarkPlonkBn254 => unsafe {
            VerifyPlonkProofBN254(proof, public_input, verification_key)
        },
        ProvingSystemId::GnarkPlonkBls12_381 => unsafe {
            VerifyPlonkProofBLS12_381(proof, public_input, verification_key)
        },
        ProvingSystemId::Groth16Bn254 => unsafe {
            VerifyGroth16ProofBN254(proof, public_input, verification_key)
        },
        _ => panic!("Unsupported proving system"),
    }
}

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
