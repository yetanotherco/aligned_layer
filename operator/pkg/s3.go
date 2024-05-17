package operator

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
)

func (o *Operator) getBatchFromS3(proofUrl string) ([]VerificationData, error) {
	log.Println("Getting batch from S3..., proofUrl:", proofUrl)
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
