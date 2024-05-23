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

extern "C" {
    pub fn VerifyPlonkProofBLS12_381(proof:&GoSlice, public_input:&GoSlice, verification_key:&GoSlice) -> bool;
    pub fn VerifyPlonkProofBN254(proof:&GoSlice, public_input:&GoSlice, verification_key:&GoSlice) -> bool;
    pub fn VerifyGroth16ProofBN254(proof:&GoSlice, public_input:&GoSlice, verification_key:&GoSlice) -> bool;
}