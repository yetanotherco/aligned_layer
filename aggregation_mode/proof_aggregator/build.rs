use risc0_build::{DockerOptionsBuilder, GuestOptionsBuilder};
use std::collections::HashMap;
use std::path::PathBuf;

// This allows us to skip the guest build in CI or local environments where it's not needed (reducing the build time)
// Note: To use this flag, the aggregation programs should be already compiled, otherwise the compilation will be done anyway.
fn should_skip_build() -> bool {
    if std::env::var("SKIP_AGG_PROGRAMS_BUILD")
        .map(|v| v == "1")
        .unwrap_or(false)
    {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let methods_path = PathBuf::from(out_dir).join("methods.rs");

        methods_path.exists()
    } else {
        false
    }
}

// Reference: https://docs.succinct.xyz/docs/sp1/writing-programs/compiling#advanced-build-options-1
fn main() {
    if should_skip_build() {
        println!("cargo:warning=SKIP_AGG_PROGRAMS_BUILD=1: methods.rs already exists, skipping aggregation programs build");
        return;
    } else {
        println!("cargo:warning=SKIP_AGG_PROGRAMS_BUILD=1 set, but path does not exist, running full build");
    }

    sp1_build::build_program_with_args("./aggregation_programs/sp1", {
        sp1_build::BuildArgs {
            output_directory: Some("./aggregation_programs/sp1/elf".to_string()),
            binaries: vec![
                "sp1_user_proofs_aggregator_program".into(),
                "sp1_chunk_aggregator_program".into(),
            ],
            // We use Docker to generate a reproducible ELF that will be identical across all platforms
            // (https://docs.succinct.xyz/docs/sp1/writing-programs/compiling#production-builds)
            docker: true,
            ..Default::default()
        }
    });

    // With this containerized build process, we ensure that all builds of the guest code,
    // regardless of the machine or local environment, will produce the same ImageID
    let docker_options = DockerOptionsBuilder::default().build().unwrap();
    // Reference: https://github.com/risc0/risc0/blob/main/risc0/build/src/config.rs#L73-L90
    let guest_options = GuestOptionsBuilder::default()
        .use_docker(docker_options)
        .build()
        .unwrap();

    risc0_build::embed_methods_with_options(HashMap::from([(
        "risc0_aggregation_program",
        guest_options,
    )]));

    // Steps followed from https://0xpolygonhermez.github.io/zisk/getting_started/writing_programs.html#build

    // build.rs runs a subprocess without the shell's rustup selection; set the toolchain
    // explicitly so cargo-zisk uses Zisk's rustc instead of the host toolchain.
    let zisk_rustc_path = rustc_path_for("zisk");

    let mut build_command = std::process::Command::new("cargo-zisk");

    let mut user_proof_aggregator_rom_setup_command = std::process::Command::new("cargo-zisk");
    let mut chunk_aggregator_rom_setup_command = std::process::Command::new("cargo-zisk");

    let mut user_proof_aggregator_rom_vk_command = std::process::Command::new("cargo-zisk");
    let mut chunk_aggregator_rom_vk_command = std::process::Command::new("cargo-zisk");

    // Zisk build elf command
    build_command
        .env("RUSTC", &zisk_rustc_path)
        .args(["build", "--release"])
        .current_dir("aggregation_programs/zisk/");

    let build_status = build_command
        .status()
        .expect("Failed to execute zisk build command");

    if !build_status.success() {
        panic!("Failed to build zisk elfs");
    }

    // Zisk rom-setup commands
    let user_proof_aggregator_rom_setup_status = user_proof_aggregator_rom_setup_command
        .args([
            "rom-setup",
            "--elf",
            "./target/riscv64ima-zisk-zkvm-elf/release/zisk_user_proofs_aggregator_program",
        ])
        .env("RUSTC", &zisk_rustc_path)
        .current_dir("./aggregation_programs/")
        .status()
        .unwrap();

    if !user_proof_aggregator_rom_setup_status.success() {
        panic!("Failed to execute rom-setup command on user proof aggregator program");
    }

    let chunk_aggregator_rom_setup_status = chunk_aggregator_rom_setup_command
        .args([
            "rom-setup",
            "--elf",
            "./target/riscv64ima-zisk-zkvm-elf/release/zisk_chunk_aggregator_program",
        ])
        .env("RUSTC", &zisk_rustc_path)
        .current_dir("./aggregation_programs/")
        .status()
        .unwrap();

    if !chunk_aggregator_rom_setup_status.success() {
        panic!("Failed to execute rom-setup command on chunk aggregator program");
    }

    // Zisk rom-vkey commands
    let user_proofs_aggregator_rom_vkey_status = user_proof_aggregator_rom_vk_command
        .args([
            "rom-vkey",
            "--elf",
            "./target/riscv64ima-zisk-zkvm-elf/release/zisk_user_proofs_aggregator_program",
            "-o",
            "zisk/vk/zisk_user_proofs_aggregator_program",
        ])
        .env("RUSTC", &zisk_rustc_path)
        .current_dir("./aggregation_programs/")
        .status()
        .unwrap();

    if !user_proofs_aggregator_rom_vkey_status.success() {
        panic!("Failed to execute rom-vkey command on user proofs aggregator program");
    }

    let chunk_aggregator_rom_vkey_status = chunk_aggregator_rom_vk_command
        .args([
            "rom-vkey",
            "--elf",
            "./target/riscv64ima-zisk-zkvm-elf/release/zisk_chunk_aggregator_program",
            "-o",
            "zisk/vk/zisk_chunk_aggregator_program",
        ])
        .env("RUSTC", &zisk_rustc_path)
        .current_dir("./aggregation_programs/")
        .status()
        .unwrap();

    if !chunk_aggregator_rom_vkey_status.success() {
        panic!("Failed to execute rom-vkey command on chunk aggregator program");
    }

    let _ = std::fs::create_dir("./aggregation_programs/zisk/elf");

    std::fs::copy(
        "./aggregation_programs/target/riscv64ima-zisk-zkvm-elf/release/zisk_user_proofs_aggregator_program",
        "./aggregation_programs/zisk/elf/zisk_user_proofs_aggregator_program",
    )
    .expect("Could not zisk_user_proofs_aggregator_program elf to aggregation_programs/zisk/elf directory");

    std::fs::copy(
        "./aggregation_programs/target/riscv64ima-zisk-zkvm-elf/release/zisk_chunk_aggregator_program",
        "./aggregation_programs/zisk/elf/zisk_chunk_aggregator_program",
    )
    .expect("Could not zisk_chunk_aggregator_program elf to aggregation_programs/zisk/elf directory");
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
