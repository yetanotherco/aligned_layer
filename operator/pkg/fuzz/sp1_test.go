package operator

import (
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/sp1"
)

func FuzzSp1(f *testing.F) {
	f.Fuzz(func(t *testing.T, proof_bytes []byte, elf_bytes []byte, seed int64) {
		// MarshalUnmarshal

		_ = sp1.VerifySp1Proof(proof_bytes, elf_bytes)

	})
}
