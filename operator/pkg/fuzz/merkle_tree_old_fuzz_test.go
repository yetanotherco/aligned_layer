package operator

import (
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/merkle_tree_old"
)

func FuzzMerkleTreeOld(f *testing.F) {
	f.Fuzz(func(t *testing.T, merkle_batch_data []byte, merkle_root_data []byte, seed int64) {
		// MarshalUnmarshal

		// Declare a [32]byte array
		var merkle_root [32]byte

		// Copy the contents of the slice into the array
		copy(merkle_root[:], merkle_root_data)
		_ = merkle_tree_old.VerifyMerkleTreeBatchOld(merkle_batch_data, uint(len(merkle_root_data)), merkle_root)

	})
}
