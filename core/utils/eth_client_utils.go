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
func CalculateGasPriceBump(currentGasPrice *big.Int, percentage int) *big.Int {
	percentageBump := new(big.Int).Div(big.NewInt(int64(percentage)), big.NewInt(100))
	bumpAmount := new(big.Int).Mul(currentGasPrice, percentageBump)
	bumpedGasPrice := new(big.Int).Add(currentGasPrice, bumpAmount)

	return bumpedGasPrice
}
