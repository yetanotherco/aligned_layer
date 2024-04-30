# Use the official Go image as a builder stage.
FROM golang:1.22.2 as builder
WORKDIR /app
COPY . .
# Install eigenlayer globally
RUN go install github.com/Layr-Labs/eigenlayer-cli/cmd/eigenlayer@latest

# Use a base image that contains basic utilities.
FROM ubuntu:latest

# Install system dependencies
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y python3 python3-pip git nano curl iputils-ping autossh nmap apt-utils supervisor jq

# Create aligned-layer user and set Foundry installation accessible
RUN useradd -u 5000 -m aligned-layer && \
    chsh -s /bin/bash aligned-layer && \
    curl -L https://foundry.paradigm.xyz | bash && \
    mv /root/.foundry /home/aligned-layer/ && \
    chown -R aligned-layer:aligned-layer /home/aligned-layer/.foundry

USER aligned-layer
WORKDIR /home/aligned-layer

# Run foundry update as user
RUN /home/aligned-layer/.foundry/bin/foundryup

# Ensure PATH includes Foundry bin
ENV PATH="/home/aligned-layer/.foundry/bin:${PATH}"

# Debug: Check Foundry installation
RUN ls -l /home/aligned-layer/.foundry/bin
RUN cast --version

# Copy the binary from the builder stage
COPY --from=builder /go/bin/eigenlayer /usr/local/bin/
# Copy scripts and configuration files
COPY scripts/fund_operator_devnet.sh /usr/local/bin/fund_operator_devnet.sh
COPY config-files/ /config-files/

ENV NETWORK="devnet"

# Start the main application
CMD ["/bin/bash", "-c", "echo 'Sending funds to operator address on devnet' && . /usr/local/bin/fund_operator_devnet.sh && eigenlayer"]
