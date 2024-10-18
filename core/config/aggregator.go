package config

import (
	"errors"
	"log"
	"os"
	"time"

	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"github.com/ethereum/go-ethereum/common"
)

type AggregatorConfig struct {
	BaseConfig  *BaseConfig
	EcdsaConfig *EcdsaConfig
	BlsConfig   *BlsConfig
	Aggregator  struct {
		ServerIpPortAddress           string
		BlsPublicKeyCompendiumAddress common.Address
		AvsServiceManagerAddress      common.Address
		EnableMetrics                 bool
		MetricsIpPortAddress          string
		TelemetryIpPortAddress        string
		GarbageCollectorPeriod        time.Duration
		GarbageCollectorTasksAge      uint64
		GarbageCollectorTasksInterval uint64
	}
}

type AggregatorConfigFromYaml struct {
	Aggregator struct {
		ServerIpPortAddress           string         `yaml:"server_ip_port_address"`
		BlsPublicKeyCompendiumAddress common.Address `yaml:"bls_public_key_compendium_address"`
		AvsServiceManagerAddress      common.Address `yaml:"avs_service_manager_address"`
		EnableMetrics                 bool           `yaml:"enable_metrics"`
		MetricsIpPortAddress          string         `yaml:"metrics_ip_port_address"`
		TelemetryIpPortAddress        string         `yaml:"telemetry_ip_port_address"`
		GarbageCollectorPeriod        time.Duration  `yaml:"garbage_collector_period"`
		GarbageCollectorTasksAge      uint64         `yaml:"garbage_collector_tasks_age"`
		GarbageCollectorTasksInterval uint64         `yaml:"garbage_collector_tasks_interval"`
	} `yaml:"aggregator"`
}

func NewAggregatorConfig(configFilePath string) *AggregatorConfig {

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

	blsConfig := NewBlsConfig(configFilePath)
	if blsConfig == nil {
		log.Fatal("Error reading bls config: ")
	}

	var aggregatorConfigFromYaml AggregatorConfigFromYaml
	err := sdkutils.ReadYamlConfig(configFilePath, &aggregatorConfigFromYaml)
	if err != nil {
		log.Fatal("Error reading aggregator config: ", err)
	}

	return &AggregatorConfig{
		BaseConfig:  baseConfig,
		EcdsaConfig: ecdsaConfig,
		BlsConfig:   blsConfig,
		Aggregator: struct {
			ServerIpPortAddress           string
			BlsPublicKeyCompendiumAddress common.Address
			AvsServiceManagerAddress      common.Address
			EnableMetrics                 bool
			MetricsIpPortAddress          string
			TelemetryIpPortAddress        string
			GarbageCollectorPeriod        time.Duration
			GarbageCollectorTasksAge      uint64
			GarbageCollectorTasksInterval uint64
		}(aggregatorConfigFromYaml.Aggregator),
	}
}
