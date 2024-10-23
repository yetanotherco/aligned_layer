package utils

import (
	"context"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	gethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	connection "github.com/yetanotherco/aligned_layer/core"
)

// Does not
func WaitForTransactionReceiptRetryable(client eth.InstrumentedClient, ctx context.Context, txHash gethcommon.Hash) (*types.Receipt, error) {
	// For if no receipt and no error TransactionReceipt return "not found" as an error catch all ref: https://github.com/ethereum/go-ethereum/blob/master/ethclient/ethclient.go#L313
	receipt_func := func() (*types.Receipt, error) { return client.TransactionReceipt(ctx, txHash) }
	return connection.RetryWithData(receipt_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
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
