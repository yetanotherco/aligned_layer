use std::{io::Write, fs};

pub fn main() {
    let (prove_sha3, verify_sha3) = guest::build_sha3();

    let (program, _) = guest::preprocess_sha3();

    // Write elf to file outside of tmp directory
    let elf = fs::read(program.elf.unwrap()).unwrap();
    let mut file = fs::File::create("./sha3-guest.elf").unwrap();
    file.write_all(&elf).unwrap();

    let input: &[u8] = &[5u8; 32];
    let (output, proof) = prove_sha3(input);
    proof.save_to_file("./sha3-guest.proof").unwrap();
    let is_valid = verify_sha3(proof);

    println!("output: {}", hex::encode(output));
    println!("valid: {}", is_valid);
}
