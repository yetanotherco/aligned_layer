package merkle_tree

import (
	"encoding/hex"
	"fmt"
	"io"
	"os"
	"testing"
)

func TestVerifyMerkleTreeBatch(t *testing.T) {
	batchFile, err := os.Open("./lib/test_files/5ba2f046e3c1072b96f55728a67d73b4e246a6c27960b0c52d7fafb77981bcb0.json")
	if err != nil {
		t.Fatalf("Error opening batch file: %v", err)
	}

	byteValue, err := io.ReadAll(batchFile)
	if err != nil {
		t.Fatalf("Error reading batch file: %v", err)
	}

	hexMerkleRootStr := "5ba2f046e3c1072b96f55728a67d73b4e246a6c27960b0c52d7fafb77981bcb0"

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
