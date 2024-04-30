package types

import (
	"github.com/yetanotherco/aligned_layer/common"
	"math/big"
)

type Task struct {
	ProvingSystem             common.ProvingSystemId
	Proof                     []byte
	PublicInput               []byte
	VerificationKey           []byte
	QuorumThresholdPercentage uint8
	Fee             		  *big.Int
}

func NewTask(provingSystemId common.ProvingSystemId, proof []byte, publicInput []byte, verificationKey []byte, quorumThresholdPercentage uint8, fee *big.Int) *Task {
	return &Task{
		ProvingSystem:             provingSystemId,
		Proof:                     proof,
		PublicInput:               publicInput,
		VerificationKey:           verificationKey,
		QuorumThresholdPercentage: quorumThresholdPercentage,
		Fee:             fee,
	}
}
