package config

import (
	"errors"
	"log"
	"os"

	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"github.com/ethereum/go-ethereum/common"
)

type OperatorConfig struct {
	BaseConfig                   *BaseConfig
	EcdsaConfig                  *EcdsaConfig
	BlsConfig                    *BlsConfig
	AlignedLayerDeploymentConfig *AlignedLayerDeploymentConfig

	Operator struct {
		AggregatorServerIpPortAddress string
		OperatorTrackerIpPortAddress  string
		Address                       common.Address
		EarningsReceiverAddress       common.Address
		DelegationApproverAddress     common.Address
		StakerOptOutWindowBlocks      int
		MetadataUrl                   string
		RegisterOperatorOnStartup     bool
		EnableMetrics                 bool
		MetricsIpPortAddress          string
		MaxBatchSize                  int64
		LastProcessedBatchFilePath    string
	}
}

type OperatorConfigFromYaml struct {
	Operator struct {
		AggregatorServerIpPortAddress string         `yaml:"aggregator_rpc_server_ip_port_address"`
		OperatorTrackerIpPortAddress  string         `yaml:"operator_tracker_ip_port_address"`
		Address                       common.Address `yaml:"address"`
		EarningsReceiverAddress       common.Address `yaml:"earnings_receiver_address"`
		DelegationApproverAddress     common.Address `yaml:"delegation_approver_address"`
		StakerOptOutWindowBlocks      int            `yaml:"staker_opt_out_window_blocks"`
		MetadataUrl                   string         `yaml:"metadata_url"`
		RegisterOperatorOnStartup     bool           `yaml:"register_operator_on_startup"`
		EnableMetrics                 bool           `yaml:"enable_metrics"`
		MetricsIpPortAddress          string         `yaml:"metrics_ip_port_address"`
		MaxBatchSize                  int64          `yaml:"max_batch_size"`
		LastProcessedBatchFilePath    string         `yaml:"last_processed_batch_filepath"`
	} `yaml:"operator"`
	EcdsaConfigFromYaml EcdsaConfigFromYaml `yaml:"ecdsa"`
	BlsConfigFromYaml   BlsConfigFromYaml   `yaml:"bls"`
}

func NewOperatorConfig(configFilePath string) *OperatorConfig {
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

	var operatorConfigFromYaml OperatorConfigFromYaml
	err := sdkutils.ReadYamlConfig(configFilePath, &operatorConfigFromYaml)

	if err != nil {
		log.Fatal("Error reading operator config: ", err)
	}

	return &OperatorConfig{
		BaseConfig:                   baseConfig,
		EcdsaConfig:                  ecdsaConfig,
		BlsConfig:                    blsConfig,
		AlignedLayerDeploymentConfig: baseConfig.AlignedLayerDeploymentConfig,
		Operator: struct {
			AggregatorServerIpPortAddress string
			OperatorTrackerIpPortAddress  string
			Address                       common.Address
			EarningsReceiverAddress       common.Address
			DelegationApproverAddress     common.Address
			StakerOptOutWindowBlocks      int
			MetadataUrl                   string
			RegisterOperatorOnStartup     bool
			EnableMetrics                 bool
			MetricsIpPortAddress          string
			MaxBatchSize                  int64
			LastProcessedBatchFilePath    string
		}(operatorConfigFromYaml.Operator),
	}
}
