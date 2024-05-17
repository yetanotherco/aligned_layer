package operator

import (
	"github.com/yetanotherco/aligned_layer/common"
)

type VerificationData struct {
	ProvingSystemId common.ProvingSystemId
	Proof           []byte
	// FIXME(marian): These two fields should probably not be here.
	// Just setting them for a PoC
	PubInput        []byte
	VerificationKey []byte
}
