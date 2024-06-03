package config

import (
	"errors"
	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"github.com/ethereum/go-ethereum/common"
	"log"
	"os"
)

type AlignedLayerDeploymentConfig struct {
	AlignedLayerServiceManagerAddr         common.Address
	AlignedLayerRegistryCoordinatorAddr    common.Address
	AlignedLayerOperatorStateRetrieverAddr common.Address
}

type AlignedLayerDeploymentConfigFromJson struct {
	Addresses struct {
		AlignedLayerServiceManagerAddr         common.Address `json:"alignedLayerServiceManager"`
		AlignedLayerRegistryCoordinatorAddr    common.Address `json:"registryCoordinator"`
		AlignedLayerOperatorStateRetrieverAddr common.Address `json:"operatorStateRetriever"`
	} `json:"addresses"`
}

func NewAlignedLayerDeploymentConfig(alignedLayerDeploymentFilePath string) *AlignedLayerDeploymentConfig {

	if _, err := os.Stat(alignedLayerDeploymentFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup aligned layer deployment file does not exist")
	}

	var alignedLayerDeploymentConfigFromJson AlignedLayerDeploymentConfigFromJson
	err := sdkutils.ReadJsonConfig(alignedLayerDeploymentFilePath, &alignedLayerDeploymentConfigFromJson)

	if err != nil {
		log.Fatal("Error reading aligned layer deployment config: ", err)
	}

	if alignedLayerDeploymentConfigFromJson.Addresses.AlignedLayerServiceManagerAddr == common.HexToAddress("") {
		log.Fatal("Aligned layer service manager address is empty")
	}

	if alignedLayerDeploymentConfigFromJson.Addresses.AlignedLayerRegistryCoordinatorAddr == common.HexToAddress("") {
		log.Fatal("Aligned layer registry coordinator address is empty")
	}

	if alignedLayerDeploymentConfigFromJson.Addresses.AlignedLayerOperatorStateRetrieverAddr == common.HexToAddress("") {
		log.Fatal("Aligned layer operator state retriever address is empty")
	}

	return &AlignedLayerDeploymentConfig{
		AlignedLayerServiceManagerAddr:         alignedLayerDeploymentConfigFromJson.Addresses.AlignedLayerServiceManagerAddr,
		AlignedLayerRegistryCoordinatorAddr:    alignedLayerDeploymentConfigFromJson.Addresses.AlignedLayerRegistryCoordinatorAddr,
		AlignedLayerOperatorStateRetrieverAddr: alignedLayerDeploymentConfigFromJson.Addresses.AlignedLayerOperatorStateRetrieverAddr,
	}
}
