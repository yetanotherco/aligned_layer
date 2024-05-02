FROM ubuntu:22.04 as base_system

# Update packages and install required dependencies
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y python3 python3-pip git curl iputils-ping apt-utils supervisor

# Create aligned-layer user
RUN useradd -u 5000 -m aligned-layer
RUN chsh -s /bin/bash aligned-layer

# Switch to aligned-layer user
USER aligned-layer

# Set the working directory
WORKDIR /app

# Copy the Anvil state file into the container
COPY alignedlayer-deployed-anvil-state.json /app/

# Run Foundry to install Anvil and then run Anvil command to load state
RUN curl -L https://foundry.paradigm.xyz | bash

RUN /home/aligned-layer/.foundry/bin/foundryup

CMD ["/home/aligned-layer/.foundry/bin/anvil", "--load-state", "/app/alignedlayer-deployed-anvil-state.json"]
