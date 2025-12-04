FROM ghcr.io/yetanotherco/aligned_layer/aligned_base:latest AS builder

WORKDIR /aligned_layer

RUN apt update -y
RUN apt install -y gcc

COPY go.mod .
COPY go.sum .

COPY core       ./core
COPY metrics    ./metrics
COPY common     ./common
COPY contracts/bindings/ ./contracts/bindings

RUN go build -o /aligned_layer/aligned-layer-operator operator/cmd/main.go

FROM debian:bookworm-slim

WORKDIR /aligned_layer

RUN apt update -y
RUN apt install -y libssl-dev

COPY --from=builder /aligned_layer/aligned-layer-operator /usr/local/bin/aligned-layer-operator
COPY --from=builder /aligned_layer/operator/ ./operator/
COPY config-files/ ./config-files/
COPY contracts ./contracts

ENV LD_LIBRARY_PATH=/aligned_layer/operator/risc_zero/lib/:/aligned_layer/operator/mina/lib:/aligned_layer/operator/mina_account/lib

CMD ["aligned-layer-operator", "start", "--config", "./config-files/config-operator-docker.yaml"]
