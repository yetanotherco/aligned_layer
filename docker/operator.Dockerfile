FROM ghcr.io/lambdaclass/aligned_layer/aligned_base:v0.8.0 AS builder

RUN apt update -y
RUN apt install -y gcc

ENV GOOS=linux
ARG GOARCH
ENV CGO_ENABLED=1
RUN go build -o /aligned_layer/aligned-layer-operator operator/cmd/main.go

FROM debian:bookworm-slim

WORKDIR /aligned_layer

RUN apt update -y
RUN apt install -y libssl-dev

COPY --from=builder /aligned_layer/aligned-layer-operator /usr/local/bin/aligned-layer-operator
COPY --from=builder /aligned_layer/operator/ ./operator/
COPY config-files/ ./config-files/
COPY contracts ./contracts

ENV LD_LIBRARY_PATH=/aligned_layer/operator/risc_zero/lib/

CMD ["aligned-layer-operator", "start", "--config", "./config-files/config-docker.yaml"]
