# Use the latest foundry image
FROM debian:bookworm-slim

RUN  apt-get update;  apt-get -y install curl ; apt-get install make
# Copy our source code into the container
COPY Makefile Makefile


ENTRYPOINT ["tail", "-f", "/dev/null"]

#RUN curl -L https://foundry.paradigm.xyz | bash
#RUN source /home/ubuntu/.bashrc
#ENTRYPOINT ["make", "anvil_start_with_block_time"]
