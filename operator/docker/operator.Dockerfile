FROM golang:1.22.4

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    openssl \
    libssl-dev

# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/app

# Copy dependencies
COPY go.mod go.sum ./

# Copy the Makefile and the operator (for the FFI)
COPY Makefile /usr/src/app
COPY operator /usr/src/app/operator

# Copy the aligned-sdk
COPY batcher/aligned-sdk /usr/src/app/batcher/aligned-sdk

# Build the FFI
RUN make build_all_ffi_linux

COPY metrics /usr/src/app/metrics
COPY contracts/script/output /usr/src/app/contracts/script/output
COPY contracts/bindings /usr/src/app/contracts/bindings
COPY core /usr/src/app/core
COPY common /usr/src/app/common

# Define operator version argument
ARG OPERATOR_VERSION

# Build the operator
RUN go build -ldflags "-X main.Version=${OPERATOR_VERSION}" -v -o /usr/local/bin/operator /usr/src/app/operator/cmd/main.go

ENTRYPOINT [ "/usr/local/bin/operator", "start", "--config", "/usr/src/config/operator.yaml"]
