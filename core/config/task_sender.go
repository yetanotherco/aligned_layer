package config

import (
	"errors"
	"log"
	"os"
)

type TaskSenderConfig struct {
	BaseConfig  *BaseConfig
	EcdsaConfig *EcdsaConfig
}

type TaskSenderConfigFromYaml struct {
	EcdsaConfigFromYaml EcdsaConfigFromYaml `yaml:"ecdsa"`
}

func NewTaskSenderConfig(configFilePath string) *TaskSenderConfig {
	if _, err := os.Stat(configFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup config file does not exist")
	}

	baseConfig := NewBaseConfig(configFilePath)
	if baseConfig == nil {
		log.Fatal("Error reading base config: ")
	}

	ecdsaConfig := NewEcdsaConfig(configFilePath, baseConfig.ChainId)
	if ecdsaConfig == nil {
		log.Fatal("Error reading ecdsa config: ")
	}

	return &TaskSenderConfig{
		BaseConfig:  baseConfig,
		EcdsaConfig: ecdsaConfig,
	}
}
