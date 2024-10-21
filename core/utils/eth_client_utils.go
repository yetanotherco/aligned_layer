package utils

import (
	"context"
	"math/big"
	"time"

	"fmt"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	gethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
)

const maxRetries = 25
const sleepTime = 5 * time.Second

func WaitForTransactionReceipt(client eth.InstrumentedClient, ctx context.Context, txHash gethcommon.Hash) (*types.Receipt, error) {
	for i := 0; i < maxRetries; i++ {
		receipt, err := client.TransactionReceipt(ctx, txHash)
		// if context has timed out, return
		if ctx.Err() != nil {
			return nil, ctx.Err()
		}
		if err != nil {
			time.Sleep(sleepTime)
		} else {
			return receipt, nil
		}
	}
	return nil, fmt.Errorf("transaction receipt not found for txHash: %s", txHash.String())
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

// Very basic algorithm to calculate the gasPrice bump based on the currentGasPrice and retry iteration.
// It adds a i/10 percentage to the current prices, where i represents the iteration.
func CalculateGasPriceBumpBasedOnRetry(currentGasPrice *big.Int, iteration int) *big.Int {
	factor := (new(big.Int).Add(big.NewInt(100), new(big.Int).Mul(big.NewInt(int64(iteration)), big.NewInt(10))))
	gasPrice := new(big.Int).Mul(currentGasPrice, factor)
	gasPrice = gasPrice.Div(gasPrice, big.NewInt(100))

	return gasPrice
}
