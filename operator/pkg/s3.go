package operator

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"github.com/yetanotherco/aligned_layer/common"
)

func (o *Operator) getBatchFromS3(proofUrl string) ([]VerificationData, error) {
	is_s3 := true
	if is_s3 {
		o.Logger.Infof("Getting batch from S3..., proofUrl: %s", proofUrl)
		resp, err := http.Get(proofUrl)
		if err != nil {
			return nil, err
		}
		defer func(Body io.ReadCloser) {
			err := Body.Close()
			if err != nil {
				fmt.Println("error closing body: ", err)
			}
		}(resp.Body)

		proof, err := io.ReadAll(resp.Body)
		if err != nil {
			return nil, err
		}

		var batch []VerificationData

		err = json.Unmarshal(proof, &batch)
		if err != nil {
			return nil, err
		}

		return batch, nil
	} else { //is local files
		proofBytes, err := os.ReadFile(proofUrl + ".proof")
		if err != nil {
			return nil, fmt.Errorf("error loading proof file: %v", err)
		}

		pubInputBytes, err := os.ReadFile(proofUrl + ".pub")
		if err != nil {
			return nil, fmt.Errorf("error loading public input file: %v", err)
		}

		verificationKeyBytes, err := os.ReadFile(proofUrl + ".vk")
		if err != nil {
			return nil, fmt.Errorf("error loading verification key file: %v", err)
		}

		// Extract the substring between the last underscore and the end of the string
		// This substring should contain the proving system name
		lastUnderscoreIndex := strings.LastIndex(proofUrl, "_")
		provingSystemText := proofUrl[lastUnderscoreIndex+1:]

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
}