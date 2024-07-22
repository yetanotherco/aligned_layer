cat $1 | jq --raw-output0 '.data.bestChain.[0].stateHashField' >protocol_state_hash.pub
cat $1 | jq --raw-output0 '.data.protocolState' >protocol_state.pub
cat $1 | jq --raw-output0 '.data.bestChain.[0].protocolStateProof.base64' >protocol_state_proof.proof
cat $1 | jq '.data.blockchainVerificationKey' >devnet_vk.json
