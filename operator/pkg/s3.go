package operator

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

func (o *Operator) getBatchFromS3(proofUrl string) ([]VerificationData, error) {
	o.Logger.Infof("Getting batch from S3..., proofUrl: %s", proofUrl)
	resp, err := http.Head(proofUrl)
	if err != nil {
		return nil, err
	}

	// Check if the response is OK
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("error getting Proof Head from S3: %s", resp.Status)
	}

	if resp.ContentLength > o.Config.Operator.MaxBatchSize {
		return nil, fmt.Errorf("proof size %d exceeds max batch size %d",
			resp.ContentLength, o.Config.Operator.MaxBatchSize)
	}

	resp, err = http.Get(proofUrl)
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
}
