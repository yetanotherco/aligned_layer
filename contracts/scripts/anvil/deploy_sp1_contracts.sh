# Deploy Groth16 SP1 verifier gateway
forge script script/deploy/SP1VerifierGatewayGroth16Deployer.s.sol:SP1VerifierGatewayScript \
    --rpc-url "http://localhost:8545" \
    --private-key "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a" \
    --broadcast

# Deploy Groth16 SP1 verifier
forge script ./script/deploy/SP1VerifierGroth16Deployer.s.sol:SP1VerifierScript \
    --rpc-url "http://localhost:8545" \
    --private-key "0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6" \
    --broadcast \
    --verify --verifier etherscan --multi 
