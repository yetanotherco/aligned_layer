FROM golang:1.22.2

WORKDIR /app

# Copy Go dependencies files
COPY go.mod go.sum ./

# Download Go dependencies
RUN go mod download

# Copy the rest of the application files
COPY . .

# Install system dependencies
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y python3 python3-pip git nano curl iputils-ping redis-tools autossh nmap apt-utils supervisor

# Create aligned-layer user
RUN useradd -u 5000 -m aligned-layer
RUN chsh -s /bin/bash aligned-layer
USER aligned-layer

# Install foundry
RUN curl -L https://foundry.paradigm.xyz | bash
RUN /home/aligned-layer/.foundry/bin/foundryup

# Copy Makefile, contracts, operator, and config files
COPY ../Makefile .
COPY contracts/ ../contracts/
COPY operator/ ../operator/
COPY config-files/ ../config-files/

# Run the Go application
CMD ["go", "run", "aggregator/cmd/main.go", "--config", "config-files/config.yaml"]


#CMD ["make", "aggregator-start"]
