package operator

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

func (o *Operator) getBatchFromS3(proofUrl string) ([]VerificationData, error) {
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
}
