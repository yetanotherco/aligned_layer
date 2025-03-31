pub mod aggregator;
mod backends;

use backends::sp1::{self, SP1Proof, SP1VerificationError};

pub enum ZKVMEngine {
    SP1,
}

pub enum Proof {
    SP1(SP1Proof),
}

impl Proof {
    pub fn hash(&self) -> [u8; 32] {
        match self {
            Proof::SP1(proof) => proof.hash(),
        }
    }
}

pub enum VerificationError {
    SP1(SP1VerificationError),
}

impl Proof {
    pub fn verify(&self, elf: &[u8]) -> Result<(), VerificationError> {
        match self {
            Proof::SP1(proof) => sp1::verify(proof, elf).map_err(VerificationError::SP1),
        }
    }
}
