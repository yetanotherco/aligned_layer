package config

import (
	"context"
	"crypto/ecdsa"
	"errors"
	"fmt"
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

// Config contains all the configuration information for a credible squaring aggregators and challengers.
// Operators use a separate config. (see config-files/operator.anvil.yaml)
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
type ConfigRaw struct {
	Environment                sdklogging.LogLevel `yaml:"environment"`
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

// BlsOperatorStateRetriever and BlsPublicKeyCompendium are deployed separately, since they are
// shared avs contracts (not part of eigenlayer core contracts).
// the blspubkeycompendium we can get from serviceManager->registryCoordinator->blsregistry->blspubkeycompendium
// so we don't need it here. The blsOperatorStateRetriever however is an independent contract not pointing to
// or pointed to from any other contract, so we need its address
//type SharedAvsContractsRaw struct {
//	BlsOperatorStateRetrieverAddr string `json:"blsOperatorStateRetriever"`
//}

// NewConfig parses config file to read from flags or environment variables
// Note: This config is shared by challenger and aggregator, so we put in the core.
// Operator has a different config and is meant to be used by the operator CLI.
func NewConfig(ctx *cli.Context) (*Config, error) {

	var configRaw ConfigRaw

	configFilePath := ctx.GlobalString(ConfigFileFlag.Name)
	if configFilePath != "" {
		err := sdkutils.ReadYamlConfig(configFilePath, &configRaw)
		if err != nil {
			fmt.Println("Could not read yaml config file")
			return nil, err
		}
	}

	logger, err := sdklogging.NewZapLogger(configRaw.Environment)
	if err != nil {
		fmt.Println("Could not initialize logger")
	}

	var alignedLayerDeploymentRaw AlignedLayerDeploymentRaw
	alignedLayerDeploymentFilePath := ctx.GlobalString(AlignedLayerDeploymentFileFlag.Name)
	if _, err := os.Stat(alignedLayerDeploymentFilePath); errors.Is(err, os.ErrNotExist) {
		panic("Path " + alignedLayerDeploymentFilePath + " does not exist")
	}
	err = sdkutils.ReadJsonConfig(alignedLayerDeploymentFilePath, &alignedLayerDeploymentRaw)
	if err != nil {
		logger.Errorf("Cannot read aligned layer deployment file", "err", err)
		return nil, err
	}

	// Commented out because we don't need this for now, we are getting the OperatorStateRetrieverAddr
	// from the alignedLayerDeploymentRaw

	//var sharedAvsContractsDeploymentRaw SharedAvsContractsRaw
	//sharedAvsContractsDeploymentFilePath := ctx.GlobalString(SharedAvsContractsDeploymentFileFlag.Name)
	//if _, err := os.Stat(sharedAvsContractsDeploymentFilePath); errors.Is(err, os.ErrNotExist) {
	//	panic("Path " + sharedAvsContractsDeploymentFilePath + " does not exist")
	//}
	//err = sdkutils.ReadJsonConfig(sharedAvsContractsDeploymentFilePath, &sharedAvsContractsDeploymentRaw)
	//if err != nil {
	//	logger.Errorf("Cannot read shared avs contracts deployment file", "err", err)
	//	return nil, err
	//}

	ethRpcClient, err := eth.NewClient(configRaw.EthRpcUrl)
	if err != nil {
		logger.Errorf("Cannot create http ethclient", "err", err)
		return nil, err
	}

	ethWsClient, err := eth.NewClient(configRaw.EthWsUrl)
	if err != nil {
		logger.Errorf("Cannot create ws ethclient", "err", err)
		return nil, err
	}

	ecdsaPrivateKeyString := ctx.GlobalString(EcdsaPrivateKeyFlag.Name)
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
		logger.Error("Cannot get operator address", "err", err)
		return nil, err
	}

	chainId, err := ethRpcClient.ChainID(context.Background())
	if err != nil {
		logger.Error("Cannot get chainId", "err", err)
		return nil, err
	}

	privateKeySigner, err := signer.NewPrivateKeySigner(ecdsaPrivateKey, chainId)
	if err != nil {
		logger.Error("Cannot create signer", "err", err)
		return nil, err
	}

	config := &Config{
		EcdsaPrivateKey: ecdsaPrivateKey,
		//BlsPrivateKey: 						blsPrivateKey
		Logger:                                 logger,
		EigenMetricsIpPortAddress:              configRaw.AggregatorServerIpPortAddr,
		EthRpcUrl:                              configRaw.EthRpcUrl,
		EthWsUrl:                               configRaw.EthWsUrl,
		EthHttpClient:                          ethRpcClient,
		EthWsClient:                            ethWsClient,
		AlignedLayerOperatorStateRetrieverAddr: common.HexToAddress(alignedLayerDeploymentRaw.Addresses.AlignedLayerOperatorStateRetrieverAddr),
		AlignedLayerServiceManagerAddr:         common.HexToAddress(alignedLayerDeploymentRaw.Addresses.AlignedLayerServiceManagerAddr),
		AlignedLayerRegistryCoordinatorAddr:    common.HexToAddress(alignedLayerDeploymentRaw.Addresses.AlignedLayerRegistryCoordinatorAddr),
		ChainId:                                chainId,
		BlsPublicKeyCompendiumAddress:          common.HexToAddress(configRaw.BLSPubkeyCompendiumAddr),
		SlasherAddr:                            common.HexToAddress(""),
		AggregatorServerIpPortAddr:             configRaw.AggregatorServerIpPortAddr,
		RegisterOperatorOnStartup:              configRaw.RegisterOperatorOnStartup,
		Signer:                                 privateKeySigner,
		OperatorAddress:                        operatorAddr,
		AVSServiceManagerAddress:               common.HexToAddress(configRaw.AvsServiceManagerAddress),
		EnableMetrics:                          configRaw.EnableMetrics,
	}
	config.Validate()
	return config, nil
}

func (c *Config) Validate() {
	// TODO: make sure every pointer is non-nil
	if c.EcdsaPrivateKey == nil {
		panic("Config: EcdsaPrivateKey is required")
	}

	if c.AlignedLayerOperatorStateRetrieverAddr == common.HexToAddress("") {
		panic("Config: AlignedLayerOperatorStateRetrieverAddr is required")
	}
	if c.AlignedLayerServiceManagerAddr == common.HexToAddress("") {
		panic("Config: AlignedLayerServiceManagerAddr is required")
	}
}

var (
	// Required Flags
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
	//SharedAvsContractsDeploymentFileFlag = cli.StringFlag{
	//	Name:     "shared-avs-contracts-deployment",
	//	Required: true,
	//	Usage:    "Load shared avs contract addresses from `FILE`",
	//}
	EcdsaPrivateKeyFlag = cli.StringFlag{
		Name:     "ecdsa-private-key",
		Usage:    "Ethereum private key",
		Required: true,
		EnvVar:   "ECDSA_PRIVATE_KEY",
	}
	// Optional Flags
)

var requiredFlags = []cli.Flag{
	ConfigFileFlag,
	AlignedLayerDeploymentFileFlag,
	//SharedAvsContractsDeploymentFileFlag,
	EcdsaPrivateKeyFlag,
}

var optionalFlags []cli.Flag

// Flags contains the list of configuration options available to the binary.
var Flags []cli.Flag

func init() {
	Flags = append(requiredFlags, optionalFlags...)
}
