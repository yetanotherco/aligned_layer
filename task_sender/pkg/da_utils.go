package pkg

func SplitIntoChunks(proof []byte, chunkSize uint) [][]byte {
	chunks := make([][]byte, 0)
	for i := 0; i < len(proof); i += int(chunkSize) {
		end := i + int(chunkSize)
		if end > len(proof) {
			end = len(proof)
		}
		chunks = append(chunks, proof[i:end])
	}
	return chunks
}
