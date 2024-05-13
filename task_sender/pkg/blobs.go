package pkg

import (
	"bytes"
	"context"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/ethereum/go-ethereum/consensus/misc/eip4844"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/crypto/kzg4844"
	"github.com/ethereum/go-ethereum/rlp"
	"github.com/holiman/uint256"
	"github.com/yetanotherco/aligned_layer/common"
	serviceManager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/utils"
	"io"
	"log"
	"math/big"
	"net/http"
	"strconv"
	"time"
)

// MaxBlobSize 128 KB
const MaxBlobSize = 128 * 1024

func (ts *TaskSender) PostProofOnBlobs(proof []byte) (*serviceManager.AlignedLayerServiceManagerDAPayload, error) {
	b := new(bytes.Buffer)
	w := io.Writer(b)

	// Encode the proof using RLP encoding
	// This is needed because blobs are a fixed size, so we will need to remove trailing zeros
	err := rlp.Encode(w, proof)
	if err != nil {
		return nil, err
	}

	encodedProof := make([]byte, hex.EncodedLen(b.Len()))
	hex.Encode(encodedProof, b.Bytes())

	chunks := SplitIntoChunks(encodedProof, MaxBlobSize)

	log.Println("Amount of chunks: ", len(chunks))
	log.Println("Posting proof on Blobs...")

	blobs := make([]kzg4844.Blob, len(chunks))
	commitments := make([]kzg4844.Commitment, len(chunks))
	proofs := make([]kzg4844.Proof, len(chunks))

	for idx, chunk := range chunks {
		var Blob kzg4844.Blob
		copy(Blob[:], chunk)

		// Compute the commitment for the blob data using KZG4844 cryptographic algorithm
		BlobCommitment, err := kzg4844.BlobToCommitment(Blob)
		if err != nil {
			return nil, fmt.Errorf("failed to compute blob commitment: %s", err)
		}

		// Compute the proof for the blob data, which will be used to verify the transaction
		BlobProof, err := kzg4844.ComputeBlobProof(Blob, BlobCommitment)
		if err != nil {
			return nil, fmt.Errorf("failed to compute blob proof: %s", err)
		}

		blobs[idx] = Blob
		commitments[idx] = BlobCommitment
		proofs[idx] = BlobProof

	}

	// Prepare the sidecar data for the transaction, which includes the blob and its cryptographic proof
	sidecar := types.BlobTxSidecar{
		Blobs:       blobs,
		Commitments: commitments,
		Proofs:      proofs,
	}

	// Compute the sender's address from the public key
	fromAddress := crypto.PubkeyToAddress(ts.EcdsaPrivKey.PublicKey)

	// Retrieve the nonce for the transaction
	nonce, err := ts.blobsConfig.EthRpcClient.PendingNonceAt(context.Background(), fromAddress)
	if err != nil {
		return nil, fmt.Errorf("failed to get nonce: %s", err)
	}

	lastBlock, err := ts.blobsConfig.EthRpcClient.BlockByNumber(context.Background(), nil)
	if err != nil {
		return nil, fmt.Errorf("failed to get last block: %s", err)
	}

	excessBlobGas := lastBlock.ExcessBlobGas()
	calcBlobFee := eip4844.CalcBlobFee(*excessBlobGas)

	blobFeeCap := uint256.MustFromBig(calcBlobFee)

	blobFeeCap.Mul(blobFeeCap, uint256.NewInt(10))

	tip := lastBlock.BaseFee().Mul(lastBlock.BaseFee(), big.NewInt(2))
	maxFeePerGas := lastBlock.BaseFee().Add(lastBlock.BaseFee(), tip)

	blobGasUsed := *lastBlock.BlobGasUsed()
	transactions := lastBlock.Transactions()
	count := 0

	// Calculate the total number of blob txs
	for _, tx := range transactions {
		if tx.BlobHashes() != nil {
			count++
		}
	}

	var gas uint64
	if count == 0 {
		// No blob transactions in the last block, default gas
		gas = 100_000
	} else {
		gas = blobGasUsed / uint64(count)
	}

	chainId, err := ts.blobsConfig.EthRpcClient.ChainID(context.Background())
	if err != nil {
		return nil, err
	}

	tx := types.NewTx(&types.BlobTx{
		ChainID:    uint256.MustFromBig(chainId),
		Nonce:      nonce,
		GasTipCap:  uint256.MustFromBig(tip),
		GasFeeCap:  uint256.MustFromBig(maxFeePerGas),
		Gas:        gas,
		Value:      uint256.NewInt(0),
		Data:       nil,
		BlobFeeCap: blobFeeCap,
		BlobHashes: sidecar.BlobHashes(),
		Sidecar:    &sidecar,
	})

	signedTx, err := types.SignTx(tx, types.LatestSignerForChainID(chainId), ts.EcdsaPrivKey)
	if err != nil {
		return nil, fmt.Errorf("failed to sign transaction: %s", err)
	}

	// Send the signed transaction to the Ethereum network
	err = ts.blobsConfig.EthRpcClient.SendTransaction(context.Background(), signedTx)
	if err != nil {
		return nil, fmt.Errorf("failed to send transaction: %s", err)
	}

	txHash := signedTx.Hash()

	log.Println("Blobs sent. Transaction hash:", txHash.String())
	log.Println("Waiting for transaction receipt...")

	receipt, err := utils.WaitForTransactionReceiptMaxRetries(ts.blobsConfig.EthRpcClient, context.Background(), txHash, 20)
	if err != nil {
		return nil, fmt.Errorf("failed to get transaction receipt: %s", err)
	}

	blockNumber := receipt.BlockNumber
	log.Println("Waiting for the next block to get the beacon root...")

	// Add one because the beacon root of the current block is the parent beacon root of the next block.
	blockNumber.Add(blockNumber, big.NewInt(1))
	block, err := waitForBlock(ts.blobsConfig.EthRpcClient, blockNumber)
	if err != nil {
		return nil, err
	}

	consensusResponse, err := ts.getResponseFromBeaconRoot(block.BeaconRoot().Bytes())
	if err != nil {
		return nil, err
	}

	daChunks := make([]serviceManager.AlignedLayerServiceManagerDAPayloadChunk, len(chunks))
	for idx, blob := range blobs {
		txIdx := -1

		for _, daChunk := range consensusResponse.Data {
			var daBlob kzg4844.Blob

			daChunk.Blob = daChunk.Blob[2:]
			daDecodedBlob, err := hex.DecodeString(daChunk.Blob)

			copy(daBlob[:], daDecodedBlob)

			// TODO: check if this can be optimized
			// e.g compare chunks instead of Blob

			if daBlob == blob {
				log.Println("Found blob in response at index", daChunk.Index)
				txIdx, err = strconv.Atoi(daChunk.Index)
				if err != nil {
					return nil, err
				}
				break
			}
		}

		if txIdx == -1 {
			return nil, fmt.Errorf("could not find blob in response")
		}

		daChunks[idx] = serviceManager.AlignedLayerServiceManagerDAPayloadChunk{
			ProofAssociatedData: block.BeaconRoot().Bytes(),
			Index:               uint64(txIdx),
		}
	}

	return &serviceManager.AlignedLayerServiceManagerDAPayload{
		Solution: common.Blobs,
		Chunks:   daChunks,
	}, nil

}

func waitForBlock(c eth.Client, blockNumber *big.Int) (*types.Block, error) {
	for {
		block, err := c.BlockByNumber(context.Background(), blockNumber)
		if err != nil {
			time.Sleep(1 * time.Second)
		} else {
			return block, nil
		}
	}
}

type BlobResponse struct {
	Data []struct {
		Index string `json:"index"`
		Blob  string `json:"blob"`
	} `json:"data"`
}

func (ts *TaskSender) getResponseFromBeaconRoot(beaconRoot []byte) (*BlobResponse, error) {
	beaconRootStr := hex.EncodeToString(beaconRoot)
	log.Println("Getting response from beacon root:", beaconRootStr)

	resp, err := http.Get(ts.blobsConfig.BeaconChainRpcUrl + "/eth/v1/beacon/blob_sidecars/0x" + beaconRootStr)
	if err != nil {
		return nil, err
	}
	if resp.StatusCode != http.StatusOK {
		return nil, errors.New("could not get response from beacon root, status code: " + resp.Status)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	decodedBody := BlobResponse{}
	err = json.Unmarshal(body, &decodedBody)
	if err != nil {
		return nil, err
	}

	return &decodedBody, nil
}
