package operator

import (
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/risc_zero"
)

func FuzzRisc0(f *testing.F) {
	f.Fuzz(func(t *testing.T, receipt_bytes []byte, image_id_bytes []byte, public_input_bytes []byte, seed int64) {
		// MarshalUnmarshal

		_ = risc_zero.VerifyRiscZeroReceipt(receipt_bytes, image_id_bytes, public_input_bytes)

	})
}
