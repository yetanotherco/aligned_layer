//! A simple script to generate and verify the proof of a given program.

use std::io;
use sp1_sdk::{ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();

    let question = "What's your name?";
    println!("{}", question);
    let name = read_answer();
    stdin.write(&name);

    let question1 = "What is the capital of France?";
    let answers1 = ["London", "Berlin", "Paris"];
    ask_question(question1, &answers1, &mut stdin);

    let question2 = "What is the first letter of the alphabet?";
    let answers2 = ["A", "C", "B"];
    ask_question(question2, &answers2, &mut stdin);

    let question3 = "What is the second planet from the sun?";
    let answers3 = ["Mars", "Venus", "Mercury"];
    ask_question(question3, &answers3, &mut stdin);

    let question4 = "What is the color of the sky on a clear day?";
    let answers4 = ["Green", "Red", "Blue"];
    ask_question(question4, &answers4, &mut stdin);

    let question5 = "What is the largest ocean on Earth?";
    let answers5 = ["Atlantic", "Pacific", "Indian"];
    ask_question(question5, &answers5, &mut stdin);

    println!("Generating Proof");

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let proof = client.prove_compressed(&pk, stdin)
        .expect("proving failed");

    println!("Verifying proof");

    // Verify proof.
    client
        .verify_compressed(&proof, &vk)
        .expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}

fn ask_question(question: &str, answers: &[&str], sp1_stdin: &mut SP1Stdin) {
    println!("{}", question);
    for (i, answer) in answers.iter().enumerate() {
        println!("{}. {}", (b'a' + i as u8) as char, answer);
    }

    let choice = read_answer().chars().next()
        .expect("failed to get first char");
    sp1_stdin.write(&choice);
}

fn read_answer() -> String {
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read from stdin");
    answer.trim().to_string()
}

