package operator

import (
	"bytes"
	"testing"

	"github.com/ugorji/go/codec"
)

// Test roundtrip of cbor serialization used in Aligned.
func FuzzMarshalUnmarshal(f *testing.F) {
	f.Fuzz(func(t *testing.T, data []byte, seed int64) {
		// MarshalUnmarshal

		var marshalled VerificationData
		decoder := codec.NewDecoderBytes(data, new(codec.CborHandle))
		err := decoder.Decode(&marshalled)
		if err != nil {
			return
		}

		var unmarshalled []byte
		encoder := codec.NewEncoderBytes(&unmarshalled, new(codec.CborHandle))
		err = encoder.Encode(marshalled)
		if err != nil {
			return
		}

		if len(data) != 0 && !bytes.Equal(data, unmarshalled) {
			t.Fatalf("data and marshalled are not equal. data[%d]: [%v], marshalled[%d]: [%s]", len(data), data, len(unmarshalled), unmarshalled)
		}
	})
}
