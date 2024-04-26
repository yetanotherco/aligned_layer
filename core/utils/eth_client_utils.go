package utils

import (
	"context"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	gethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"time"
)

func WaitForTransactionReceipt(client eth.Client, ctx context.Context, txHash gethcommon.Hash) *types.Receipt {
	for {
		receipt, err := client.TransactionReceipt(ctx, txHash)
		if err != nil {
			time.Sleep(2 * time.Second)
		} else {
			return receipt
		}
	}
}
