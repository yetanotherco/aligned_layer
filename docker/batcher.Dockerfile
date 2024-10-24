FROM ghcr.io/yetanotherco/aligned_layer/aligned_base:latest AS base

COPY go.mod .
COPY go.sum .
COPY batcher/aligned-batcher/gnark/verifier.go /aligned_layer/batcher/aligned-batcher/gnark/verifier.go

RUN apt update -y && apt install -y gcc
RUN go build -buildmode=c-archive -o libverifier.a /aligned_layer/batcher/aligned-batcher/gnark/verifier.go

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

FROM chef AS planner

COPY batcher/aligned-batcher/Cargo.toml /aligned_layer/batcher/aligned-batcher/Cargo.toml
COPY batcher/aligned-batcher/src/main.rs /aligned_layer/batcher/aligned-batcher/src/main.rs
WORKDIR /aligned_layer/batcher/aligned-batcher/
RUN cargo chef prepare --recipe-path /aligned_layer/batcher/aligned-batcher/recipe.json

COPY batcher/aligned/Cargo.toml /aligned_layer/batcher/aligned/Cargo.toml
COPY batcher/aligned/src/main.rs /aligned_layer/batcher/aligned/src/main.rs
WORKDIR /aligned_layer/batcher/aligned/
RUN cargo chef prepare --recipe-path /aligned_layer/batcher/aligned/recipe.json

FROM chef AS chef_builder
COPY batcher/aligned-sdk/ /aligned_layer/batcher/aligned-sdk/

COPY --from=planner /aligned_layer/batcher/aligned-batcher/recipe.json /aligned_layer/batcher/aligned-batcher/recipe.json
WORKDIR /aligned_layer/batcher/aligned-batcher
RUN cargo chef cook --release --recipe-path /aligned_layer/batcher/aligned-batcher/recipe.json

COPY --from=planner /aligned_layer/batcher/aligned/recipe.json /aligned_layer/batcher/aligned/recipe.json
WORKDIR /aligned_layer/batcher/aligned/
RUN cargo chef cook --release --recipe-path /aligned_layer/batcher/aligned/recipe.json

FROM base AS builder
COPY . /aligned_layer/

COPY --from=chef_builder /aligned_layer/batcher/aligned-batcher/target/ /aligned_layer/batcher/aligned-batcher/target/
WORKDIR /aligned_layer/batcher/aligned-batcher/
RUN cargo build --manifest-path /aligned_layer/batcher/aligned-batcher/Cargo.toml --release

COPY --from=chef_builder /aligned_layer/batcher/aligned/target/ /aligned_layer/batcher/aligned/target/
WORKDIR /aligned_layer/batcher/aligned/
RUN cargo build --manifest-path /aligned_layer/batcher/aligned/Cargo.toml --release

COPY scripts/test_files/gnark_groth16_bn254_infinite_script/ /aligned_layer/scripts/test_files/gnark_groth16_bn254_infinite_script/
WORKDIR /aligned_layer
RUN go build -o /aligned_layer/gnark_groth16_bn254_infinite_script scripts/test_files/gnark_groth16_bn254_infinite_script/cmd/main.go

RUN rm -rf operator/

FROM debian:bookworm-slim AS final

COPY --from=builder /aligned_layer /aligned_layer
COPY --from=builder /aligned_layer/batcher/target/release/aligned-batcher /usr/local/bin/
COPY --from=builder /aligned_layer/batcher/target/release/aligned /usr/local/bin/
COPY --from=builder /aligned_layer/gnark_groth16_bn254_infinite_script /usr/local/bin
COPY ./contracts/script ./contracts/script
COPY ../scripts/test_files/ ./scripts/test_files
COPY ./config-files/config-batcher-docker.yaml ./config-files/
COPY ./config-files/anvil.batcher.ecdsa.key.json ./config-files/

RUN apt update -y && apt install -y libssl-dev ca-certificates

CMD ["aligned-batcher", "--config", "./config-files/config-batcher-docker.yaml"]
