package types

import (
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
)

type SignedTaskResponse struct {
	TaskResponse servicemanager.AlignedLayerServiceManagerBatchProofVerificationTaskResponse
	BlsSignature bls.Signature
	OperatorId   eigentypes.OperatorId
}
