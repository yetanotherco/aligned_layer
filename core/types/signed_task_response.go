package types

import (
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	"github.com/Layr-Labs/eigensdk-go/types"
)

type SignedTaskResponse struct {
	TaskResponse string // TODO: Get Interface from contract bindings
	BlsSignature bls.Signature
	OperatorId   types.OperatorId
}
