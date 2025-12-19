package config

import (
	"errors"
	"log"
	"os"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/yetanotherco/aligned_layer/core/utils"
)

type OperatorConfig struct {
	BaseConfig                   *BaseConfig
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
		PollLatestBatchInterval       time.Duration
		MinaVerifierEnabled           bool
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
		PollLatestBatchInterval       string         `yaml:"poll_latest_batch_interval"`
		MinaVerifierEnabled           bool           `yaml:"mina_verifier_enabled"`
	} `yaml:"operator"`
	BlsConfigFromYaml BlsConfigFromYaml `yaml:"bls"`
}

func NewOperatorConfig(configFilePath string) *OperatorConfig {
	if _, err := os.Stat(configFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup config file does not exist")
	}

	baseConfig := NewBaseConfig(configFilePath)
	if baseConfig == nil {
		log.Fatal("Error reading base config: ")
	}

	blsConfig := NewBlsConfig(configFilePath)
	if blsConfig == nil {
		log.Fatal("Error reading bls config: ")
	}

	var operatorConfigFromYaml OperatorConfigFromYaml
	err := utils.ReadYamlConfig(configFilePath, &operatorConfigFromYaml)

	if err != nil {
		log.Fatal("Error reading operator config: ", err)
	}

	pollInterval := 20 * time.Second
	if operatorConfigFromYaml.Operator.PollLatestBatchInterval != "" {
		if parsed, err := time.ParseDuration(operatorConfigFromYaml.Operator.PollLatestBatchInterval); err == nil {
			pollInterval = parsed
		}
	}

	return &OperatorConfig{
		BaseConfig:                   baseConfig,
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
			PollLatestBatchInterval       time.Duration
			MinaVerifierEnabled           bool
		}{
			AggregatorServerIpPortAddress: operatorConfigFromYaml.Operator.AggregatorServerIpPortAddress,
			OperatorTrackerIpPortAddress:  operatorConfigFromYaml.Operator.OperatorTrackerIpPortAddress,
			Address:                       operatorConfigFromYaml.Operator.Address,
			EarningsReceiverAddress:       operatorConfigFromYaml.Operator.EarningsReceiverAddress,
			DelegationApproverAddress:     operatorConfigFromYaml.Operator.DelegationApproverAddress,
			StakerOptOutWindowBlocks:      operatorConfigFromYaml.Operator.StakerOptOutWindowBlocks,
			MetadataUrl:                   operatorConfigFromYaml.Operator.MetadataUrl,
			RegisterOperatorOnStartup:     operatorConfigFromYaml.Operator.RegisterOperatorOnStartup,
			EnableMetrics:                 operatorConfigFromYaml.Operator.EnableMetrics,
			MetricsIpPortAddress:          operatorConfigFromYaml.Operator.MetricsIpPortAddress,
			MaxBatchSize:                  operatorConfigFromYaml.Operator.MaxBatchSize,
			LastProcessedBatchFilePath:    operatorConfigFromYaml.Operator.LastProcessedBatchFilePath,
			PollLatestBatchInterval:       pollInterval,
			MinaVerifierEnabled:           operatorConfigFromYaml.Operator.MinaVerifierEnabled,
		},
	}
}
