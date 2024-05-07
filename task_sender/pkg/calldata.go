package pkg

import (
	"github.com/yetanotherco/aligned_layer/common"
	serviceManager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"log"
)

func (ts *TaskSender) PostProofOnCalldata(proof []byte) (*serviceManager.AlignedLayerServiceManagerTaskDA, error) {
	log.Println("Posting proof on Calldata...")

	taskDA := &serviceManager.AlignedLayerServiceManagerTaskDA{
		Solution: common.Calldata,
		Proof:    proof,
	}

	return taskDA, nil
}
