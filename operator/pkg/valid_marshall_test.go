package operator

import (
	"bytes"
	"testing"

	fuzz "github.com/AdaLogics/go-fuzz-headers"
	"github.com/fxamacker/cbor/v2"
)

// Test roundtrip of cbor serialization used in Aligned to check correctness.
func FuzzValidMarshall(f *testing.F) {
	f.Fuzz(func(t *testing.T, data []byte, seed int64) {
		fz := fuzz.NewConsumer(data)
		var verification_data VerificationData
		err := fz.GenerateStruct(&verification_data)
		if err != nil {
			return
		}

		var marshalled []byte
		encoder := cbor.NewEncoder(bytes.NewBuffer(marshalled))
		err = encoder.Encode(&verification_data)
		if err != nil {
			return
		}

		if len(data) != 0 && !bytes.Equal(data, marshalled) {
			t.Fatalf("data and marshalled are not equal. data[%d]: [%v], marshalled[%d]: [%s]", len(data), data, len(marshalled), marshalled)
		}
	})
}
