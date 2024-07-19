cat mina_mainnet_protocol_query.json | jq -r '.data.bestChain.[0].stateHashField' >protocol_state_hash.pub
cat mina_mainnet_protocol_query.json | jq -r '.data.protocolState' >protocol_state.pub
cat mina_mainnet_protocol_query.json | jq -r '.data.bestChain.[0].protocolStateProof.base64' >protocol_state_proof.proof
