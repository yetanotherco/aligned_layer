package types

import "github.com/yetanotherco/aligned_layer/common"

type Task struct {
	ProvingSystem             common.ProvingSystemId
	Proof                     []byte
	PublicInput               []byte
	VerificationKey           []byte
	QuorumThresholdPercentage uint8
}

func NewTask(provingSystemId common.ProvingSystemId, proof []byte, publicInput []byte, verificationKey []byte, quorumThresholdPercentage uint8) *Task {
	return &Task{
		ProvingSystem:             provingSystemId,
		Proof:                     proof,
		PublicInput:               publicInput,
		VerificationKey:           verificationKey,
		QuorumThresholdPercentage: quorumThresholdPercentage,
	}
}
