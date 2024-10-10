FROM ghcr.io/yetanotherco/aligned_layer/aligned_base:latest AS builder

WORKDIR /aligned_layer

COPY go.mod .
COPY go.sum .
COPY aggregator   ./aggregator
COPY core         ./core
COPY metrics      ./metrics
COPY contracts/bindings/ ./contracts/bindings

RUN go build -o ./aligned-layer-aggregator aggregator/cmd/main.go

FROM debian:bookworm-slim

WORKDIR /aggregator

COPY --from=builder /aligned_layer/aligned-layer-aggregator /usr/local/bin/aligned-layer-aggregator
COPY config-files/config-aggregator-docker.yaml ./config-files/config-aggregator-docker.yaml
COPY contracts/script/output/devnet/alignedlayer_deployment_output.json ./contracts/script/output/devnet/alignedlayer_deployment_output.json
COPY contracts/script/output/devnet/eigenlayer_deployment_output.json ./contracts/script/output/devnet/eigenlayer_deployment_output.json
COPY config-files/anvil.aggregator.ecdsa.key.json ./config-files/anvil.aggregator.ecdsa.key.json
COPY config-files/anvil.aggregator.bls.key.json ./config-files/anvil.aggregator.bls.key.json

CMD ["aligned-layer-aggregator", "--config", "config-files/config-aggregator-docker.yaml"]
