package utils

import (
	"context"
	"fmt"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	gethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"time"
)

const maxRetries = 10

func WaitForTransactionReceipt(client eth.Client, ctx context.Context, txHash gethcommon.Hash) (*types.Receipt, error) {
	for i := 0; i < maxRetries; i++ {
		receipt, err := client.TransactionReceipt(ctx, txHash)
		if err != nil {
			time.Sleep(2 * time.Second)
		} else {
			return receipt, nil
		}
	}
	return nil, fmt.Errorf("transaction receipt not found for txHash: %s", txHash.String())
}
