package operator

import (
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/halo2ipa"
)

func FuzzHalo2Ipa(f *testing.F) {
	f.Fuzz(func(t *testing.T, proof_data []byte, params_data []byte, public_input_data []byte, seed int64) {
		// MarshalUnmarshal

		_ = halo2ipa.VerifyHalo2IpaProof(
			([]byte)(proof_data),
			([]byte)(params_data),
			([]byte)(public_input_data),
		)
	})
}
