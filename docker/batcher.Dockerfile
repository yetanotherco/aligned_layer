FROM ghcr.io/yetanotherco/aligned_layer/aligned_base:latest AS builder

RUN apt update -y
RUN apt install -y gcc

COPY go.mod .
COPY go.sum .
COPY batcher ./batcher

WORKDIR /aligned_layer/batcher/aligned-batcher

RUN go build -buildmode=c-archive -o libverifier.a ./gnark/verifier.go

WORKDIR /aligned_layer

COPY batcher/aligned-batcher/Cargo.toml batcher/aligned-batcher/Cargo.toml
COPY batcher/aligned/Cargo.toml batcher/aligned/Cargo.toml

RUN cargo build --manifest-path ./batcher/aligned-batcher/Cargo.toml --release
RUN cargo build --manifest-path ./batcher/aligned/Cargo.toml --release

COPY scripts/test_files/gnark_groth16_bn254_infinite_script scripts/test_files/gnark_groth16_bn254_infinite_script

RUN go build -o ./gnark_groth16_bn254_infinite_script scripts/test_files/gnark_groth16_bn254_infinite_script/cmd/main.go

RUN rm -rf operator/

FROM debian:bookworm-slim

WORKDIR /aligned_layer

COPY --from=builder /aligned_layer /aligned_layer
COPY --from=builder /aligned_layer/batcher/target/release/aligned-batcher /usr/local/bin/
COPY --from=builder /aligned_layer/batcher/target/release/aligned /usr/local/bin/
COPY --from=builder /aligned_layer/gnark_groth16_bn254_infinite_script /usr/local/bin
COPY ./contracts/script ./contracts/script
COPY ../scripts/test_files/ ./scripts/test_files
COPY ./config-files/config-batcher-docker.yaml ./config-files/
COPY ./config-files/anvil.batcher.ecdsa.key.json ./config-files/

RUN apt update -y
RUN apt install -y libssl-dev ca-certificates

CMD ["aligned-batcher", "--config", "./config-files/config-batcher-docker.yaml"]
