package utils

import (
	"context"
	"time"

	"fmt"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	gethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	connection "github.com/yetanotherco/aligned_layer/core"
)

const maxRetries = 25
const sleepTime = 5 * time.Second

func WaitForTransactionReceipt(client eth.InstrumentedClient, ctx context.Context, txHash gethcommon.Hash) (*types.Receipt, error) {
	for i := 0; i < maxRetries; i++ {
		receipt, err := client.TransactionReceipt(ctx, txHash)
		if err != nil {
			time.Sleep(sleepTime)
		} else {
			return receipt, nil
		}
	}
	return nil, fmt.Errorf("transaction receipt not found for txHash: %s", txHash.String())
}

func WaitForTransactionReceiptRetryable(client eth.InstrumentedClient, ctx context.Context, txHash gethcommon.Hash) (*types.Receipt, error) {
	receipt_func := func() (*types.Receipt, error) { return client.TransactionReceipt(ctx, txHash) }
	//TODO: make these constants some value.
	receipt, err := connection.RetryWithData(receipt_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
	if err != nil {
		// If the retries stop early we propogate the Permanent error to the caller.
		// TODO: ask if this is the behavior we want.
		return nil, err
	}
	return receipt, nil
}

func BytesToQuorumNumbers(quorumNumbersBytes []byte) eigentypes.QuorumNums {
	quorumNums := make(eigentypes.QuorumNums, len(quorumNumbersBytes))
	for i, quorumNumberByte := range quorumNumbersBytes {
		quorumNums[i] = eigentypes.QuorumNum(quorumNumberByte)
	}
	return quorumNums
}

func BytesToQuorumThresholdPercentages(quorumThresholdPercentagesBytes []byte) eigentypes.QuorumThresholdPercentages {
	quorumThresholdPercentages := make(eigentypes.QuorumThresholdPercentages, len(quorumThresholdPercentagesBytes))
	for i, quorumNumberByte := range quorumThresholdPercentagesBytes {
		quorumThresholdPercentages[i] = eigentypes.QuorumThresholdPercentage(quorumNumberByte)
	}
	return quorumThresholdPercentages
}
