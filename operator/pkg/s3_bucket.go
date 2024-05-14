package operator

import (
	"io"
	"net/http"
)

// TODO(Nico): If we batch the proofs, we need to receive a list of proofIds and fetch all of them
func (o *Operator) getProofFromS3Bucket(proofId string) ([]byte, error) {

	resp, err := http.Get(o.Config.S3BucketConfig.Url + proofId)
	if err != nil {
		return nil, err
	}

	defer resp.Body.Close()

	proof, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	return proof, nil
}
