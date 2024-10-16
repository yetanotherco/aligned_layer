ROM ghcr.io/yetanotherco/aligned_layer/aligned_base:latest AS base

COPY go.mod .
COPY go.sum .
COPY batcher /aligned_layer/batcher/

RUN apt update -y && apt install -y gcc
RUN go build -buildmode=c-archive -o libverifier.a /aligned_layer/batcher/aligned-batcher/gnark/verifier.go

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

FROM chef AS planner

COPY --from=base /aligned_layer/batcher/aligned-batcher /aligned_layer/batcher/aligned-batcher
WORKDIR /aligned_layer/batcher/aligned-batcher/
RUN cargo chef prepare --recipe-path /aligned_layer/batcher/aligned-batcher/recipe.json

COPY --from=base /aligned_layer/batcher/aligned/Cargo.toml /aligned_layer/batcher/aligned/Cargo.toml
WORKDIR /aligned_base/batcher/aligned/
RUN cargo chef prepare --recipe-path /aligned_layer/batcher/aligned/recipe.json

FROM chef AS chef_builder

WORKDIR /aligned_layer/batcher/aligned-batcher
COPY --from=planner /aligned_layer/batcher/aligned-batcher/recipe.json /aligned_layer/batcher/aligned-batcher/recipe.json
RUN cargo chef cook --release --recipe-path /aligned_layer/batcher/aligned-batcher/recipe.json

WORKDIR /aligned_layer/batcher/aligned/
COPY --from=planner /aligned_layer/batcher/aligned/recipe.json /aligned_layer/batcher/aligned/recipe.json
RUN cargo chef cook --release --recipe-path /aligned_layer/batcher/aligned/recipe.json

FROM base AS builder

COPY --from=chef_builder /aligned_layer/batcher/** /aligned_layer/batcher/
RUN cargo build --manifest-path /aligned_layer/batcher/aligned-batcher/Cargo.toml --release
RUN cargo build --manifest-path /aligned_layer/batcher/aligned/Cargo.toml --release

FROM debian:bookworm-slim AS final

COPY --from=builder /aligned_layer /aligned_layer
COPY --from=builder /aligned_layer/batcher/target/release/aligned-batcher /usr/local/bin/
COPY --from=builder /aligned_layer/batcher/target/release/aligned /usr/local/bin/
COPY --from=builder /aligned_layer/gnark_groth16_bn254_infinite_script /usr/local/bin

RUN apt update -y && apt install -y libssl-dev ca-certificates

CMD ["aligned-batcher", "--config", "./config-files/config-batcher-docker.yaml"]
