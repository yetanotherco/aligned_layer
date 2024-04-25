package config

import (
	"context"
	"crypto/ecdsa"
	"errors"
	ecdsa2 "github.com/Layr-Labs/eigensdk-go/crypto/ecdsa"
	"github.com/ethereum/go-ethereum/common"
	"github.com/urfave/cli"
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
	EcdsaPrivateKey              *ecdsa.PrivateKey
	BlsPrivateKey                *bls.PrivateKey
	EigenMetricsIpPortAddress    string
	ChainId                      *big.Int
	Signer                       signer.Signer
}

type BaseConfigFromYaml struct {
	AlignedLayerDeploymentConfigFilePath string              `yaml:"aligned_layer_deployment_config_file_path"`
	EigenLayerDeploymentConfigFilePath   string              `yaml:"eigen_layer_deployment_config_file_path"`
	Environment                          sdklogging.LogLevel `yaml:"environment"`
	EthRpcUrl                            string              `yaml:"eth_rpc_url"`
	EthWsUrl                             string              `yaml:"eth_ws_url"`
	EcdsaPrivateKeyStorePath             string              `yaml:"ecdsa_private_key_store_path"`
	EcdsaPrivateKeyStorePassword         string              `yaml:"ecdsa_private_key_store_password"`
	BlsPrivateKeyStorePath               string              `yaml:"bls_private_key_store_path"`
	BlsPrivateKeyStorePassword           string              `yaml:"bls_private_key_store_password"`
	EigenMetricsIpPortAddress            string              `yaml:"eigen_metrics_ip_port_address"`
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
	BaseConfig *BaseConfig
	Aggregator struct {
		AggregatorServerIpPortAddress string         `yaml:"aggregator_server_ip_port_address"`
		BlsPublicKeyCompendiumAddress common.Address `yaml:"bls_public_key_compendium_address"`
		AvsServiceManagerAddress      common.Address `yaml:"avs_service_manager_address"`
		EnableMetrics                 bool           `yaml:"enable_metrics"`
	} `yaml:"aggregator"`
}

type OperatorConfig struct {
	BaseConfig *BaseConfig
	Operator   struct {
		Address                   common.Address `yaml:"address"`
		EarningsReceiverAddress   common.Address `yaml:"earnings_receiver_address"`
		DelegationApproverAddress common.Address `yaml:"delegation_approver_address"`
		StakerOptOutWindowBlocks  int            `yaml:"staker_opt_out_window_blocks"`
		MetadataUrl               string         `yaml:"metadata_url"`
		RegisterOperatorOnStartup bool           `yaml:"register_operator_on_startup"`
	} `yaml:"operator"`
}

var (
	// Required Flags
	BaseConfigFileFlag = cli.StringFlag{
		Name:     "base-config-file",
		Required: true,
		Usage:    "Load base configurations from `FILE`",
	}
	AggregatorConfigFileFlag = cli.StringFlag{
		Name:     "aggregator-config-file",
		Required: true,
		Usage:    "Load aggregator configurations from `FILE`",
	}
	OperatorConfigFileFlag = cli.StringFlag{
		Name:     "operator-config-file",
		Required: true,
		Usage:    "Load operator configurations from `FILE`",
	}
	// Optional Flags
)

func NewAggregatorConfig(baseConfigFilePath, aggregatorConfigFilePath string) (*AggregatorConfig, error) {

	if _, err := os.Stat(baseConfigFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup base config file does not exist")
	}

	if _, err := os.Stat(aggregatorConfigFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup aggregator config file does not exist")
	}

	baseConfig, err := newBaseConfig(baseConfigFilePath)

	if err != nil {
		log.Fatal("Error reading base config: ", err)
	}

	var aggregatorConfigFromYaml AggregatorConfig
	err = sdkutils.ReadYamlConfig(aggregatorConfigFilePath, &aggregatorConfigFromYaml)

	if err != nil {
		log.Fatal("Error reading aggregator config: ", err)
	}

	return &AggregatorConfig{
		BaseConfig: baseConfig,
		Aggregator: aggregatorConfigFromYaml.Aggregator,
	}, nil

}

func NewOperatorConfig(baseConfigFilePath, operatorConfigFilePath string) (*OperatorConfig, error) {

	if _, err := os.Stat(baseConfigFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup base config file does not exist")
	}

	if _, err := os.Stat(operatorConfigFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup operator config file does not exist")
	}

	baseConfig, err := newBaseConfig(baseConfigFilePath)

	if err != nil {
		log.Fatal("Error reading base config: ", err)
	}

	var operatorConfigFromYaml OperatorConfig
	err = sdkutils.ReadYamlConfig(operatorConfigFilePath, &operatorConfigFromYaml)

	if err != nil {
		log.Fatal("Error reading operator config: ", err)
	}

	return &OperatorConfig{
		BaseConfig: baseConfig,
		Operator:   operatorConfigFromYaml.Operator,
	}, nil

}

func newBaseConfig(baseConfigFilePath string) (*BaseConfig, error) {

	if _, err := os.Stat(baseConfigFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup base config file does not exist")
	}

	var baseConfigFromYaml BaseConfigFromYaml

	err := sdkutils.ReadYamlConfig(baseConfigFilePath, &baseConfigFromYaml)

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

	alignedLayerDeploymentConfig, err := newAlignedLayerDeploymentConfig(alignedLayerDeploymentConfigFilePath)

	if err != nil {
		log.Fatal("Error reading aligned layer deployment config: ", err)
	}

	eigenLayerDeploymentConfigFilePath := baseConfigFromYaml.EigenLayerDeploymentConfigFilePath

	if eigenLayerDeploymentConfigFilePath == "" {
		log.Fatal("Eigen layer deployment config file path is empty")
	}

	if _, err := os.Stat(eigenLayerDeploymentConfigFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup eigen layer deployment file does not exist")
	}

	eigenLayerDeploymentConfig, err := newEigenLayerDeploymentConfig(baseConfigFromYaml.EigenLayerDeploymentConfigFilePath)

	if err != nil {
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

	if baseConfigFromYaml.BlsPrivateKeyStorePath == "" {
		log.Fatal("Bls private key store path is empty")
	}

	blsKeyPair, err := bls.ReadPrivateKeyFromFile(baseConfigFromYaml.BlsPrivateKeyStorePath, baseConfigFromYaml.BlsPrivateKeyStorePassword)

	if err != nil {
		log.Fatal("Error reading ecdsa private key from file: ", err)
	}

	if baseConfigFromYaml.EcdsaPrivateKeyStorePath == "" {
		log.Fatal("Ecdsa private key store path is empty")
	}

	ecdsaPrivateKey, err := ecdsa2.ReadKey(baseConfigFromYaml.EcdsaPrivateKeyStorePath, baseConfigFromYaml.EcdsaPrivateKeyStorePassword)

	if err != nil {
		log.Fatal("Error reading ecdsa private key from file: ", err)
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
		return nil, err
	}

	privateKeySigner, err := signer.NewPrivateKeySigner(ecdsaPrivateKey, chainId)

	if err != nil {
		logger.Error("Cannot create private key signer from ecdsa private key and chain id", "err", err)
		return nil, err
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
		EcdsaPrivateKey:              ecdsaPrivateKey,
		BlsPrivateKey:                blsKeyPair.PrivKey,
		EigenMetricsIpPortAddress:    baseConfigFromYaml.EigenMetricsIpPortAddress,
		ChainId:                      chainId,
		Signer:                       privateKeySigner,
	}, nil
}

func newAlignedLayerDeploymentConfig(alignedLayerDeploymentFilePath string) (*AlignedLayerDeploymentConfig, error) {

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
	}, nil
}

func newEigenLayerDeploymentConfig(eigenLayerDeploymentFilePath string) (*EigenLayerDeploymentConfig, error) {

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
	}, nil
}

var requiredFlags = []cli.Flag{
	BaseConfigFileFlag,
	AggregatorConfigFileFlag,
	OperatorConfigFileFlag,
}

var optionalFlags []cli.Flag

// Flags contains the list of configuration options available to the binary.
var Flags []cli.Flag

func init() {
	Flags = append(requiredFlags, optionalFlags...)
}
