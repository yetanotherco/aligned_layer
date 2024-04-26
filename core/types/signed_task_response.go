package types

import (
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	"github.com/Layr-Labs/eigensdk-go/types"
)

type SignedTaskResponse struct {
	TaskIndex uint64
	// TODO: Might be better to include hash
	BlsSignature bls.Signature
	OperatorId   types.OperatorId
}
