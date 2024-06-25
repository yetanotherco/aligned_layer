//! A simple program to be proven inside the zkVM.
//! Consists in a 5 question multiple choice quiz
//! with 3 possible answers each.

#![no_main]

use tiny_keccak::{Hasher, Sha3};
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let name = sp1_zkvm::io::read::<String>();
    sp1_zkvm::io::commit(&name);

    let answers = sp1_zkvm::io::read::<String>();
    let mut sha3 = Sha3::v256();
    let mut output = [0u8; 32];

    sha3.update(&answers.as_bytes());

    sha3.finalize(&mut output);

    if output != [164,149,9,202,181,178,182,47,78,106,69,81,119,231,55,185,10,188,53,20,162,164,182,209,217,207,27,19,179,52,50,135] {
        panic!("Answers do not match");
    }
}
