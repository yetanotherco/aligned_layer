package operator

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
)

func (o *Operator) getBatchFromS3(proofHash string) ([]VerificationData, error) {
	log.Println("Getting batch from S3..., proofHash:", proofHash)
	url := o.Config.S3BucketConfig.Url + proofHash
	resp, err := http.Get(url)
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
