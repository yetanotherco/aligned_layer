use agg_mode_sdk::blockchain::AggregationModeProvingSystem;
use sha3::{Digest, Keccak256};

pub const VADCOP_FINAL_VERKEY_BIN: &[u8] =
    include_bytes!("../../aggregation_programs/zisk/vk/vadcop_final.verkey.bin");

pub const USER_PROOFS_PROGRAM_ROM_VK_BYTES: &[u8] =
    include_bytes!("../../aggregation_programs/zisk/vk/zisk_user_proofs_aggregator_program");

pub const CHUNK_PROGRAM_ROM_VK_BYTES: &[u8] =
    include_bytes!("../../aggregation_programs/zisk/vk/zisk_chunk_aggregator_program");

pub const USER_PROOFS_PROGRAM_ROM_VK: [u64; 4] =
    vk_bytes_to_u64_4(USER_PROOFS_PROGRAM_ROM_VK_BYTES);
pub const CHUNK_PROGRAM_ROM_VK: [u64; 4] = vk_bytes_to_u64_4(CHUNK_PROGRAM_ROM_VK_BYTES);

// Directory where zisk aggregation programs are located (relative to repo root, intended to be run from root with make proof_aggregator_start)
const ZISK_PROGRAMS_DIR: &str = "aggregation_mode/proof_aggregator/aggregation_programs/zisk";

// ELF files for zisk programs (relative to ZISK_PROGRAMS_DIR)
const USER_PROOFS_ELF_PATH: &str = "./elf/zisk_user_proofs_aggregator_program";
const CHUNK_ELF_PATH: &str = "./elf/zisk_chunk_aggregator_program";

// Paths for cargo-zisk prove commands (relative to ZISK_PROGRAMS_DIR)
const INPUT_PATH: &str = "./input.bin";
const OUTPUT_PATH: &str = "./output";
const SNARK_OUTPUT_PATH: &str = "./snark_output";
const PROVING_KEY_SNARK_DIR: &str = ".zisk/provingKeySnark";

const fn vk_bytes_to_u64_4(bytes: &[u8]) -> [u64; 4] {
    let mut out = [0_u64; 4];
    let mut i = 0;
    while i < 4 {
        let base = i * 8;
        out[i] = u64::from_le_bytes([
            bytes[base],
            bytes[base + 1],
            bytes[base + 2],
            bytes[base + 3],
            bytes[base + 4],
            bytes[base + 5],
            bytes[base + 6],
            bytes[base + 7],
        ]);
        i += 1;
    }
    out
}

#[derive(Debug)]
pub struct ZiskSnarkProof {
    pub proof: Vec<u8>,
    pub public_values: Vec<u8>,
    pub vk: [u64; 4],
}

/// A Zisk STARK proof.
/// The proof bytes contain the rom_vk and the public inputs.
#[derive(Debug)]
pub struct ZiskStarkProof {
    proof: Vec<u8>,
}

impl ZiskStarkProof {
    pub fn new(proof: Vec<u8>) -> Self {
        Self { proof }
    }

    pub fn hash_proof(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(AggregationModeProvingSystem::ZISK.id_bytes());
        hasher.update(&self.proof);
        hasher.finalize().into()
    }
}

#[derive(Debug)]
pub enum AlignedZiskError {
    Aggregation(String),
    Serialization(String),
    Io(String),
}

impl From<std::io::Error> for AlignedZiskError {
    fn from(e: std::io::Error) -> Self {
        AlignedZiskError::Io(e.to_string())
    }
}

pub(crate) fn run_user_proofs_aggregator(
    proofs: &[ZiskStarkProof],
) -> Result<ZiskStarkProof, AlignedZiskError> {
    let zisk_rustc_path = rustc_path_for("zisk");
    let mut command = std::process::Command::new("cargo-zisk");

    let proofs: Vec<zisk_aggregation_program::ZiskProof> = proofs
        .iter()
        .map(|e| zisk_aggregation_program::ZiskProof {
            proof: e.proof.clone(),
        })
        .collect();

    let input = zisk_aggregation_program::UserProofsAggregatorInput::new(
        proofs,
        VADCOP_FINAL_VERKEY_BIN.to_vec(),
    );
    let input_bytes =
        bincode::serialize(&input).map_err(|e| AlignedZiskError::Serialization(e.to_string()))?;

    // Write input file to the zisk programs directory
    let input_file_path = format!("{ZISK_PROGRAMS_DIR}/input.bin");
    std::fs::write(&input_file_path, input_bytes.as_slice())?;

    let status = command
        .env("RUSTC", &zisk_rustc_path)
        .args([
            "prove",
            "-e",
            USER_PROOFS_ELF_PATH,
            "-i",
            INPUT_PATH,
            "-o",
            OUTPUT_PATH,
            "-a",
            "-y",
        ])
        .current_dir(ZISK_PROGRAMS_DIR)
        .status()?;

    if !status.success() {
        return Err(AlignedZiskError::Aggregation(format!(
            "cargo-zisk prove failed with exit code: {:?}",
            status.code()
        )));
    }

    let proof_path = format!("{ZISK_PROGRAMS_DIR}/output/vadcop_final_proof.bin");
    let proof_bytes = std::fs::read(&proof_path)?;
    let proof = ZiskStarkProof { proof: proof_bytes };

    Ok(proof)
}

pub(crate) fn run_chunk_aggregator(
    proofs: &[(ZiskStarkProof, Vec<[u8; 32]>)],
) -> Result<ZiskSnarkProof, AlignedZiskError> {
    let zisk_rustc_path = rustc_path_for("zisk");
    let mut command = std::process::Command::new("cargo-zisk");

    let proofs_and_leaves: Vec<(zisk_aggregation_program::ZiskProof, Vec<[u8; 32]>)> = proofs
        .iter()
        .map(|(proof, leaves)| {
            (
                zisk_aggregation_program::ZiskProof {
                    proof: proof.proof.clone(),
                },
                leaves.clone(),
            )
        })
        .collect();

    let input = zisk_aggregation_program::ChunkAggregatorInput {
        proofs_and_leaves_commitment: proofs_and_leaves,
        vk: VADCOP_FINAL_VERKEY_BIN.to_vec(),
    };
    let input_bytes =
        bincode::serialize(&input).map_err(|e| AlignedZiskError::Serialization(e.to_string()))?;

    // Write input file to the zisk programs directory
    let input_file_path = format!("{ZISK_PROGRAMS_DIR}/input.bin");
    std::fs::write(&input_file_path, input_bytes.as_slice())?;

    // generate stark proof
    let status = command
        .env("RUSTC", &zisk_rustc_path)
        .args([
            "prove",
            "-e",
            CHUNK_ELF_PATH,
            "-i",
            INPUT_PATH,
            "-o",
            OUTPUT_PATH,
            "-u",
            "-a",
            "-y",
            "-f",
        ])
        .current_dir(ZISK_PROGRAMS_DIR)
        .status()?;

    if !status.success() {
        return Err(AlignedZiskError::Aggregation(format!(
            "cargo-zisk prove (chunk) failed with exit code: {:?}",
            status.code()
        )));
    }

    // Files needed to generate snark proof
    let recursivef_path = format!("{ZISK_PROGRAMS_DIR}/recursivef.json");
    std::fs::File::create(&recursivef_path)?;
    let snark_output_dir = format!("{ZISK_PROGRAMS_DIR}/{SNARK_OUTPUT_PATH}");
    std::fs::create_dir_all(&snark_output_dir)?;
    let snark_output_proofs_dir = format!("{snark_output_dir}/proofs");
    std::fs::create_dir_all(&snark_output_proofs_dir)?;

    // wrap it to snark
    let stark_proof_path = format!("{OUTPUT_PATH}/vadcop_final_proof.bin");
    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let proving_key_path = format!("{home_dir}/{PROVING_KEY_SNARK_DIR}");
    let snark_status = {
        let mut run_snark = || {
            std::process::Command::new("cargo-zisk")
                .env("RUSTC", &zisk_rustc_path)
                .args([
                    "prove-snark",
                    "-p",
                    &stark_proof_path,
                    "-k",
                    &proving_key_path,
                    "-o",
                    SNARK_OUTPUT_PATH,
                ])
                .current_dir(ZISK_PROGRAMS_DIR)
                .status()
        };
        // Dark magic: the first run tends to fail, while the second succeeds.
        let _ = run_snark()?;
        run_snark()?
    };

    if !snark_status.success() {
        return Err(AlignedZiskError::Aggregation(format!(
            "cargo-zisk prove-snark failed with exit code: {:?}",
            snark_status.code()
        )));
    }

    let proof_path = format!("{ZISK_PROGRAMS_DIR}/snark_output/proofs/final_snark_proof.bin");
    let public_values_path =
        format!("{ZISK_PROGRAMS_DIR}/snark_output/proofs/final_snark_publics.bin");
    let proof_bytes = std::fs::read(&proof_path)?;
    let public_values_bytes = std::fs::read(&public_values_path)?;

    let proof = ZiskSnarkProof {
        proof: proof_bytes,
        public_values: public_values_bytes,
        vk: CHUNK_PROGRAM_ROM_VK,
    };

    Ok(proof)
}

fn rustc_path_for(toolchain: &str) -> std::path::PathBuf {
    let output = std::process::Command::new("rustup")
        .args(["which", "rustc", "--toolchain", toolchain])
        .output()
        .expect("failed to execute rustup");

    if !output.status.success() {
        panic!("rustup which rustc failed for toolchain {toolchain}");
    }

    std::path::PathBuf::from(String::from_utf8_lossy(&output.stdout).trim())
}
