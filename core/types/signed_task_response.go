package types

import (
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	"github.com/Layr-Labs/eigensdk-go/types"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
)

type SignedTaskResponse struct {
	TaskResponse servicemanager.AlignedLayerServiceManagerTask
	BlsSignature bls.Signature
	OperatorId   types.OperatorId
}
