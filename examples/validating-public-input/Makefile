generate_risc0_fibonacci_proof:
	@cd risc_zero/fibonacci_proof_generator && \
		cargo run --release && \
		echo "Fibonacci proof, pub input and image ID generated in risc_zero folder"

generate_sp1_fibonacci_proof:
	@cd sp1/fibonacci/script && \
		cargo run --release && \
		echo "Fibonacci proof, pub input and image ID generated in sp1 folder"

prove_zkRust_risc0_fibonacci_devnet:
	zkRust prove-risc0 \
		./zkRust/fibonacci \
		--network devnet \
		--submit-to-aligned \
		--keystore-path $(KEYSTORE_PATH) \
		--rpc-url "http://localhost:8545" \
		--batcher-url "ws://localhost:8080" \
		--aligned-verification-data-path ./aligned-integration/batch_inclusion_data

prove_zkRust_sp1_fibonacci_devnet:
	zkRust prove-sp1 \
		./zkRust/fibonacci \
		--network devnet \
		--submit-to-aligned \
		--keystore-path $(KEYSTORE_PATH) \
		--rpc-url "http://localhost:8545" \
		--batcher-url "ws://localhost:8080" \
		--aligned-verification-data-path ./aligned-integration/batch_inclusion_data

prove_zkRust_risc0_fibonacci_testnet:
	zkRust prove-risc0 \
		./zkRust/fibonacci \
		--submit-to-aligned \
		--keystore-path $(KEYSTORE_PATH) \
		--aligned-verification-data-path ./aligned-integration/batch_inclusion_data

prove_zkRust_sp1_fibonacci_testnet:
	zkRust prove-sp1 \
		./zkRust/fibonacci \
		--submit-to-aligned \
		--keystore-path $(KEYSTORE_PATH) \
		--aligned-verification-data-path ./aligned-integration/batch_inclusion_data

submit_fibonacci_sp1_proof_devnet:
	@cd aligned-integration && \
	RUST_LOG=info cargo run --release -- --proving-system "SP1" --network "devnet" --batcher-url "ws://localhost:8080" --rpc-url "http://localhost:8545"

submit_fibonacci_sp1_proof:
	@cd aligned-integration && \
	RUST_LOG=info cargo run --release -- --keystore-path $(KEYSTORE_PATH) --proving-system "SP1"

submit_fibonacci_risc0_proof_devnet:
	@cd aligned-integration && \
	RUST_LOG=info cargo run --release -- --proving-system "Risc0" --network "devnet" --batcher-url "ws://localhost:8080" --rpc-url "http://localhost:8545"

submit_fibonacci_risc0_proof:
	@cd aligned-integration && \
	RUST_LOG=info cargo run --release -- --keystore-path $(KEYSTORE_PATH) --proving-system "Risc0"

verify_sp1_batch_inclusion:
	@. ./contracts/.env  && . ./contracts/validate_batch_inclusion.sh $(FIBONACCI_VALIDATOR_ADDRESS) $(DATA_FILE_NAME) SP1

verify_risc0_batch_inclusion:
	@. ./contracts/.env  && . ./contracts/validate_batch_inclusion.sh $(FIBONACCI_VALIDATOR_ADDRESS) $(DATA_FILE_NAME) Risc0

verify_risc0_batch_inclusion_devnet:
	@. ./contracts/.env.devnet  && . ./contracts/validate_batch_inclusion.sh $(FIBONACCI_VALIDATOR_ADDRESS) $(DATA_FILE_NAME) Risc0

verify_sp1_batch_inclusion_devnet:
	@. ./contracts/.env.devnet  && . ./contracts/validate_batch_inclusion.sh $(FIBONACCI_VALIDATOR_ADDRESS) $(DATA_FILE_NAME) SP1

deploy_fibonacci_validator:
	@. ./contracts/.env && . ./contracts/deploy.sh

deploy_fibonacci_validator_devnet:
	@. ./contracts/.env.devnet && . ./contracts/deploy.sh
