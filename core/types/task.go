package types

import (
	"github.com/yetanotherco/aligned_layer/common"
	"math/big"
)

type Task struct {
	ProvingSystem   common.ProvingSystemId
	Proof           []byte
	PublicInput     []byte
	VerificationKey []byte
	Fee             *big.Int
}

func NewTask(provingSystemId common.ProvingSystemId, proof []byte, publicInput []byte, verificationKey []byte, fee *big.Int) *Task {
	return &Task{
		ProvingSystem:   provingSystemId,
		Proof:           proof,
		PublicInput:     publicInput,
		VerificationKey: verificationKey,
		Fee:             fee,
	}
}
