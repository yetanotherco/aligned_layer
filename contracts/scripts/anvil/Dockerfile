FROM alpine:latest

RUN apk add --no-cache curl bash

RUN curl -L https://foundry.paradigm.xyz | bash

ENV PATH="/root/.foundry/bin:$PATH"

WORKDIR /app

COPY contracts/ /app/contracts/scripts/anvil/state


CMD ["make", "anvil-start"]