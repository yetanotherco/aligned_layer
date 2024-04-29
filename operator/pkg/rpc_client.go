package operator

import (
	"net/rpc"

	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/yetanotherco/aligned_layer/core/types"
)

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
	}, nil
}

func (c *AggregatorRpcClient) SendSignedTaskResponseToAggregator(signedTaskResponse *types.SignedTaskResponse) {
	var reply bool
	err := c.rpcClient.Call("Aggregator.SubmitTaskResponse", signedTaskResponse, &reply)
	if err != nil {
		c.logger.Fatal("Received error from aggregator", "err", err)
	} else {
		c.logger.Info("Signed task response header accepted by aggregator.", "reply", reply)
	}
}
