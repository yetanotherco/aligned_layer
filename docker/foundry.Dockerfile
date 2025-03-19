FROM debian:bookworm-slim

ENV FOUNDRY_VERSION=nightly-a428ba6ad8856611339a6319290aade3347d25d9

RUN apt update -y
RUN apt upgrade -y
RUN apt install -y curl git

# Install foundry
RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/root/.foundry/bin:${PATH}"
RUN foundryup -i ${FOUNDRY_VERSION}
