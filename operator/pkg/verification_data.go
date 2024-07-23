package operator

import (
	"github.com/yetanotherco/aligned_layer/common"
)

type VerificationData struct {
	ProvingSystemId common.ProvingSystemId `json:"proving_system"`
	Proof           []byte                 `json:"proof"`
	PubInput        []byte                 `json:"pub_input"`
	VerificationKey []byte                 `json:"verification_key"`
	VmProgramCode   []byte                 `json:"vm_program_code"`
}

type NoncedVerificationData struct {
	VerificationData VerificationData `json:"verification_data"`
	Nonce            []byte           `json:"nonce"`
}
