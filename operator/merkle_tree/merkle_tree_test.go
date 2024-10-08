package merkle_tree

import (
	"encoding/hex"
	"fmt"
	"os"
	"testing"
)

const BatchFilePath = "lib/test_files/merkle_tree_batch.bin"

const RootFilePath = "lib/test_files/merkle_root.bin"

func TestVerifyMerkleTreeBatch(t *testing.T) {
	batchByteValue, err := os.ReadFile(BatchFilePath)
	if err != nil {
		t.Fatalf("Error reading batch file: %v", err)
	}

	rootByteValue, err := os.ReadFile(RootFilePath)
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

	verified, err := VerifyMerkleTreeBatch(batchByteValue, merkleRoot)
	if err != nil || !verified {
		t.Errorf("Batch did not verify Merkle Root")
	}

}
