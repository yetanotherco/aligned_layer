# Query wrap proof data and wrap proof verifier key
curl -d '{"query": "{
  blockchainVerificationKey
  bestChain {
	  protocolStateProof {
	    json
	  }
  }
}"}' -H 'Content-Type: application/json' $MINA_GRAPHQL_URL -o mina_state_proof_vk_query.json
