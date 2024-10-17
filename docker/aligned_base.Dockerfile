FROM rust:slim-bookworm

# Install rust nightly-2024-04-17
RUN rustup toolchain install nightly-2024-04-17

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
RUN go install github.com/maoueh/zap-pretty@latest
RUN go install github.com/ethereum/go-ethereum/cmd/abigen@latest
RUN go install github.com/Layr-Labs/eigenlayer-cli/cmd/eigenlayer@latest

WORKDIR /aligned_layer

COPY Makefile .
COPY operator ./operator
COPY batcher/aligned-sdk ./batcher/aligned-sdk

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN make build_all_ffi_linux
