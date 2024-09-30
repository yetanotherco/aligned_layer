package operator

import (
	"testing"
)

// Test roundtrip of cbor serialization used in Aligned.
func FuzzMarshal(f *testing.F) {
	f.Fuzz(func(t *testing.T, data []byte, seed int64) {
		// MarshalUnmarshal

		var marshalled VerificationData
		decoder, err := createDecoderMode()
		if err != nil {
			return
		}
		err = decoder.Unmarshal(data, &marshalled)
		if err != nil {
			return
		}
	})
}
