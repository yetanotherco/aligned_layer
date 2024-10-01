package operator

import (
	"github.com/fxamacker/cbor/v2"
)

func createDecoderMode() (cbor.DecMode, error) {
	// Use the max value for length of an array to correctly decode the batch.
	return cbor.DecOptions{MaxArrayElements: 2147483647}.DecMode()
}
