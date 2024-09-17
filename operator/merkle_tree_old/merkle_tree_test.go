package merkle_tree

import (
	"encoding/hex"
	"fmt"
	"io"
	"os"
	"testing"
)

func TestVerifyMerkleTreeBatch(t *testing.T) {
	batchFile, err := os.Open("lib/test_files/merkle_tree_batch.bin")
	if err != nil {
		t.Fatalf("Error opening batch file: %v", err)
	}

	batchByteValue, err := io.ReadAll(batchFile)
	if err != nil {
		t.Fatalf("Error reading batch file: %v", err)
	}

	rootFile, err := os.Open("lib/test_files/merkle_root.bin")
	if err != nil {
		t.Fatalf("Error opening batch file: %v", err)
	}

	rootByteValue, err := io.ReadAll(rootFile)
	if err != nil {
		t.Fatalf("Error reading batch file: %v", err)
	}

	merkle_root := make([]byte, hex.DecodedLen(len(rootByteValue)))
	_, err = hex.Decode(merkle_root, rootByteValue)
	if err != nil {
		fmt.Println("Error decoding hex string:", err)
		return
	}

	var merkleRoot [32]byte
	copy(merkleRoot[:], merkle_root)

	if !VerifyMerkleTreeBatch(batchByteValue, uint(len(batchByteValue)), merkleRoot) {
		t.Errorf("Batch did not verify Merkle Root")
	}

}
