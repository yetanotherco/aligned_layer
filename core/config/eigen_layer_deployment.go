package config

import (
	"errors"
	"log"
	"os"

	"github.com/ethereum/go-ethereum/common"
	"github.com/yetanotherco/aligned_layer/core/utils"
)

type EigenLayerDeploymentConfig struct {
	DelegationManagerAddr common.Address
	AVSDirectoryAddr      common.Address
}

type EigenLayerDeploymentConfigFromJson struct {
	Addresses struct {
		DelegationManagerAddr common.Address `json:"delegationManager"`
		AVSDirectoryAddr      common.Address `json:"avsDirectory"`
	} `json:"addresses"`
}

func NewEigenLayerDeploymentConfig(eigenLayerDeploymentFilePath string) *EigenLayerDeploymentConfig {

	if _, err := os.Stat(eigenLayerDeploymentFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup eigen layer deployment file does not exist")
	}

	var eigenLayerDeploymentConfigFromJson EigenLayerDeploymentConfigFromJson
	err := utils.ReadJsonConfig(eigenLayerDeploymentFilePath, &eigenLayerDeploymentConfigFromJson)

	if err != nil {
		log.Fatal("Error reading eigen layer deployment config: ", err)
	}

	if eigenLayerDeploymentConfigFromJson.Addresses.DelegationManagerAddr == common.HexToAddress("") {
		log.Fatal("Delegation manager address is empty")
	}

	if eigenLayerDeploymentConfigFromJson.Addresses.AVSDirectoryAddr == common.HexToAddress("") {
		log.Fatal("AVS directory address is empty")
	}

	return &EigenLayerDeploymentConfig{
		DelegationManagerAddr: eigenLayerDeploymentConfigFromJson.Addresses.DelegationManagerAddr,
		AVSDirectoryAddr:      eigenLayerDeploymentConfigFromJson.Addresses.AVSDirectoryAddr,
	}
}
