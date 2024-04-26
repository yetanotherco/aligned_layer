package config

import (
	"context"
	"crypto/ecdsa"
	"errors"
	ecdsa2 "github.com/Layr-Labs/eigensdk-go/crypto/ecdsa"
	"github.com/ethereum/go-ethereum/common"
	"github.com/urfave/cli/v2"
	"log"
	"math/big"
	"os"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"

	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
)

type BaseConfig struct {
	AlignedLayerDeploymentConfig *AlignedLayerDeploymentConfig
	EigenLayerDeploymentConfig   *EigenLayerDeploymentConfig
	Logger                       sdklogging.Logger
	EthRpcUrl                    string
	EthWsUrl                     string
	EthRpcClient                 eth.Client
	EthWsClient                  eth.Client
	EigenMetricsIpPortAddress    string
	ChainId                      *big.Int
}

type BaseConfigFromYaml struct {
	AlignedLayerDeploymentConfigFilePath string              `yaml:"aligned_layer_deployment_config_file_path"`
	EigenLayerDeploymentConfigFilePath   string              `yaml:"eigen_layer_deployment_config_file_path"`
	Environment                          sdklogging.LogLevel `yaml:"environment"`
	EthRpcUrl                            string              `yaml:"eth_rpc_url"`
	EthWsUrl                             string              `yaml:"eth_ws_url"`
	EigenMetricsIpPortAddress            string              `yaml:"eigen_metrics_ip_port_address"`
}

type EcdsaConfig struct {
	PrivateKey *ecdsa.PrivateKey
	Signer     signer.Signer
}

type EcdsaConfigFromYaml struct {
	Ecdsa struct {
		PrivateKeyStorePath     string `yaml:"private_key_store_path"`
		PrivateKeyStorePassword string `yaml:"private_key_store_password"`
	} `yaml:"ecdsa"`
}

type BlsConfig struct {
	KeyPair *bls.KeyPair
}

type BlsConfigFromYaml struct {
	Bls struct {
		PrivateKeyStorePath     string `yaml:"private_key_store_path"`
		PrivateKeyStorePassword string `yaml:"private_key_store_password"`
	} `yaml:"bls"`
}

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

type EigenLayerDeploymentConfig struct {
	DelegationManagerAddr common.Address
	AVSDirectoryAddr      common.Address
	SlasherAddr           common.Address
}

type EigenLayerDeploymentConfigFromJson struct {
	Addresses struct {
		DelegationManagerAddr common.Address `json:"delegationManager"`
		AVSDirectoryAddr      common.Address `json:"avsDirectory"`
		SlasherAddr           common.Address `json:"slasher"`
	} `json:"addresses"`
}

type AggregatorConfig struct {
	BaseConfig  *BaseConfig
	EcdsaConfig *EcdsaConfig
	BlsConfig   *BlsConfig
	Aggregator  struct {
		ServerIpPortAddress           string
		BlsPublicKeyCompendiumAddress common.Address
		AvsServiceManagerAddress      common.Address
		EnableMetrics                 bool
	}
}

type AggregatorConfigFromYaml struct {
	Aggregator struct {
		ServerIpPortAddress           string         `yaml:"server_ip_port_address"`
		BlsPublicKeyCompendiumAddress common.Address `yaml:"bls_public_key_compendium_address"`
		AvsServiceManagerAddress      common.Address `yaml:"avs_service_manager_address"`
		EnableMetrics                 bool           `yaml:"enable_metrics"`
	} `yaml:"aggregator"`
}

type OperatorConfig struct {
	BaseConfig  *BaseConfig
	EcdsaConfig *EcdsaConfig
	BlsConfig   *BlsConfig
	Operator    struct {
		Address                   common.Address
		EarningsReceiverAddress   common.Address
		DelegationApproverAddress common.Address
		StakerOptOutWindowBlocks  int
		MetadataUrl               string
		RegisterOperatorOnStartup bool
	}
}

type OperatorConfigFromYaml struct {
	Operator struct {
		Address                   common.Address `yaml:"address"`
		EarningsReceiverAddress   common.Address `yaml:"earnings_receiver_address"`
		DelegationApproverAddress common.Address `yaml:"delegation_approver_address"`
		StakerOptOutWindowBlocks  int            `yaml:"staker_opt_out_window_blocks"`
		MetadataUrl               string         `yaml:"metadata_url"`
		RegisterOperatorOnStartup bool           `yaml:"register_operator_on_startup"`
	} `yaml:"operator"`
	EcdsaConfigFromYaml EcdsaConfigFromYaml `yaml:"ecdsa"`
	BlsConfigFromYaml   BlsConfigFromYaml   `yaml:"bls"`
}

type TaskSenderConfig struct {
	BaseConfig  *BaseConfig
	EcdsaConfig *EcdsaConfig
}

type TaskSenderConfigFromYaml struct {
	EcdsaConfigFromYaml EcdsaConfigFromYaml `yaml:"ecdsa"`
}

var (
	ConfigFileFlag = &cli.StringFlag{
		Name:     "config",
		Required: true,
		Usage:    "Load base configurations from `FILE`",
	}
)

func NewBaseConfig(configFilePath string) *BaseConfig {

	if _, err := os.Stat(configFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup base config file does not exist")
	}

	var baseConfigFromYaml BaseConfigFromYaml

	err := sdkutils.ReadYamlConfig(configFilePath, &baseConfigFromYaml)
	if err != nil {
		log.Fatal("Error reading setup config: ", err)
	}

	alignedLayerDeploymentConfigFilePath := baseConfigFromYaml.AlignedLayerDeploymentConfigFilePath
	if alignedLayerDeploymentConfigFilePath == "" {
		log.Fatal("Aligned layer deployment config file path is empty")
	}

	if _, err := os.Stat(alignedLayerDeploymentConfigFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup aligned layer deployment file does not exist")
	}

	alignedLayerDeploymentConfig := newAlignedLayerDeploymentConfig(alignedLayerDeploymentConfigFilePath)
	if alignedLayerDeploymentConfig == nil {
		log.Fatal("Error reading aligned layer deployment config: ", err)
	}

	eigenLayerDeploymentConfigFilePath := baseConfigFromYaml.EigenLayerDeploymentConfigFilePath
	if eigenLayerDeploymentConfigFilePath == "" {
		log.Fatal("Eigen layer deployment config file path is empty")
	}

	if _, err := os.Stat(eigenLayerDeploymentConfigFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup eigen layer deployment file does not exist")
	}
	eigenLayerDeploymentConfig := newEigenLayerDeploymentConfig(baseConfigFromYaml.EigenLayerDeploymentConfigFilePath)

	if eigenLayerDeploymentConfig == nil {
		log.Fatal("Error reading eigen layer deployment config: ", err)
	}
	logger, err := NewLogger(baseConfigFromYaml.Environment)

	if err != nil {
		log.Fatal("Error initializing logger: ", err)
	}

	if baseConfigFromYaml.EthWsUrl == "" {
		log.Fatal("Eth ws url is empty")
	}

	ethWsClient, err := eth.NewClient(baseConfigFromYaml.EthWsUrl)

	if err != nil {
		log.Fatal("Error initializing eth ws client: ", err)
	}

	if baseConfigFromYaml.EthRpcUrl == "" {
		log.Fatal("Eth rpc url is empty")
	}

	ethRpcClient, err := eth.NewClient(baseConfigFromYaml.EthRpcUrl)
	if err != nil {
		log.Fatal("Error initializing eth rpc client: ", err)
	}

	chainId, err := ethRpcClient.ChainID(context.Background())
	if err != nil {
		logger.Error("Cannot get chainId from eth rpc client", "err", err)
		return nil
	}

	if baseConfigFromYaml.EigenMetricsIpPortAddress == "" {
		log.Fatal("Eigen metrics ip port address is empty")
	}

	return &BaseConfig{
		AlignedLayerDeploymentConfig: alignedLayerDeploymentConfig,
		EigenLayerDeploymentConfig:   eigenLayerDeploymentConfig,
		Logger:                       logger,
		EthRpcUrl:                    baseConfigFromYaml.EthRpcUrl,
		EthWsUrl:                     baseConfigFromYaml.EthWsUrl,
		EthRpcClient:                 ethRpcClient,
		EthWsClient:                  ethWsClient,
		EigenMetricsIpPortAddress:    baseConfigFromYaml.EigenMetricsIpPortAddress,
		ChainId:                      chainId,
	}
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

	blsConfig := newBlsConfig(configFilePath)
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
		}(aggregatorConfigFromYaml.Aggregator),
	}
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

	blsConfig := newBlsConfig(configFilePath)
	if blsConfig == nil {
		log.Fatal("Error reading bls config: ")
	}

	var operatorConfigFromYaml OperatorConfigFromYaml
	err := sdkutils.ReadYamlConfig(configFilePath, &operatorConfigFromYaml)

	if err != nil {
		log.Fatal("Error reading operator config: ", err)
	}

	return &OperatorConfig{
		BaseConfig:  baseConfig,
		EcdsaConfig: ecdsaConfig,
		BlsConfig:   blsConfig,
		Operator: struct {
			Address                   common.Address
			EarningsReceiverAddress   common.Address
			DelegationApproverAddress common.Address
			StakerOptOutWindowBlocks  int
			MetadataUrl               string
			RegisterOperatorOnStartup bool
		}(operatorConfigFromYaml.Operator),
	}
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

func NewEcdsaConfig(ecdsaConfigFilePath string, chainId *big.Int) *EcdsaConfig {
	if _, err := os.Stat(ecdsaConfigFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup ecdsa config file does not exist")
	}

	var ecdsaConfigFromYaml EcdsaConfigFromYaml
	err := sdkutils.ReadYamlConfig(ecdsaConfigFilePath, &ecdsaConfigFromYaml)
	if err != nil {
		log.Fatal("Error reading ecdsa config: ", err)
	}

	if ecdsaConfigFromYaml.Ecdsa.PrivateKeyStorePath == "" {
		log.Fatal("Ecdsa private key store path is empty")
	}

	ecdsaKeyPair, err := ecdsa2.ReadKey(ecdsaConfigFromYaml.Ecdsa.PrivateKeyStorePath, ecdsaConfigFromYaml.Ecdsa.PrivateKeyStorePassword)
	if err != nil {
		log.Fatal("Error reading ecdsa private key from file: ", err)
	}

	privateKeySigner, err := signer.NewPrivateKeySigner(ecdsaKeyPair, chainId)
	if err != nil {
		log.Fatal("Error creating private key signer: ", err)
	}

	return &EcdsaConfig{
		PrivateKey: ecdsaKeyPair,
		Signer:     privateKeySigner,
	}
}

func newBlsConfig(blsConfigFilePath string) *BlsConfig {
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

func newAlignedLayerDeploymentConfig(alignedLayerDeploymentFilePath string) *AlignedLayerDeploymentConfig {

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

func newEigenLayerDeploymentConfig(eigenLayerDeploymentFilePath string) *EigenLayerDeploymentConfig {

	if _, err := os.Stat(eigenLayerDeploymentFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup eigen layer deployment file does not exist")
	}

	var eigenLayerDeploymentConfigFromJson EigenLayerDeploymentConfigFromJson
	err := sdkutils.ReadJsonConfig(eigenLayerDeploymentFilePath, &eigenLayerDeploymentConfigFromJson)

	if err != nil {
		log.Fatal("Error reading eigen layer deployment config: ", err)
	}

	if eigenLayerDeploymentConfigFromJson.Addresses.DelegationManagerAddr == common.HexToAddress("") {
		log.Fatal("Delegation manager address is empty")
	}

	if eigenLayerDeploymentConfigFromJson.Addresses.AVSDirectoryAddr == common.HexToAddress("") {
		log.Fatal("AVS directory address is empty")
	}

	if eigenLayerDeploymentConfigFromJson.Addresses.SlasherAddr == common.HexToAddress("") {
		log.Fatal("Slasher address is empty")
	}

	return &EigenLayerDeploymentConfig{
		DelegationManagerAddr: eigenLayerDeploymentConfigFromJson.Addresses.DelegationManagerAddr,
		AVSDirectoryAddr:      eigenLayerDeploymentConfigFromJson.Addresses.AVSDirectoryAddr,
		SlasherAddr:           eigenLayerDeploymentConfigFromJson.Addresses.SlasherAddr,
	}
}
