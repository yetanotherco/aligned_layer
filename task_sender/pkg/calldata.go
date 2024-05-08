package pkg

import (
	"github.com/yetanotherco/aligned_layer/common"
	serviceManager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"log"
)

func (ts *TaskSender) PostProofOnCalldata(proof []byte) (*serviceManager.AlignedLayerServiceManagerDAPayload, error) {
	log.Println("Posting proof on Calldata...")

	DAPayload := &serviceManager.AlignedLayerServiceManagerDAPayload{
		Solution:            common.Calldata,
		ProofAssociatedData: proof,
	}

	return DAPayload, nil
}
