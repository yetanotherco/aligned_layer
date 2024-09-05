package operator

import (
	"errors"
	"net/rpc"
	"time"

	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/yetanotherco/aligned_layer/core/types"
)

// AggregatorRpcClient is the client to communicate with the aggregator via RPC
type AggregatorRpcClient struct {
	rpcClient            *rpc.Client
	aggregatorIpPortAddr string
	logger               logging.Logger
}

const (
	MaxRetries    = 10
	RetryInterval = 10 * time.Second
)

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
	for retries := 0; retries < MaxRetries; retries++ {
		err := c.rpcClient.Call("Aggregator.ProcessOperatorSignedTaskResponseV2", signedTaskResponse, &reply)
		if err != nil {
			c.logger.Error("Received error from aggregator", "err", err)
			if errors.Is(err, rpc.ErrShutdown) {
				c.logger.Error("Aggregator is shutdown. Reconnecting...")
				client, err := rpc.DialHTTP("tcp", c.aggregatorIpPortAddr)
				if err != nil {
					c.logger.Error("Could not reconnect to aggregator", "err", err)
					time.Sleep(RetryInterval)
				} else {
					c.rpcClient = client
					c.logger.Info("Reconnected to aggregator")
				}
			} else {
				c.logger.Infof("Received error from aggregator: %s. Retrying ProcessOperatorSignedTaskResponseV2 RPC call...", err)
				time.Sleep(RetryInterval)
			}
		} else {
			c.logger.Info("Signed task response header accepted by aggregator.", "reply", reply)
			return
		}
	}
}
