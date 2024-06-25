//! A simple script to generate and verify the proof of a given program.

use std::io;
use sp1_sdk::{ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();

    let question = "What's your ethereum address?";
    println!("{}", question);
    let name = read_answer();
    // Check if the input is a valid ethereum address
    if !name.starts_with("0x") || name.len() != 42 {
        panic!("Invalid ethereum address");
    }
    stdin.write(&name);

    let mut user_awnsers = "".to_string();
    let question1 = "What is the capital of France?";
    let answers1 = ["London", "Berlin", "Paris"];
    user_awnsers.push(ask_question(question1, &answers1));

    let question2 = "What is the first letter of the alphabet?";
    let answers2 = ["A", "C", "B"];
    user_awnsers.push(ask_question(question2, &answers2));

    let question3 = "What is the second planet from the sun?";
    let answers3 = ["Mars", "Venus", "Mercury"];
    user_awnsers.push(ask_question(question3, &answers3));

    let question4 = "What is the color of the sky on a clear day?";
    let answers4 = ["Green", "Red", "Blue"];
    user_awnsers.push(ask_question(question4, &answers4));

    let question5 = "What is the largest ocean on Earth?";
    let answers5 = ["Atlantic", "Pacific", "Indian"];
    user_awnsers.push(ask_question(question5, &answers5));

    stdin.write(&user_awnsers);
    println!("Generating Proof ");

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    match client.prove_compressed(&pk, stdin) {
        Ok(proof) => {
            println!("Proof generated successfully. Verifying proof...");
            // Verify proof.
            client
                .verify_compressed(&proof, &vk)
                .expect("verification failed");

            // Save proof.
            proof
                .save("proof-with-io.json")
                .expect("saving proof failed");

            println!("Successfully generated and verified proof for the program!")
        }
        Err(_) => {
            println!("Proof generation failed. Incorrect answer");
            return;
        }
    }
}

fn ask_question(question: &str, answers: &[&str]) -> char {
    println!("{}", question);
    for (i, answer) in answers.iter().enumerate() {
        println!("{}. {}", (b'a' + i as u8) as char, answer);
    }

    return read_answer().chars().next()
        .expect("failed to get first char");
}

fn read_answer() -> String {
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read from stdin");
    answer.trim().to_string()
}

