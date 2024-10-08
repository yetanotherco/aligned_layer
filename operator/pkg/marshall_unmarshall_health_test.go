package operator

import (
	"bytes"
	"testing"

	"github.com/fxamacker/cbor/v2"
)

// Test roundtrip of cbor serialization and deserialization used in Aligned to verify that it doesn't panic.
func FuzzMarshalUnmarshal(f *testing.F) {
	f.Fuzz(func(t *testing.T, data []byte, seed int64) {
		// MarshalUnmarshal

		var unmarshalled VerificationData
		decoder, err := createDecoderMode()
		if err != nil {
			return
		}
		err = decoder.Unmarshal(data, &unmarshalled)
		if err != nil {
			return
		}

		var marshalled []byte
		encoder := cbor.NewEncoder(bytes.NewBuffer(marshalled))
		err = encoder.Encode(&unmarshalled)
		if err != nil {
			return
		}
	})
}
