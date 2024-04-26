FROM golang:1.22.2

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download
COPY . .

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

COPY ../Makefile .
COPY contracts/ ../contracts/
COPY operator/ ../operator/
COPY config-files/ ../config-files/


CMD ["make", "aggregator-start"]
