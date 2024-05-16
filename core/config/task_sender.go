package config

import (
	"errors"
	"github.com/celestiaorg/celestia-node/api/rpc/perms"
	alcommon "github.com/yetanotherco/aligned_layer/common"
	"log"
	"os"
)

type TaskSenderConfig struct {
	BaseConfig     *BaseConfig
	EcdsaConfig    *EcdsaConfig
	EigenDAConfig  *EigenDAConfig
	CelestiaConfig *CelestiaConfig
}

type TaskSenderConfigFromYaml struct {
	EcdsaConfigFromYaml EcdsaConfigFromYaml `yaml:"ecdsa"`
}

func NewTaskSenderConfig(configFilePath string, sol alcommon.DASolution) *TaskSenderConfig {
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

	var (
		eigenDAConfig  *EigenDAConfig
		celestiaConfig *CelestiaConfig
	)

	switch sol {
	case alcommon.EigenDA:
		eigenDAConfig = NewEigenDAConfig(configFilePath)
	case alcommon.Celestia:
		celestiaConfig = NewCelestiaConfig(configFilePath, perms.ReadWritePerms)
	case alcommon.Calldata:
	default:
		log.Fatal("Invalid solution")
	}

	return &TaskSenderConfig{
		BaseConfig:     baseConfig,
		EcdsaConfig:    ecdsaConfig,
		EigenDAConfig:  eigenDAConfig,
		CelestiaConfig: celestiaConfig,
	}
}
