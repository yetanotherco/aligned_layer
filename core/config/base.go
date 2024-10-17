package config

import (
	"context"
	"errors"
	"log"
	"math/big"
	"os"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	rpccalls "github.com/Layr-Labs/eigensdk-go/metrics/collectors/rpc_calls"
	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/urfave/cli/v2"
)

var (
	ConfigFileFlag = &cli.StringFlag{
		Name:     "config",
		Required: true,
		Usage:    "Load base configurations from `FILE`",
	}
)

type BaseConfig struct {
	AlignedLayerDeploymentConfig *AlignedLayerDeploymentConfig
	EigenLayerDeploymentConfig   *EigenLayerDeploymentConfig
	Logger                       sdklogging.Logger
	EthRpcUrl                    string
	EthWsUrl                     string
	EthRpcClient                 eth.InstrumentedClient
	EthRpcClientFallback         eth.InstrumentedClient
	EthWsClient                  eth.InstrumentedClient
	EthWsClientFallback          eth.InstrumentedClient
	EthRpcUrlFallback            string
	EthWsUrlFallback             string
	EigenMetricsIpPortAddress    string
	ChainId                      *big.Int
}

type BaseConfigFromYaml struct {
	AlignedLayerDeploymentConfigFilePath string              `yaml:"aligned_layer_deployment_config_file_path"`
	EigenLayerDeploymentConfigFilePath   string              `yaml:"eigen_layer_deployment_config_file_path"`
	Environment                          sdklogging.LogLevel `yaml:"environment"`
	EthRpcUrl                            string              `yaml:"eth_rpc_url"`
	EthRpcUrlFallback                    string              `yaml:"eth_rpc_url_fallback"`
	EthWsUrl                             string              `yaml:"eth_ws_url"`
	EthWsUrlFallback                     string              `yaml:"eth_ws_url_fallback"`
	EigenMetricsIpPortAddress            string              `yaml:"eigen_metrics_ip_port_address"`
}

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

	alignedLayerDeploymentConfig := NewAlignedLayerDeploymentConfig(alignedLayerDeploymentConfigFilePath)
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
	eigenLayerDeploymentConfig := NewEigenLayerDeploymentConfig(baseConfigFromYaml.EigenLayerDeploymentConfigFilePath)

	if eigenLayerDeploymentConfig == nil {
		log.Fatal("Error reading eigen layer deployment config: ", err)
	}
	logger, err := NewLogger(baseConfigFromYaml.Environment)

	if err != nil {
		log.Fatal("Error initializing logger: ", err)
	}

	if baseConfigFromYaml.EthWsUrl == "" || baseConfigFromYaml.EthWsUrlFallback == "" {
		log.Fatal("Eth ws url or fallback is empty")
	}

	reg := prometheus.NewRegistry()
	rpcCallsCollector := rpccalls.NewCollector("ethWs", reg)
	ethWsClient, err := eth.NewInstrumentedClient(baseConfigFromYaml.EthWsUrl, rpcCallsCollector)
	if err != nil {
		log.Fatal("Error initializing eth ws client: ", err)
	}
	reg = prometheus.NewRegistry()
	rpcCallsCollector = rpccalls.NewCollector("ethWsFallback", reg)
	ethWsClientFallback, err := eth.NewInstrumentedClient(baseConfigFromYaml.EthWsUrlFallback, rpcCallsCollector)
	if err != nil {
		log.Fatal("Error initializing eth ws client fallback: ", err)
	}

	if baseConfigFromYaml.EthRpcUrl == "" || baseConfigFromYaml.EthRpcUrlFallback == "" {
		log.Fatal("Eth rpc url is empty")
	}

	reg = prometheus.NewRegistry()
	rpcCallsCollector = rpccalls.NewCollector("ethRpc", reg)
	ethRpcClient, err := eth.NewInstrumentedClient(baseConfigFromYaml.EthRpcUrl, rpcCallsCollector)
	if err != nil {
		log.Fatal("Error initializing eth rpc client: ", err)
	}

	reg = prometheus.NewRegistry()
	rpcCallsCollector = rpccalls.NewCollector("ethRpc", reg)
	ethRpcClientFallback, err := eth.NewInstrumentedClient(baseConfigFromYaml.EthRpcUrlFallback, rpcCallsCollector)
	if err != nil {
		log.Fatal("Error initializing eth rpc client fallback: ", err)
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
		EthRpcClient:                 *ethRpcClient,
		EthRpcClientFallback:         *ethRpcClientFallback,
		EthWsClient:                  *ethWsClient,
		EthWsClientFallback:          *ethWsClientFallback,
		EthRpcUrlFallback:            baseConfigFromYaml.EthRpcUrlFallback,
		EthWsUrlFallback:             baseConfigFromYaml.EthWsUrlFallback,
		EigenMetricsIpPortAddress:    baseConfigFromYaml.EigenMetricsIpPortAddress,
		ChainId:                      chainId,
	}
}
