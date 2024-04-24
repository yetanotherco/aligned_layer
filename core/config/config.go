package config

import (
	"context"
	"crypto/ecdsa"
	"errors"
	"fmt"
	"log"
	"math/big"
	"os"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/urfave/cli"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"

	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
)

type Config struct {
	EcdsaPrivateKey           *ecdsa.PrivateKey
	BlsPrivateKey             *bls.PrivateKey
	Logger                    sdklogging.Logger
	EigenMetricsIpPortAddress string
	// we need the url for the eigensdk currently... eventually standardize api so as to
	// only take an ethclient or an rpcUrl (and build the ethclient at each constructor site)
	EthRpcUrl                              string
	EthWsUrl                               string
	EthHttpClient                          eth.Client
	EthWsClient                            eth.Client
	AlignedLayerOperatorStateRetrieverAddr common.Address
	AlignedLayerServiceManagerAddr         common.Address
	AlignedLayerRegistryCoordinatorAddr    common.Address
	ChainId                                *big.Int
	BlsPublicKeyCompendiumAddress          common.Address
	SlasherAddr                            common.Address
	AggregatorServerIpPortAddr             string
	RegisterOperatorOnStartup              bool
	Signer                                 signer.Signer
	OperatorAddress                        common.Address
	AVSServiceManagerAddress               common.Address
	EnableMetrics                          bool
}

// These are read from ConfigFileFlag
type ConfigFromYaml struct {
	Environment                sdklogging.LogLevel `yaml:"environment"`
	EigenMetricsIpPortAddress  string              `yaml:"eigen_metrics_ip_port_address"`
	EthRpcUrl                  string              `yaml:"eth_rpc_url"`
	EthWsUrl                   string              `yaml:"eth_ws_url"`
	AggregatorServerIpPortAddr string              `yaml:"aggregator_server_ip_port_address"`
	RegisterOperatorOnStartup  bool                `yaml:"register_operator_on_startup"`
	BLSPubkeyCompendiumAddr    string              `yaml:"bls_public_key_compendium_address"`
	AvsServiceManagerAddress   string              `yaml:"avs_service_manager_address"`
	EnableMetrics              bool                `yaml:"enable_metrics"`
}

// These are read from AlignedLayerDeploymentFileFlag
type AlignedLayerDeploymentRaw struct {
	Addresses AlignedLayerContractsRaw `json:"addresses"`
}

type AlignedLayerContractsRaw struct {
	AlignedLayerServiceManagerAddr         string `json:"alignedLayerServiceManager"`
	AlignedLayerRegistryCoordinatorAddr    string `json:"registryCoordinator"`
	AlignedLayerOperatorStateRetrieverAddr string `json:"operatorStateRetriever"`
}

type BaseConfig struct {
	Logger       sdklogging.Logger
	EthRpcClient eth.Client
	EthWsClient  eth.Client
}

type BaseConfigFromYaml struct {
	Environment string `yaml:"environment"`
	EthRpcUrl   string `yaml:"eth_rpc_url"`
	EthWsUrl    string `yaml:"eth_ws_url"`
}

var (
	// Required Flags
	SetupConfigFileFlag = cli.StringFlag{
		Name:     "setup-config",
		Required: true,
		Usage:    "Load configuration from `FILE`",
	}
	ConfigFileFlag = cli.StringFlag{
		Name:     "config",
		Required: true,
		Usage:    "Load configuration from `FILE`",
	}
	AlignedLayerDeploymentFileFlag = cli.StringFlag{
		Name:     "aligned-layer-deployment",
		Required: true,
		Usage:    "Load credible squaring contract addresses from `FILE`",
	}
	EcdsaPrivateKeyFlag = cli.StringFlag{
		Name:     "ecdsa-private-key",
		Usage:    "Ethereum private key",
		Required: true,
		EnvVar:   "ECDSA_PRIVATE_KEY",
	}
	// Optional Flags
)

// NewConfig parses config file to read from flags or environment variables
func NewConfig(
	configFilePath string, alignedLayerDeploymentFilePath string, ecdsaPrivateKeyString string,
	logger sdklogging.Logger, ethRpcClient eth.Client, ethWsClient eth.Client) (*Config, error) {

	if configFilePath == "" {
		return nil, errors.New("Config file path is required")
	}

	if alignedLayerDeploymentFilePath == "" {
		return nil, errors.New("Aligned layer deployment file path is required")
	}

	if ecdsaPrivateKeyString == "" {
		return nil, errors.New("ECDSA private key is required")
	}

	if _, err := os.Stat(configFilePath); errors.Is(err, os.ErrNotExist) {
		logger.Errorf("Config file path does not exist", "path", alignedLayerDeploymentFilePath)
		return nil, err
	}

	if _, err := os.Stat(alignedLayerDeploymentFilePath); errors.Is(err, os.ErrNotExist) {
		logger.Errorf("Aligned Layer deployment file path does not exist", "path", alignedLayerDeploymentFilePath)
		return nil, err
	}

	if logger == nil {
		return nil, errors.New("Logger is required")
	}

	if ethRpcClient == nil {
		return nil, errors.New("EthRpcClient is required")
	}

	if ethWsClient == nil {
		return nil, errors.New("EthWsClient is required")
	}

	var configFromYaml ConfigFromYaml

	err := sdkutils.ReadYamlConfig(configFilePath, &configFromYaml)

	if err != nil {
		fmt.Println("Cannot read config file")
		return nil, err
	}

	var alignedLayerDeploymentRaw AlignedLayerDeploymentRaw

	err = sdkutils.ReadJsonConfig(alignedLayerDeploymentFilePath, &alignedLayerDeploymentRaw)

	if err != nil {
		logger.Errorf("Cannot read aligned layer deployment file", "err", err)
		return nil, err
	}

	if ecdsaPrivateKeyString[:2] == "0x" {
		ecdsaPrivateKeyString = ecdsaPrivateKeyString[2:]
	}

	ecdsaPrivateKey, err := crypto.HexToECDSA(ecdsaPrivateKeyString)

	if err != nil {
		logger.Errorf("Cannot parse ecdsa private key", "err", err)
		return nil, err
	}

	operatorAddr, err := sdkutils.EcdsaPrivateKeyToAddress(ecdsaPrivateKey)

	if err != nil {
		logger.Error("Cannot get operator address from ecdsa private key", "err", err)
		return nil, err
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

	config := &Config{
		EcdsaPrivateKey: ecdsaPrivateKey,
		//BlsPrivateKey: 							blsPrivateKey
		Logger:                                 logger,
		EigenMetricsIpPortAddress:              configFromYaml.EigenMetricsIpPortAddress,
		EthRpcUrl:                              configFromYaml.EthRpcUrl,
		EthWsUrl:                               configFromYaml.EthWsUrl,
		EthHttpClient:                          ethRpcClient,
		EthWsClient:                            ethWsClient,
		AlignedLayerOperatorStateRetrieverAddr: common.HexToAddress(alignedLayerDeploymentRaw.Addresses.AlignedLayerOperatorStateRetrieverAddr),
		AlignedLayerServiceManagerAddr:         common.HexToAddress(alignedLayerDeploymentRaw.Addresses.AlignedLayerServiceManagerAddr),
		AlignedLayerRegistryCoordinatorAddr:    common.HexToAddress(alignedLayerDeploymentRaw.Addresses.AlignedLayerRegistryCoordinatorAddr),
		ChainId:                                chainId,
		BlsPublicKeyCompendiumAddress:          common.HexToAddress(configFromYaml.BLSPubkeyCompendiumAddr),
		SlasherAddr:                            common.HexToAddress(""),
		AggregatorServerIpPortAddr:             configFromYaml.AggregatorServerIpPortAddr,
		RegisterOperatorOnStartup:              configFromYaml.RegisterOperatorOnStartup,
		Signer:                                 privateKeySigner,
		OperatorAddress:                        operatorAddr,
		AVSServiceManagerAddress:               common.HexToAddress(configFromYaml.AvsServiceManagerAddress),
		EnableMetrics:                          configFromYaml.EnableMetrics,
	}

	return config, nil
}

func NewBaseConfig(setupConfigFilePath string) (*BaseConfig, error) {
	var baseConfigFromYaml BaseConfigFromYaml
	err := sdkutils.ReadYamlConfig(setupConfigFilePath, &baseConfigFromYaml)
	if err != nil {
		log.Fatal("Error reading setup config: ", err)
	}

	logger, err := NewLogger(baseConfigFromYaml.Environment)

	if err != nil {
		log.Fatal("Error initializing logger: ", err)
	}

	ethRpcClient, err := eth.NewClient(baseConfigFromYaml.EthRpcUrl)

	if err != nil {
		log.Fatal("Error initializing eth rpc client: ", err)
	}

	ethWsClient, err := eth.NewClient(baseConfigFromYaml.EthWsUrl)

	if err != nil {
		log.Fatal("Error initializing eth ws client: ", err)
	}

	return &BaseConfig{
		Logger:       logger,
		EthRpcClient: ethRpcClient,
		EthWsClient:  ethWsClient,
	}, nil
}

var requiredFlags = []cli.Flag{
	SetupConfigFileFlag,
	ConfigFileFlag,
	AlignedLayerDeploymentFileFlag,
	EcdsaPrivateKeyFlag,
}

var optionalFlags []cli.Flag

// Flags contains the list of configuration options available to the binary.
var Flags []cli.Flag

func init() {
	Flags = append(requiredFlags, optionalFlags...)
}
