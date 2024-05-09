package pkg

func SplitIntoChunks(proof []byte, chunkSize uint64) [][]byte {
	proofLen := len(proof)

	// Calculate the number of chunks
	numChunks := proofLen / int(chunkSize)
	if proofLen%int(chunkSize) != 0 {
		numChunks++
	}

	chunks := make([][]byte, numChunks)
	chunkIdx := 0
	for i := 0; i < proofLen; i += int(chunkSize) {
		end := i + int(chunkSize)
		if end > proofLen {
			end = proofLen
		}
		chunks[chunkIdx] = proof[i:end]
		chunkIdx++
	}

	return chunks
}
