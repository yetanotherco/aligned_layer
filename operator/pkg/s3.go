package operator

import (
	"fmt"
	"os"
	"strings"

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
	// Improvement: now reads from local files, but should be replaced with actual s3 read
	proofBytes, err := os.ReadFile(_batchDataPointer + ".proof")
	if err != nil {
		return nil, fmt.Errorf("error loading proof file: %v", err)
	}

	pubInputBytes, err := os.ReadFile(_batchDataPointer + ".pub")
	if err != nil {
		return nil, fmt.Errorf("error loading public input file: %v", err)
	}

	verificationKeyBytes, err := os.ReadFile(_batchDataPointer + ".vk")
	if err != nil {
		return nil, fmt.Errorf("error loading verification key file: %v", err)
	}

	// Extract the substring between the last underscore and the end of the string
	// This substring should contain the proving system name
	lastUnderscoreIndex := strings.LastIndex(_batchDataPointer, "_")
	provingSystemText := _batchDataPointer[lastUnderscoreIndex+1:]

	var currentProvingSystemId common.ProvingSystemId
	switch provingSystemText {
	case "groth16":
		currentProvingSystemId = common.Groth16Bn254
	default: //TODO add all cases
		currentProvingSystemId = common.GnarkPlonkBn254
	}

	verificationData := VerificationData{
		ProvingSystemId: currentProvingSystemId,
		Proof:           proofBytes,
		PubInput:        pubInputBytes,
		VerificationKey: verificationKeyBytes,
	}

	batch := []VerificationData{verificationData}

	return batch, nil
}
