package config

import (
	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"log"
)

type S3BucketConfig struct {
	Url string
}

type S3BucketConfigFromYaml struct {
	S3Bucket struct {
		Url string `yaml:"url"`
	} `yaml:"s3_bucket"`
}

func NewS3BucketConfig(s3BucketConfigFilePath string) *S3BucketConfig {
	var s3BucketConfigFromYaml S3BucketConfigFromYaml
	err := sdkutils.ReadYamlConfig(s3BucketConfigFilePath, &s3BucketConfigFromYaml)
	if err != nil {
		log.Fatal("Error reading s3 bucket config: ", err)
	}

	if s3BucketConfigFromYaml.S3Bucket.Url == "" {
		log.Fatal("S3 bucket url is empty")
	}

	return &S3BucketConfig{
		Url: s3BucketConfigFromYaml.S3Bucket.Url,
	}
}
