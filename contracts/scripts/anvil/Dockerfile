# WIP: This Dockerfile is a work in progress and is not yet functional

FROM alpine:latest

WORKDIR /app

COPY . /app/

RUN ls -la /app/

RUN chmod +x /app/anvil/deploy_aligned_contracts.sh /app/anvil/deploy_eigen_contracts.sh

RUN ls -la /app/anvil/

RUN sh /app/anvil/deploy_aligned_contracts.sh

CMD ["sh", "-c", "sudo /app/anvil/deploy_aligned_contracts.sh && /app/anvil/deploy_eigen_contracts.sh"]
