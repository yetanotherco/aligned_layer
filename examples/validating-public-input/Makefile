generate_risc_zero_fibonacci_proof:
	@cd risc_zero/fibonacci_proof_generator && \
		cargo run --release && \
		echo "Fibonacci proof, pub input and image ID generated in risc_zero folder"

submit_fibonacci_proof:
	@cd aligned-integration && \
	RUST_LOG=info cargo run --release

deploy_fibonacci_validator:
	@. ./contracts/.env && . ./contracts/deploy.sh
