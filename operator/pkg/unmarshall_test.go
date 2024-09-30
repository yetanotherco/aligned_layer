package operator

import (
	"bytes"
	"testing"

	fuzz "github.com/AdaLogics/go-fuzz-headers"
	"github.com/fxamacker/cbor/v2"
)

// Test roundtrip of cbor serialization used in Aligned.
func FuzzUnMarshal(f *testing.F) {
	f.Fuzz(func(t *testing.T, data []byte, seed int64) {
		fz := fuzz.NewConsumer(data)
		var verification_data VerificationData
		err := fz.GenerateStruct(&verification_data)
		if err != nil {
			return
		}

		var unmarshalled []byte
		encoder := cbor.NewEncoder(bytes.NewBuffer(unmarshalled))
		err = encoder.Encode(&verification_data)
		if err != nil {
			return
		}
	})
}
