package pkg

import (
	"github.com/yetanotherco/aligned_layer/common"
	serviceManager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"log"
)

func (ts *TaskSender) PostProofOnCalldata(proof []byte) (*serviceManager.AlignedLayerServiceManagerDAPayload, error) {
	log.Println("Posting proof on Calldata...")

	chunk := serviceManager.AlignedLayerServiceManagerDAPayloadChunk{
		ProofAssociatedData: proof,
	}
	DAPayload := &serviceManager.AlignedLayerServiceManagerDAPayload{
		Solution: common.Calldata,
		Chunks:   []serviceManager.AlignedLayerServiceManagerDAPayloadChunk{chunk},
	}

	return DAPayload, nil
}
