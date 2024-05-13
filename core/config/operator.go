package config

import (
	"errors"
	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"github.com/celestiaorg/celestia-node/api/rpc/perms"
	"github.com/ethereum/go-ethereum/common"
	"log"
	"os"
)

type OperatorConfig struct {
	BaseConfig                   *BaseConfig
	EcdsaConfig                  *EcdsaConfig
	BlsConfig                    *BlsConfig
	AlignedLayerDeploymentConfig *AlignedLayerDeploymentConfig
	EigenDADisperserConfig       *EigenDAConfig
	CelestiaConfig               *CelestiaConfig

	Operator struct {
		AggregatorServerIpPortAddress string
		Address                       common.Address
		EarningsReceiverAddress       common.Address
		DelegationApproverAddress     common.Address
		StakerOptOutWindowBlocks      int
		MetadataUrl                   string
		RegisterOperatorOnStartup     bool
	}
}

type OperatorConfigFromYaml struct {
	Operator struct {
		AggregatorServerIpPortAddress string         `yaml:"aggregator_rpc_server_ip_port_address"`
		Address                       common.Address `yaml:"address"`
		EarningsReceiverAddress       common.Address `yaml:"earnings_receiver_address"`
		DelegationApproverAddress     common.Address `yaml:"delegation_approver_address"`
		StakerOptOutWindowBlocks      int            `yaml:"staker_opt_out_window_blocks"`
		MetadataUrl                   string         `yaml:"metadata_url"`
		RegisterOperatorOnStartup     bool           `yaml:"register_operator_on_startup"`
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

	eigenDADisperserConfig := NewEigenDAConfig(configFilePath)

	celestiaConfig := NewCelestiaConfig(configFilePath, perms.ReadPerms)

	return &OperatorConfig{
		BaseConfig:                   baseConfig,
		EcdsaConfig:                  ecdsaConfig,
		BlsConfig:                    blsConfig,
		AlignedLayerDeploymentConfig: baseConfig.AlignedLayerDeploymentConfig,
		EigenDADisperserConfig:       eigenDADisperserConfig,
		CelestiaConfig:               celestiaConfig,
		Operator: struct {
			AggregatorServerIpPortAddress string
			Address                       common.Address
			EarningsReceiverAddress       common.Address
			DelegationApproverAddress     common.Address
			StakerOptOutWindowBlocks      int
			MetadataUrl                   string
			RegisterOperatorOnStartup     bool
		}(operatorConfigFromYaml.Operator),
	}
}
