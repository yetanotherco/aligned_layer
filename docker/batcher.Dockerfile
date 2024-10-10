FROM ghcr.io/lambdaclass/aligned_layer/aligned_base:testnet AS builder

RUN apt update -y
RUN apt install -y gcc

WORKDIR /aligned_layer/batcher/aligned-batcher

ENV GOOS=linux
ARG GOARCH
ENV CGO_ENABLED=1
RUN go build -buildmode=c-archive -o libverifier.a ./gnark/verifier.go

WORKDIR /aligned_layer

COPY batcher/aligned-batcher/Cargo.toml batcher/aligned-batcher/Cargo.toml
COPY batcher/aligned/Cargo.toml batcher/aligned/Cargo.toml

RUN cargo install --path ./batcher/aligned-batcher/
RUN cargo install --path ./batcher/aligned/

COPY batcher/aligned-batcher/ batcher/aligned-batcher/
COPY batcher/aligned/ batcher/aligned/

RUN cargo build --manifest-path ./batcher/aligned-batcher/Cargo.toml --release
RUN cargo build --manifest-path ./batcher/aligned/Cargo.toml --release
RUN go build -o ./gnark_groth16_bn254_infinite_script scripts/test_files/gnark_groth16_bn254_infinite_script/cmd/main.go

RUN rm -rf operator/

FROM debian:bookworm-slim

WORKDIR /aligned_layer

COPY --from=builder /aligned_layer /aligned_layer
COPY --from=builder /aligned_layer/batcher/target/release/aligned-batcher /usr/local/bin/
COPY --from=builder /aligned_layer/batcher/target/release/aligned /usr/local/bin/
COPY --from=builder /aligned_layer/gnark_groth16_bn254_infinite_script /usr/local/bin

RUN apt update -y
RUN apt install -y libssl-dev ca-certificates

CMD ["aligned-batcher", "--config", "./config-files/config-batcher-docker.yaml"]
