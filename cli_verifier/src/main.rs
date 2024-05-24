use clap::{Arg, Command};
use rand::seq::SliceRandom;
use std::fs;
use std::io::Error;
use sp1_sdk::verify_sp1_proof_ffi;

fn main() {
    let matches = Command::new("Proving System CLI")
        .version("1.0")
        .author("Tu Nombre <tuemail@ejemplo.com>")
        .about("Verifica una prueba aleatoria usando SP1")
        .arg(Arg::new("input")
             .short('i')
             .long("input")
             .value_name("DIR")
             .help("Directorio con las pruebas")
             .takes_value(true))
        .get_matches();

    if let Some(input_dir) = matches.value_of("input") {
        match verify_random_proof(input_dir) {
            Ok(_) => println!("Verification completed successfully."),
            Err(e) => eprintln!("Error during verification: {}", e),
        }
    } else {
        eprintln!("Please provide an input directory.");
    }
}

fn verify_random_proof(dir_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let paths = fs::read_dir(dir_path)?
        .filter_map(Result::ok)
        .filter(|d| d.path().is_file())
        .collect::<Vec<_>>();

    if paths.is_empty() {
        return Err("No proof files found in the directory.".into());
    }

    let mut rng = rand::thread_rng();
    let random_path = paths.choose(&mut rng).ok_or("Failed to select a random proof.")?.path();

    let proof = fs::read(random_path)?;

    // Assuming ELF file is in a known location, adjust the path as needed
    let elf_path = "task_sender/test_examples/sp1/elf/riscv32im-succinct-zkvm-elf";
    let elf = fs::read(elf_path)?;

    // Verifica la prueba usando SP1
    let verification_result = verify_proof_sp1(&proof, &elf)?;

    println!("{}", verification_result);

    Ok(())
}

fn verify_proof_sp1(proof: &[u8], elf: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let mut proof_buffer = [0u8; 2 * 1024 * 1024];
    let proof_len = proof.len();
    proof_buffer[..proof_len].clone_from_slice(proof);

    let mut elf_buffer = [0u8; 1024 * 1024];
    let elf_len = elf.len();
    elf_buffer[..elf_len].clone_from_slice(elf);

    let result = unsafe { verify_sp1_proof_ffi(&proof_buffer, proof_len, &elf_buffer, elf_len) };

    if result {
        Ok("Proof verified successfully".to_string())
    } else {
        Err("Proof verification failed".into())
    }
}
