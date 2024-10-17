FROM debian:bookworm-slim

RUN apt update -y
RUN apt upgrade -y
RUN apt install -y curl git

# Install foundry
RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/root/.foundry/bin:${PATH}"
RUN foundryup
