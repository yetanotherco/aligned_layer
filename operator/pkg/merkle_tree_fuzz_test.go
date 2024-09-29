package operator

import (
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/merkle_tree"
)

func FuzzMerkleTree(f *testing.F) {
	f.Fuzz(func(t *testing.T, merkle_batch_data []byte, merkle_root_data []byte, seed int64) {
		// MarshalUnmarshal

		// Declare a [32]byte array
		var merkle_root [32]byte

		// Copy the contents of the slice into the array
		copy(merkle_root[:], merkle_root_data)
		_ = merkle_tree.VerifyMerkleTreeBatch(merkle_batch_data, merkle_root)

	})
}
