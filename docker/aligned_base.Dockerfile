FROM debian:bookworm-slim AS base

ARG BUILDARCH
ENV GO_VERSION=1.22.2

RUN apt update -y && apt upgrade -y
RUN apt install -y wget \
                   tar \
                   curl \
                   git \
                   make \
                   clang \
                   pkg-config \
                   openssl \
                   libssl-dev \
                   yq \
                   jq

RUN wget https://golang.org/dl/go$GO_VERSION.linux-${BUILDARCH}.tar.gz
RUN tar -C /usr/local -xzf go$GO_VERSION.linux-${BUILDARCH}.tar.gz
RUN rm go$GO_VERSION.linux-${BUILDARCH}.tar.gz
RUN apt clean -y
RUN rm -rf /var/lib/apt/lists/*
ENV PATH="/usr/local/go/bin:${PATH}"

# Install go deps
RUN go install github.com/maoueh/zap-pretty@v0.3.0
RUN go install github.com/ethereum/go-ethereum/cmd/abigen@v1.14.0
RUN go install github.com/Layr-Labs/eigenlayer-cli/cmd/eigenlayer@v0.13.0

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /aligned_layer

COPY Makefile .
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

FROM chef AS planner

# build_sp1_linux
COPY operator/sp1/lib/Cargo.toml /aligned_layer/operator/sp1/lib/Cargo.toml
COPY operator/sp1/lib/src/ /aligned_layer/operator/sp1/lib/src/
WORKDIR /aligned_layer/operator/sp1/lib
RUN cargo chef prepare --recipe-path /aligned_layer/operator/sp1/lib/recipe.json

# build_risc_zero_linux
COPY operator/risc_zero/lib/Cargo.toml /aligned_layer/operator/risc_zero/lib/Cargo.toml
COPY operator/risc_zero/lib/src/ /aligned_layer/operator/risc_zero/lib/src/
WORKDIR /aligned_layer/operator/risc_zero/lib
RUN cargo chef prepare --recipe-path /aligned_layer/operator/risc_zero/lib/recipe.json

# build_merkle_tree_linux
COPY operator/merkle_tree/lib/Cargo.toml /aligned_layer/operator/merkle_tree/lib/Cargo.toml
COPY operator/merkle_tree/lib/src/ /aligned_layer/operator/merkle_tree/lib/src/
WORKDIR operator/merkle_tree/lib
RUN cargo chef prepare --recipe-path /aligned_layer/operator/merkle_tree/lib/recipe.json

FROM chef AS chef_builder

COPY crates/sdk /aligned_layer/crates/sdk/

# build_sp1_linux
COPY operator/sp1/ /aligned_layer/operator/sp1/
COPY --from=planner /aligned_layer/operator/sp1/lib/recipe.json /aligned_layer/operator/sp1/lib/recipe.json
WORKDIR /aligned_layer/operator/sp1/lib/
RUN cargo chef cook --release --recipe-path /aligned_layer/operator/sp1/lib/recipe.json

# build_risc_zero_linux
COPY operator/risc_zero/ /aligned_layer/operator/risc_zero/
COPY --from=planner /aligned_layer/operator/risc_zero/lib/recipe.json /aligned_layer/operator/risc_zero/lib/recipe.json
WORKDIR /aligned_layer/operator/risc_zero/lib/
RUN cargo chef cook --release --recipe-path /aligned_layer/operator/risc_zero/lib/recipe.json

# build_merkle_tree_linux
COPY operator/merkle_tree/ /aligned_layer/operator/merkle_tree/
COPY --from=planner /aligned_layer/operator/merkle_tree/lib/recipe.json /aligned_layer/operator/merkle_tree/lib/recipe.json
WORKDIR /aligned_layer/operator/merkle_tree/lib/
RUN cargo chef cook --release --recipe-path /aligned_layer/operator/merkle_tree/lib/recipe.json

FROM base AS builder

ENV RELEASE_FLAG=--release
ENV TARGET_REL_PATH=release
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

COPY operator/ /aligned_layer/operator/
COPY crates/ /aligned_layer/crates/
COPY --from=chef_builder /aligned_layer/crates/sdk /aligned_layer/crates/sdk

# build_sp1_linux
COPY --from=chef_builder /aligned_layer/operator/sp1/lib/target/ /aligned_layer/operator/sp1/lib/target/
WORKDIR /aligned_layer/operator/sp1/lib
RUN cargo build ${RELEASE_FLAG}
RUN cp /aligned_layer/operator/sp1/lib/target/${TARGET_REL_PATH}/libsp1_verifier_ffi.so /aligned_layer/operator/sp1/lib/libsp1_verifier_ffi.so

# build_risc_zero_linux
COPY --from=chef_builder /aligned_layer/operator/risc_zero/lib/target/ /aligned_layer/operator/risc_zero/lib/target/
WORKDIR /aligned_layer/operator/risc_zero/lib
RUN cargo build ${RELEASE_FLAG}
RUN cp /aligned_layer/operator/risc_zero/lib/target/${TARGET_REL_PATH}/librisc_zero_verifier_ffi.so /aligned_layer/operator/risc_zero/lib/librisc_zero_verifier_ffi.so

# build_merkle_tree_linux
COPY --from=chef_builder /aligned_layer/operator/merkle_tree/lib/target/ /aligned_layer/operator/merkle_tree/lib/target/
WORKDIR /aligned_layer/operator/merkle_tree/lib
RUN cargo build ${RELEASE_FLAG}
RUN cp /aligned_layer/operator/merkle_tree/lib/target/${TARGET_REL_PATH}/libmerkle_tree.so /aligned_layer/operator/merkle_tree/lib/libmerkle_tree.so
RUN cp /aligned_layer/operator/merkle_tree/lib/target/${TARGET_REL_PATH}/libmerkle_tree.a /aligned_layer/operator/merkle_tree/lib/libmerkle_tree.a
