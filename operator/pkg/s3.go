package operator

import (
	"fmt"
	"os"

	"github.com/yetanotherco/aligned_layer/common"
)

func (o *Operator) getBatchFromS3(_batchDataPointer string) ([]VerificationData, error) {
	// resp, err := http.Get(o.Config.S3BucketConfig.Url + proofId)
	// if err != nil {
	// 	return nil, err
	// }
	// defer resp.Body.Close()

	// proof, err := io.ReadAll(resp.Body)
	// if err != nil {
	// 	return nil, err
	// }

	// FIXME(marian): We are simulating getting the batch from s3, but this is hardcoded right now
	proofBytes, err := os.ReadFile("./task_sender/test_examples/gnark_plonk_bn254_script/plonk.proof")
	if err != nil {
		return nil, fmt.Errorf("error loading proof file: %v", err)
	}

	pubInputBytes, err := os.ReadFile("./task_sender/test_examples/gnark_plonk_bn254_script/plonk_pub_input.pub")
	if err != nil {
		return nil, fmt.Errorf("error loading public input file: %v", err)
	}

	verificationKeyBytes, err := os.ReadFile("./task_sender/test_examples/gnark_plonk_bn254_script/plonk.vk")
	if err != nil {
		return nil, fmt.Errorf("error loading verification key file: %v", err)
	}

	verificationData := VerificationData{ProvingSystemId: common.GnarkPlonkBn254, Proof: proofBytes, PubInput: pubInputBytes, VerificationKey: verificationKeyBytes}

	batch := []VerificationData{verificationData}

	return batch, nil
}
