cat mina_devnet_protocol_query.json | jq -r '.data.bestChain.[0].protocolState.previousStateHash' >protocol_state_hash.pub
cat mina_devnet_protocol_query.json | jq -r '.data.block.protocolStateProof.base64' >protocol_state_proof.proof
