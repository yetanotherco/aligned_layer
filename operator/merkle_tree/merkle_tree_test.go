package merkle_tree

import (
	"encoding/hex"
	"fmt"
	"io"
	"os"
	"testing"
)

func TestVerifyMerkleTreeBatch(t *testing.T) {
	batchFile, err := os.Open("lib/test_files/a3cf9e0284d77d342087b1ed4ab2de0267417577452a3187c9b9592e4cc89188.json")
	if err != nil {
		t.Fatalf("Error opening batch file: %v", err)
	}

	byteValue, err := io.ReadAll(batchFile)
	if err != nil {
		t.Fatalf("Error reading batch file: %v", err)
	}

	hexMerkleRootStr := "a3cf9e0284d77d342087b1ed4ab2de0267417577452a3187c9b9592e4cc89188"

	byteSliceFromMerkleRoot, err := hex.DecodeString(hexMerkleRootStr)
	if err != nil {
		fmt.Println("Error decoding hex string:", err)
		return
	}

	var merkleRoot [32]byte
	copy(merkleRoot[:], byteSliceFromMerkleRoot)

	if !VerifyMerkleTreeBatch(byteValue, uint(len(byteValue)), merkleRoot) {
		t.Errorf("Batch did not verify Merkle Root")
	}

}
