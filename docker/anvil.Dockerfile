FROM ghcr.io/yetanotherco/aligned_layer/foundry:latest

COPY contracts/scripts/anvil/state/* .

CMD ["anvil", "--load-state", "alignedlayer-deployed-anvil-state.json", "--block-time", "7", "--host", "0.0.0.0"]
