FROM debian:bookworm-slim

ENV FOUNDRY_VERSION=v1.1.0

RUN apt update -y
RUN apt upgrade -y
RUN apt install -y curl git

# Install foundry
RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/root/.foundry/bin:${PATH}"
RUN foundryup -i ${FOUNDRY_VERSION}
