package types

import "github.com/yetanotherco/aligned_layer/common"

type Task struct {
	ProvingSystem   common.ProvingSystemId
	Proof           []byte
	PublicInput     []byte
	VerificationKey []byte
}

func NewTask(provingSystemId common.ProvingSystemId, proof []byte, publicInput []byte, verificationKey []byte) *Task {
	return &Task{
		ProvingSystem:   provingSystemId,
		Proof:           proof,
		PublicInput:     publicInput,
		VerificationKey: verificationKey,
	}
}
