package config

import (
	"errors"
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"log"
	"os"
)

type BlsConfig struct {
	KeyPair *bls.KeyPair
}

type BlsConfigFromYaml struct {
	Bls struct {
		PrivateKeyStorePath     string `yaml:"private_key_store_path"`
		PrivateKeyStorePassword string `yaml:"private_key_store_password"`
	} `yaml:"bls"`
}

func NewBlsConfig(blsConfigFilePath string) *BlsConfig {
	if _, err := os.Stat(blsConfigFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup bls config file does not exist")
	}

	var blsConfigFromYaml BlsConfigFromYaml
	err := sdkutils.ReadYamlConfig(blsConfigFilePath, &blsConfigFromYaml)
	if err != nil {
		log.Fatal("Error reading bls config: ", err)
	}

	if blsConfigFromYaml.Bls.PrivateKeyStorePath == "" {
		log.Fatal("Bls private key store path is empty")
	}

	blsKeyPair, err := bls.ReadPrivateKeyFromFile(blsConfigFromYaml.Bls.PrivateKeyStorePath, blsConfigFromYaml.Bls.PrivateKeyStorePassword)
	if err != nil {
		log.Fatal("Error reading bls private key from file: ", err)
	}

	return &BlsConfig{
		KeyPair: blsKeyPair,
	}
}
