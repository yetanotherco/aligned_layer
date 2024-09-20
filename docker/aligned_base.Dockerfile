FROM debian:bookworm-slim

RUN apt update -y && apt upgrade -y

# Install golang 1.22.2
ENV GO_VERSION=1.22.2
RUN apt install -y wget \
                   tar \
                   curl \
                   git \
                   build-essential \
                   pkg-config \
                   openssl \
                   libssl-dev \
                   yq \
                   jq
RUN wget https://golang.org/dl/go$GO_VERSION.linux-amd64.tar.gz
RUN tar -C /usr/local -xzf go$GO_VERSION.linux-amd64.tar.gz
RUN rm go$GO_VERSION.linux-amd64.tar.gz
RUN apt clean -y
RUN rm -rf /var/lib/apt/lists/*
ENV PATH="/usr/local/go/bin:${PATH}"

# Install go deps
RUN go install github.com/maoueh/zap-pretty@latest
RUN go install github.com/ethereum/go-ethereum/cmd/abigen@latest
RUN go install github.com/Layr-Labs/eigenlayer-cli/cmd/eigenlayer@latest

# Install foundry
RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/root/.foundry/bin:${PATH}"
RUN foundryup

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /aligned_layer

COPY . .

RUN git submodule update --init --recursive

RUN make build_all_ffi_linux
