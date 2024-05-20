package types

import (
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
)

type SignedTaskResponse struct {
	BatchMerkleRoot  [32]byte
	TaskCreatedBlock uint32
	BlsSignature     bls.Signature
	OperatorId       eigentypes.OperatorId
}
