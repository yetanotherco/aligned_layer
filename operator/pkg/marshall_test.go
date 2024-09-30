package operator

import (
	"testing"

	"github.com/ugorji/go/codec"
)

// Test roundtrip of cbor serialization used in Aligned.
func FuzzMarshal(f *testing.F) {
	f.Fuzz(func(t *testing.T, data []byte, seed int64) {
		// MarshalUnmarshal

		var marshalled VerificationData
		decoder := codec.NewDecoderBytes(data, new(codec.CborHandle))
		err := decoder.Decode(&marshalled)
		if err != nil {
			return
		}
	})
}
