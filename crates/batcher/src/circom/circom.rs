use aligned_sdk::common::types::ProvingSystemId;

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

pub fn verify_circom(
    proving_system: &ProvingSystemId,
    proof: &Vec<u8>,
    public_input: &Vec<u8>,
    verification_key: &Vec<u8>,
) -> bool {
    let proof = proof.into();
    let public_input = public_input.into();
    let verification_key = verification_key.into();

    match proving_system {
        ProvingSystemId::CircomGroth16Bn128 => unsafe {
            VerifyCircomGroth16ProofBN128(proof, public_input, verification_key)
        },
        _ => false,
    }
}

extern "C" {
    pub fn VerifyCircomGroth16ProofBN128(
        proof: ListRef,
        public_input: ListRef,
        verification_key: ListRef,
    ) -> bool;
}

