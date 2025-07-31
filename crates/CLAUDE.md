# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

### Rust Workspace
- **Build all crates**: `cargo build` (from `/crates` directory)
- **Build specific crate**: `cargo build --manifest-path ./crates/[crate-name]/Cargo.toml`
- **Build with release optimization**: `cargo build --release`

### Batcher
- **Build**: `cargo build --manifest-path ./crates/batcher/Cargo.toml --release`
- **Run**: `cargo run --manifest-path ./crates/batcher/Cargo.toml --release -- --config ./config-files/config-batcher.yaml --env-file ./crates/batcher/.env`
- **Start locally**: `make batcher_start_local`

### CLI
- **Build**: `cd crates/cli && cargo build --release`
- **Install**: `cargo install --path crates/cli`
- **Install script**: `./crates/cli/install_aligned.sh`

### SDK
- **Build**: `cargo build --manifest-path ./crates/sdk/Cargo.toml`
- **Test**: `cargo test --manifest-path ./crates/sdk/Cargo.toml`

## Testing Commands

### Rust Tests
- **Run all tests**: `cargo test` (from `/crates` directory)
- **Run specific crate tests**: `cargo test --manifest-path ./crates/[crate-name]/Cargo.toml`
- **Run with release mode**: `cargo test --release`

### Go Tests
- **Run all Go tests**: `go test ./... -timeout 15m`
- **Run retry tests**: `cd core/ && go test -v -timeout 15m`

### FFI Tests
- **SP1 Rust FFI**: `cd operator/sp1/lib && RUST_MIN_STACK=83886080 cargo test --release`
- **RISC Zero Rust FFI**: `cd operator/risc_zero/lib && cargo test --release`
- **Merkle Tree FFI**: `cd operator/merkle_tree/lib && RUST_MIN_STACK=83886080 cargo test --release`

## Linting Commands

### Solidity Contracts
- **Lint contracts**: `cd contracts && npm run lint:sol`

### Rust (via Makefile targets)
- Check individual crate formatting: `cargo fmt --check --manifest-path ./crates/[crate-name]/Cargo.toml`
- Check individual crate linting: `cargo clippy --manifest-path ./crates/[crate-name]/Cargo.toml`

## Common Development Commands

### Dependencies
- **Install all dependencies**: `make deps`
- **Install Go dependencies**: `make go_deps`
- **Initialize submodules**: `make submodules`

### Development Environment
- **Start Anvil**: `make anvil_start`
- **Start full local environment**: `make setup_local_aligned_all`
- **Build all FFIs**: `make build_all_ffi`

### Proof Submission
- **Send SP1 proof**: `make batcher_send_sp1_task RPC_URL=http://localhost:8545 NETWORK=devnet`
- **Send RISC0 proof**: `make batcher_send_risc0_task RPC_URL=http://localhost:8545 NETWORK=devnet`
- **Send Gnark proofs**: `make batcher_send_gnark_plonk_bn254_task RPC_URL=http://localhost:8545 NETWORK=devnet`

## Architecture Overview

### Core Components

**Aligned Layer** is a verification layer for zero-knowledge proofs built on EigenLayer. The system consists of several key components:

1. **Batcher** (`crates/batcher/`): Aggregates multiple proofs into batches for efficient verification
   - Listens for WebSocket connections from clients
   - Collects verification data and batches them based on time/size thresholds
   - Submits batches to the verification layer

2. **SDK** (`crates/sdk/`): Provides client libraries for interacting with Aligned Layer
   - **Verification Layer**: Core verification functionality
   - **Aggregation Layer**: Handles proof aggregation modes
   - **Communication**: Protocol implementations for client-server communication
   - **Ethereum Integration**: Smart contract interfaces and utilities

3. **CLI** (`crates/cli/`): Command-line interface for submitting proofs and interacting with the system
   - Proof submission with various proving systems (SP1, RISC0, Gnark, Circom)
   - Balance queries and verification status checks
   - Batch verification data handling

4. **Task Sender** (`crates/task-sender/`): Utility for load testing and automated proof submission
   - Wallet generation and funding
   - Infinite proof submission with configurable parameters
   - Connection testing utilities

### Supported Proving Systems

The system supports multiple zero-knowledge proving systems:
- **SP1**: Succinct's zkVM proving system
- **RISC Zero**: General-purpose zkVM
- **Gnark**: Groth16 and PLONK protocols (BN254, BLS12-381)
- **Circom**: Circuit compiler with Groth16 backend

### Key Architectural Patterns

1. **Modular Design**: Each component (batcher, SDK, CLI) is a separate crate with clear boundaries
2. **Async/Await**: Heavy use of Tokio for asynchronous operations
3. **FFI Integration**: Foreign function interfaces for integrating with Go-based verifiers
4. **EigenLayer Integration**: Built as an AVS (Actively Validated Service) on EigenLayer
5. **Multi-Network Support**: Configurable for different networks (devnet, testnet, mainnet)

### Development Workflow

1. **Local Development**: Use `make anvil_start` to start local blockchain
2. **Component Testing**: Each crate can be built and tested independently
3. **Integration Testing**: Full system testing using Docker compose or Makefile targets
4. **Proof Generation**: Scripts in `scripts/test_files/` for generating test proofs

### Configuration Management

- **YAML Configuration**: Primary configuration files in `config-files/`
- **Environment Variables**: `.env` files for sensitive configuration
- **Network-Specific Config**: Separate configurations for different networks
- **Makefile Parameters**: Extensive use of Make variables for configuration