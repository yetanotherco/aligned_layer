deps:
	make -C contracts deps
	go install github.com/maoueh/zap-pretty@latest

anvil-deploy-eigen-contracts:
	make -C contracts anvil-deploy-eigen-contracts

anvil-start:
	make -C contracts anvil-start

