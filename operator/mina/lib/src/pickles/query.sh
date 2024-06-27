# Query wrap proof data
curl -d '{"query": "{
  bestChain {
	  protocolStateProof {
	    json
	  }
  }
}"}' -H 'Content-Type: application/json' $MINA_GRAPHQL_URL -o protocolStateProof.json

# Query wrap proof verifier key
curl -d '{"query": "{
  blockchainVerificationKey
}"}' -H 'Content-Type: application/json' $MINA_GRAPHQL_URL -o blockchainVerificationKey.json
