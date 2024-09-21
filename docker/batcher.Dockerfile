FROM aligned_base AS builder

WORKDIR /aligned_layer

RUN cargo build --manifest-path ./batcher/aligned-batcher/Cargo.toml --release

FROM debian:bookworm-slim

WORKDIR /aligned_layer

COPY batcher/aligned-batcher/.env.docker ./batcher/aligned-batcher/.env.docker
COPY --from=builder /aligned_layer/batcher/target/release/aligned-batcher /usr/local/bin/
COPY ./config-files/config-batcher-docker.yaml ./config-files/
COPY ./batcher/aligned-batcher/.env.docker ./

RUN apt update -y
RUN apt install -y libssl-dev ca-certificates

CMD ["sh", "-c", "sleep 80 && aligned-batcher --config ./config-files/config-batcher-docker.yaml"]
