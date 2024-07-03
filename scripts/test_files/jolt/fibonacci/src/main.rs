use std::{io::Write, fs};

pub fn main() {
    let (prove_fib, verify_fib) = guest::build_fib();

    let (program, _) = guest::preprocess_fib();

    // Write elf to file outside of tmp directory
    let elf = fs::read(program.elf.unwrap()).unwrap();
    let mut file = fs::File::create("./fibonacci-guest.elf").unwrap();
    file.write_all(&elf).unwrap();

    let (output, proof) = prove_fib(50);
    proof.save_to_file("./fibonacci-guest.proof").unwrap();
    let is_valid = verify_fib(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);
}
