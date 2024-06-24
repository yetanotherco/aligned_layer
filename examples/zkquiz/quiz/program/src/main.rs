//! A simple program to be proven inside the zkVM.
//! Consists in a 5 question multiple choice quiz
//! with 3 possible answers each.

#![no_main]

use std::io;
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let name = sp1_zkvm::io::read::<String>();
    sp1_zkvm::io::commit(&name);

    check_answer('c');
    check_answer('a');
    check_answer('b');
    check_answer('c');
    check_answer('b');
}

fn check_answer(correct_answer: char) {
    let answer = sp1_zkvm::io::read::<char>();
    assert_eq!(answer, correct_answer, "Wrong answer");
}
