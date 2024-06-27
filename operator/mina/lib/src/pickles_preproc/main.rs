use pickles::parse_and_verify;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let proof_file_path = args.get(1).ok_or("Error: No proof file path provided")?;

    parse_and_verify(proof_file_path)
}
