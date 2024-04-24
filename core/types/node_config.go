package types

type NodeConfig struct {
	EthRpcUrl                      string `yaml:"eth_rpc_url"`
	EthWsUrl                       string `yaml:"eth_ws_url"`
	OperatorStateRetrieverAddr     string `yaml:"operator_state_retriever_address"`
	AlignedLayerServiceManagerAddr string `yaml:"avs_service_manager_address"`
}
