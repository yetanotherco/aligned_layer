package config

import (
	"crypto/tls"
	"errors"
	"github.com/Layr-Labs/eigenda/api/grpc/disperser"
	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"log"
	"os"
)

type EigenDAConfig struct {
	Disperser disperser.DisperserClient
}

type EigenDAConfigFromYaml struct {
	EigenDA struct {
		Url string `yaml:"url"`
	} `yaml:"eigen_da"`
}

func NewEigenDAConfig(eigenDAConfigFilePath string) *EigenDAConfig {
	if _, err := os.Stat(eigenDAConfigFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup eigen da config file does not exist")
	}

	var eigenDAConfigFromYaml EigenDAConfigFromYaml
	err := sdkutils.ReadYamlConfig(eigenDAConfigFilePath, &eigenDAConfigFromYaml)
	if err != nil {
		log.Fatal("Error reading eigen da config: ", err)
	}

	if eigenDAConfigFromYaml.EigenDA.Url == "" {
		log.Fatal("Eigen DA url is empty")
	}

	tlsConfig := &tls.Config{}
	credential := credentials.NewTLS(tlsConfig)

	clientConn, err := grpc.NewClient(eigenDAConfigFromYaml.EigenDA.Url, grpc.WithTransportCredentials(credential))
	if err != nil {
		log.Fatal("Error creating grpc client: ", err)
	}

	disperserClient := disperser.NewDisperserClient(clientConn)

	return &EigenDAConfig{
		Disperser: disperserClient,
	}
}
