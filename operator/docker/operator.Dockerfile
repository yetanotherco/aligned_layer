FROM golang:1.22.4

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl \
    openssl \
    libssl-dev

# Update new packages
RUN apt-get update

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# Add cargo to path
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/app

# Copy the Makefile and the operator (for the FFI)
COPY Makefile /usr/src/app
COPY operator /usr/src/app/operator

# Build the FFI
RUN make build_all_ffi_linux

# Copy dependencies
COPY go.mod go.sum ./
COPY metrics /usr/src/app/metrics
COPY contracts/script/output /usr/src/app/contracts/script/output
COPY contracts/bindings /usr/src/app/contracts/bindings
COPY core /usr/src/app/core
COPY common /usr/src/app/common

# Download dependencies
RUN go mod download && go mod tidy && go mod verify

# Build the operator
RUN go build -v -o /usr/local/bin/operator /usr/src/app/operator/cmd/main.go

ENTRYPOINT [ "/usr/local/bin/operator", "start", "--config", "/usr/src/config/operator.yaml"]
