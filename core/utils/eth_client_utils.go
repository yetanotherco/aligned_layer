package utils

import (
	"context"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"math/big"
	"time"

	"fmt"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	gethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
)

const (
	maxRetries          = 25
	sleepTime           = 5 * time.Second
	incrementPercentage = 10
)

func WaitForTransactionReceipt(client eth.Client, ctx context.Context, txHash gethcommon.Hash) (*types.Receipt, error) {
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

func WaitForTransactionReceiptWithIncreasingTip(w *chainio.AvsWriter, ctx context.Context, txHash gethcommon.Hash, txNonce *big.Int, txOpts *bind.TransactOpts, batchMerkleRoot [32]byte, nonSignerStakesAndSignature servicemanager.IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Receipt, error) {
	currentSleepTime := 0 * time.Second

	for i := 0; i < maxRetries; i++ {
		receipt, err := w.Client.TransactionReceipt(ctx, txHash)

		if err == nil {
			return receipt, nil
		}

		currentSleepTime += sleepTime
		time.Sleep(sleepTime)

		// If one minute elapses, increase the gas limit and gas tip cap
		if currentSleepTime%60*time.Second == 0 {
			// Simulate the transaction to get the gas limit again
			tx, err := w.SimulateRespondToTask(batchMerkleRoot, nonSignerStakesAndSignature)
			if err != nil {
				return nil, err
			}

			// Use the same nonce as the original transaction
			txOpts.Nonce = txNonce

			// Add 10% to the gas limit
			txOpts.GasLimit = tx.Gas() * 110 / 100

			// Increase the gas tip cap by 10%
			var newGasTipCap *big.Int
			newGasTipCap = new(big.Int).Mul(big.NewInt(int64(incrementPercentage+100)), tx.GasTipCap())
			newGasTipCap.Div(newGasTipCap, big.NewInt(100))
			txOpts.GasTipCap = newGasTipCap

			// Submit the transaction with the new gas tip cap
			tx, err = w.SendRespondToTask(batchMerkleRoot, nonSignerStakesAndSignature)
			if err != nil {
				return nil, err
			}

			// Update the transaction hash
			txHash = tx.Hash()
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
