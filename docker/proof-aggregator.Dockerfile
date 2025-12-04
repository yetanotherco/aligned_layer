FROM ghcr.io/yetanotherco/aligned_layer/aligned_base:latest AS base

# Install SP1 toolchain
RUN curl -L https://sp1up.succinct.xyz | bash -s -- -y
ENV PATH="/root/.sp1/bin:${PATH}"
RUN sp1up

# Install Risc0 toolchain
RUN curl -L https://risczero.com/install | bash
ENV PATH="/root/.risc0/bin:${PATH}"
RUN rzup install

COPY crates /aligned_layer/crates/
COPY aggregation_mode /aligned_layer/aggregation_mode/
WORKDIR /aligned_layer

RUN IN_DOCKER=true cargo build --manifest-path ./aggregation_mode/Cargo.toml --features prove --release --bin proof_aggregator_cpu
RUN cd aggregation_mode && IN_DOCKER=true ./scripts/build_programs.sh

FROM debian:bookworm-slim AS final

RUN apt update -y && apt install -y libssl-dev ca-certificates

# Install required tools and set up Docker repository
# Installing docker is necessary for SP1 and Risc0 wrapping to snark
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    gnupg \
    lsb-release && \
    install -m 0755 -d /etc/apt/keyrings && \
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc && \
    chmod a+r /etc/apt/keyrings/docker.asc && \
    echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/debian \
    $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
    tee /etc/apt/sources.list.d/docker.list > /dev/null && \
    apt-get update

# Note, we don't need to install docker-ce and containerd.io as we pass the docker engine socket via docker volume
RUN apt-get install docker-ce-cli docker-buildx-plugin docker-compose-plugin -y

COPY --from=base /aligned_layer/aggregation_mode/target/release/proof_aggregator_cpu /aligned_layer/proof_aggregator_cpu
COPY config-files/config-proof-aggregator-docker.yaml /aligned_layer/config-files/
COPY config-files/proof-aggregator.last_aggregated_block.json /aligned_layer/config-files/
COPY config-files/anvil.proof-aggregator.ecdsa.key.json /aligned_layer/config-files/

# Leave it in the background as this container is used to exec the proof_aggregator binary
CMD ["sleep","infinity"]

