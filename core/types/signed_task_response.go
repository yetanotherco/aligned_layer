package types

import (
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
)

type SignedTaskResponse struct {
	BatchMerkleRoot [32]byte
	SenderAddress [20]byte
	BatchIdentifierHash [32]byte
	BlsSignature    bls.Signature
	OperatorId      eigentypes.OperatorId
}
