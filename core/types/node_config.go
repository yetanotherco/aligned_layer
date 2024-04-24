package types

import (
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/ethereum/go-ethereum/common"
)

type NodeConfig struct {
	EthRpcUrl                      string
	EthHttpClient                  eth.Client
	EthWsClient                    eth.Client
	BlsOperatorStateRetrieverAddr  common.Address
	AlignedLayerServiceManagerAddr common.Address
	Logger                         sdklogging.Logger
}
