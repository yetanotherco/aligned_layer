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
	connection "github.com/yetanotherco/aligned_layer/core"
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
	percentageBump := new(big.Int).Div(big.NewInt(int64(iteration)), big.NewInt(10))
	bumpAmount := new(big.Int).Mul(currentGasPrice, percentageBump)
	bumpedGasPrice := new(big.Int).Add(currentGasPrice, bumpAmount)

	return bumpedGasPrice
}

// Sends a transaction and waits for the receipt for three blocks, if not received
// it will try again bumping the gas price based on `CalculateGasPriceBumpBasedOnRetry`
// and pass it to executeTransaction (make sure you update the txOpts with the new gasPrice)
// This process happens indefinitely until we get the receipt or the receipt status is an err.
func SendTransactionWithInfiniteRetryAndBumpingGasPrice(executeTransaction func(*big.Int) (*types.Transaction, error), client eth.InstrumentedClient, baseGasPrice *big.Int) (*types.Receipt, error) {
	i := 0
	sendTransaction := func() (*types.Receipt, error) {
		i++
		gasPrice := CalculateGasPriceBumpBasedOnRetry(baseGasPrice, i)

		tx, err := executeTransaction(gasPrice)
		if err != nil {
			return nil, err
		}

		ctx, cancel := context.WithTimeout(context.Background(), time.Second*36)
		defer cancel()
		receipt, err := WaitForTransactionReceipt(client, ctx, tx.Hash())

		if receipt != nil {
			return receipt, nil
		}
		// if we are here, this means we have reached the timeout (after three blocks it hasn't been included)
		// so we try again by bumping the fee to make sure its included
		if err != nil {
			return nil, err
		}
		return nil, fmt.Errorf("transaction failed")

	}
	return connection.RetryWithData(sendTransaction, 1000, 2, 0)
}
