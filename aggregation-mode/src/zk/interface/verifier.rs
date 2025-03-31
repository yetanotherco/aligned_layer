use crate::zk::backends::sp1::{self, SP1Proof, SP1VerificationError};

pub enum Proof {
    SP1(SP1Proof),
}

pub enum VerificationError {
    SP1(SP1VerificationError),
}

pub fn verify_proof(proof: &Proof) -> Result<(), VerificationError> {
    match proof {
        Proof::SP1(proof) => sp1::verify(proof).map_err(VerificationError::SP1),
    }
}
