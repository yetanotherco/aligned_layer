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

// Very basic algorithm to calculate the gasPrice bump based on the currentGasPrice a constant percentage and the retry number.
// It adds a the percentage to the current gas price and a 5% * i, where i is the iteration number. That is:
func CalculateGasPriceBumpBasedOnRetry(currentGasPrice *big.Int, percentage int, i int) *big.Int {
	retryPercentage := new(big.Int).Mul(big.NewInt(5), big.NewInt(int64(i)))
	percentageBump := new(big.Int).Add(big.NewInt(int64(percentage)), retryPercentage)
	bumpAmount := new(big.Int).Mul(currentGasPrice, percentageBump)
	bumpAmount = new(big.Int).Div(bumpAmount, big.NewInt(100))
	bumpedGasPrice := new(big.Int).Add(currentGasPrice, bumpAmount)

	return bumpedGasPrice
}
