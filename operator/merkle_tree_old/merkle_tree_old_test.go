package merkle_tree_old

import (
	"encoding/hex"
	"fmt"
	"os"
	"testing"
)

const BatchFilePath = "lib/test_files/merkle_tree_batch.bin"

const RootFilePath = "lib/test_files/merkle_root.bin"

func TestVerifyMerkleTreeBatchOld(t *testing.T) {
	batchByteValue, err := os.ReadFile(BatchFilePath)
	if err != nil {
		t.Fatalf("Error opening batch file: %v", err)
	}

	rootByteValue, err := os.ReadFile(RootFilePath)
	if err != nil {
		t.Fatalf("Error opening batch file: %v", err)
	}

	merkle_root := make([]byte, hex.DecodedLen(len(rootByteValue)))
	_, err = hex.Decode(merkle_root, rootByteValue)
	if err != nil {
		fmt.Println("Error decoding hex string:", err)
		return
	}

	var merkleRoot [32]byte
	copy(merkleRoot[:], merkle_root)

	if !VerifyMerkleTreeBatchOld(batchByteValue, merkleRoot) {
		t.Errorf("Batch did not verify Merkle Root")
	}

}