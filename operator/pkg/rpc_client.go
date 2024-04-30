package operator

import (
	"net/rpc"

	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/yetanotherco/aligned_layer/core/types"
)

// AggregatorRpcClient is the client to communicate with the aggregator via RPC
type AggregatorRpcClient struct {
	rpcClient            *rpc.Client
	aggregatorIpPortAddr string
	logger               logging.Logger
}

func NewAggregatorRpcClient(aggregatorIpPortAddr string, logger logging.Logger) (*AggregatorRpcClient, error) {
	client, err := rpc.DialHTTP("tcp", aggregatorIpPortAddr)
	if err != nil {
		return nil, err
	}

	return &AggregatorRpcClient{
		rpcClient:            client,
		aggregatorIpPortAddr: aggregatorIpPortAddr,
		logger:               logger,
	}, nil
}

// SendSignedTaskResponseToAggregator is the method called by operators via RPC to send
// their signed task response.
func (c *AggregatorRpcClient) SendSignedTaskResponseToAggregator(signedTaskResponse *types.SignedTaskResponse) {
	var reply uint8
	err := c.rpcClient.Call("Aggregator.ProcessTaskResponse", signedTaskResponse, &reply)
	if err != nil {
		c.logger.Error("Received error from aggregator", "err", err)
	} else {
		c.logger.Info("Signed task response header accepted by aggregator.", "reply", reply)
	}
}
