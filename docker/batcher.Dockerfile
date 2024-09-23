FROM aligned_base AS builder

RUN apt update -y
RUN apt install -y gcc

WORKDIR /aligned_layer/batcher/aligned-batcher

ENV GOOS=linux
ARG GOARCH
ENV CGO_ENABLED=1
RUN go build -buildmode=c-archive -o libverifier.a ./gnark/verifier.go

WORKDIR /aligned_layer
RUN cargo build --manifest-path ./batcher/aligned-batcher/Cargo.toml --release

#FROM debian:bookworm-slim

#WORKDIR /aligned_layer

#COPY --from=builder /aligned_layer/batcher/target/release/aligned-batcher /usr/local/bin/
COPY ./config-files/config-batcher-docker.yaml ./config-files/
#COPY contracts ./contracts

RUN apt update -y
RUN apt install -y libssl-dev ca-certificates

CMD ["/aligned_layer/batcher/target/release/aligned-batcher", "--config", "./config-files/config-batcher-docker.yaml"]
