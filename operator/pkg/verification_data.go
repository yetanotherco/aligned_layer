package operator

import (
	"github.com/lambdaclass/aligned_layer/common"
)

type VerificationData struct {
	ProvingSystemId common.ProvingSystemId `json:"proving_system"`
	Proof           []byte                 `json:"proof"`
	PubInput        []byte                 `json:"pub_input"`
	VerificationKey []byte                 `json:"verification_key"`
	VmProgramCode   []byte                 `json:"vm_program_code"`
}
