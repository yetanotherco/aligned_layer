package chainio

import (
	"context"
	"encoding/hex"
	"sync"
	"time"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	ethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/event"

	"github.com/ethereum/go-ethereum/core/types"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	retry "github.com/yetanotherco/aligned_layer/core"
	"github.com/yetanotherco/aligned_layer/core/config"

	"fmt"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/ethereum/go-ethereum/crypto"
)

const (
	MaxRetries                        = 100
	RetryInterval                     = 1 * time.Second
	BlockInterval              uint64 = 1000
	PollLatestBatchInterval           = 5 * time.Second
	RemoveBatchFromSetInterval        = 5 * time.Minute
)

// NOTE(marian): Leaving this commented code here as it may be useful in the short term.
// type AvsSubscriberer interface {
// 	SubscribeToNewTasks(newTaskCreatedChan chan *cstaskmanager.ContractAlignedLayerTaskManagerNewTaskCreated) event.Subscription
// 	SubscribeToTaskResponses(taskResponseLogs chan *cstaskmanager.ContractAlignedLayerTaskManagerTaskResponded) event.Subscription
// 	ParseTaskResponded(rawLog types.Log) (*cstaskmanager.ContractAlignedLayerTaskManagerTaskResponded, error)
// }

// Subscribers use a ws connection instead of http connection like Readers
// kind of stupid that the geth client doesn't have a unified interface for both...
// it takes a single url, so the bindings, even though they have watcher functions, those can't be used
// with the http connection... seems very very stupid. Am I missing something?
type AvsSubscriber struct {
	AvsContractBindings            *AvsServiceBindings
	AlignedLayerServiceManagerAddr ethcommon.Address
	logger                         sdklogging.Logger
}

type ErrorPair struct {
	ErrorMainRPC     error
	ErrorFallbackRPC error
}

func NewAvsSubscriberFromConfig(baseConfig *config.BaseConfig) (*AvsSubscriber, error) {
	avsContractBindings, err := NewAvsServiceBindings(
		baseConfig.AlignedLayerDeploymentConfig.AlignedLayerServiceManagerAddr,
		baseConfig.AlignedLayerDeploymentConfig.AlignedLayerOperatorStateRetrieverAddr,
		baseConfig.EthWsClient, baseConfig.EthWsClientFallback, baseConfig.Logger)

	if err != nil {
		baseConfig.Logger.Errorf("Failed to create contract bindings", "err", err)
		return nil, err
	}

	return &AvsSubscriber{
		AvsContractBindings:            avsContractBindings,
		AlignedLayerServiceManagerAddr: baseConfig.AlignedLayerDeploymentConfig.AlignedLayerServiceManagerAddr,
		logger:                         baseConfig.Logger,
	}, nil
}

func (s *AvsSubscriber) SubscribeToNewTasksV2(newTaskCreatedChan chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2, errorPairChannel chan ErrorPair) *ErrorPair {
	// Create a new channel to receive new tasks
	internalChannel := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2)

	// Subscribe to new tasks
	sub, errMain := SubscribeToNewTasksV2Retryable(&bind.WatchOpts{}, s.AvsContractBindings.ServiceManager, internalChannel, nil, retry.NetworkRetryParams())
	if errMain != nil {
		s.logger.Error(fmt.Sprintf("Fallback failed to subscribe to new AlignedLayer V3 tasks after %d retries", MaxRetries), "errMain", fmt.Sprintf("%v", errMain))
	}

	subFallback, errFallback := SubscribeToNewTasksV2Retryable(&bind.WatchOpts{}, s.AvsContractBindings.ServiceManagerFallback, internalChannel, nil, retry.NetworkRetryParams())
	if errFallback != nil {
		s.logger.Error(fmt.Sprintf("Fallback failed to subscribe to new AlignedLayer V3 tasks after %d retries", MaxRetries), "errFallback", fmt.Sprintf("%v", errFallback))
	}

	if errMain != nil && errFallback != nil {
		s.logger.Error("Failed to subscribe to new AlignedLayer V2 tasks with both RPCs", "errMain", errMain, "errFallback", errFallback)
		return &ErrorPair{ErrorMainRPC: errMain, ErrorFallbackRPC: errFallback}
	}

	s.logger.Info("Subscribed to new AlignedLayer V2 tasks")

	pollLatestBatchTicker := time.NewTicker(PollLatestBatchInterval)

	// Forward the new tasks to the provided channel
	go func() {
		defer pollLatestBatchTicker.Stop()
		newBatchMutex := &sync.Mutex{}
		batchesSet := make(map[[32]byte]struct{})
		for {
			select {
			case newBatch := <-internalChannel:
				s.processNewBatchV2(newBatch, batchesSet, newBatchMutex, newTaskCreatedChan)
			case <-pollLatestBatchTicker.C:
				latestBatch, err := s.getLatestNotRespondedTaskFromEthereumV2()
				if err != nil {
					s.logger.Debug("Failed to get latest task from blockchain", "err", err)
					continue
				}
				if latestBatch != nil {
					s.processNewBatchV2(latestBatch, batchesSet, newBatchMutex, newTaskCreatedChan)
				}
			}
		}

	}()

	// Handle errors and resubscribe
	go func() {
		var errMain, errFallback error
		var auxSub, auxSubFallback event.Subscription
		for errMain == nil || errFallback == nil { //while one is active
			select {
			case err := <-sub.Err():
				s.logger.Warn("Error in new task subscription of main connection", "err", err)

				auxSub, errMain = SubscribeToNewTasksV2Retryable(&bind.WatchOpts{}, s.AvsContractBindings.ServiceManager, internalChannel, nil, retry.NetworkRetryParams())
				if errMain == nil {
					sub = auxSub // update the subscription only if it was successful
					s.logger.Info("Main connection resubscribed to new task subscription")
				}
			case err := <-subFallback.Err():
				s.logger.Warn("Error in new task subscription of fallback connection", "err", err)

				auxSubFallback, errFallback = SubscribeToNewTasksV2Retryable(&bind.WatchOpts{}, s.AvsContractBindings.ServiceManagerFallback, internalChannel, nil, retry.NetworkRetryParams())
				if errFallback == nil {
					subFallback = auxSubFallback // update the subscription only if it was successful
					s.logger.Info("Resubscribed to fallback new task subscription")
				}
			}
		}
		errorPairChannel <- ErrorPair{ErrorMainRPC: errMain, ErrorFallbackRPC: errFallback}
	}()

	return nil
}

func (s *AvsSubscriber) SubscribeToNewTasksV3(newTaskCreatedChan chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3, errorPairChannel chan ErrorPair) *ErrorPair {
	// Create a new channel to receive new tasks
	internalChannel := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3)

	s.logger.Info("Starting subscription to new AlignedLayer V3 tasks")
	// Subscribe to new tasks
	sub, errMain := SubscribeToNewTasksV3Retryable(&bind.WatchOpts{}, s.AvsContractBindings.ServiceManager, internalChannel, nil, retry.NetworkRetryParams())
	if errMain != nil {
		s.logger.Error(fmt.Sprintf("Fallback failed to subscribe to new AlignedLayer V3 tasks after %d retries", MaxRetries), "err", fmt.Sprintf("%v", errMain))
	}

	subFallback, errFallback := SubscribeToNewTasksV3Retryable(&bind.WatchOpts{}, s.AvsContractBindings.ServiceManagerFallback, internalChannel, nil, retry.NetworkRetryParams())
	if errFallback != nil {
		s.logger.Error(fmt.Sprintf("Fallback failed to subscribe to new AlignedLayer V3 tasks after %d retries", MaxRetries), "err", fmt.Sprintf("%v", errFallback))
	}

	if errMain != nil && errFallback != nil {
		s.logger.Error("Failed to subscribe to new AlignedLayer V3 tasks with both RPCs", "errMain", errMain, "errFallback", errFallback)
		return &ErrorPair{ErrorMainRPC: errMain, ErrorFallbackRPC: errFallback}
	}

	s.logger.Info("Subscribed to new AlignedLayer V3 tasks")

	pollLatestBatchTicker := time.NewTicker(PollLatestBatchInterval)

	// Forward the new tasks to the provided channel
	go func() {
		defer pollLatestBatchTicker.Stop()
		newBatchMutex := &sync.Mutex{}
		batchesSet := make(map[[32]byte]struct{})
		for {
			select {
			case newBatch := <-internalChannel:
				s.processNewBatchV3(newBatch, batchesSet, newBatchMutex, newTaskCreatedChan)
			case <-pollLatestBatchTicker.C:
				latestBatch, err := s.getLatestNotRespondedTaskFromEthereumV3()
				if err != nil {
					s.logger.Debug("Failed to get latest task from blockchain", "err", err)
					continue
				}
				if latestBatch != nil {
					s.processNewBatchV3(latestBatch, batchesSet, newBatchMutex, newTaskCreatedChan)
				}
			}
		}

	}()

	// Handle errors and resubscribe
	go func() {
		s.logger.Info("Starting error handling goroutine")
		var errMain, errFallback error
		var auxSub, auxSubFallback event.Subscription
		for errMain == nil || errFallback == nil { //while one is active
			select {
			case err := <-sub.Err():
				s.logger.Warn("Error in new task subscription of main connection", "err", err)

				auxSub, errMain = SubscribeToNewTasksV3Retryable(&bind.WatchOpts{}, s.AvsContractBindings.ServiceManager, internalChannel, nil, retry.NetworkRetryParams())
				if errMain == nil {
					sub = auxSub // update the subscription only if it was successful
					s.logger.Info("Resubscribed to fallback new task subscription")
				}
			case err := <-subFallback.Err():
				s.logger.Warn("Error in new task subscription of fallback connection", "err", err)

				auxSubFallback, errFallback = SubscribeToNewTasksV3Retryable(&bind.WatchOpts{}, s.AvsContractBindings.ServiceManagerFallback, internalChannel, nil, retry.NetworkRetryParams())
				if errFallback == nil {
					subFallback = auxSubFallback // update the subscription only if it was successful
					s.logger.Info("Resubscribed to fallback new task subscription")
				}
			}
		}
		errorPairChannel <- ErrorPair{ErrorMainRPC: errMain, ErrorFallbackRPC: errFallback}
	}()

	return nil
}

func (s *AvsSubscriber) processNewBatchV2(batch *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2, batchesSet map[[32]byte]struct{}, newBatchMutex *sync.Mutex, newTaskCreatedChan chan<- *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2) {
	newBatchMutex.Lock()
	defer newBatchMutex.Unlock()

	batchIdentifier := append(batch.BatchMerkleRoot[:], batch.SenderAddress[:]...)
	var batchIdentifierHash = *(*[32]byte)(crypto.Keccak256(batchIdentifier))

	if _, ok := batchesSet[batchIdentifierHash]; !ok {
		s.logger.Info("Received new task",
			"batchMerkleRoot", hex.EncodeToString(batch.BatchMerkleRoot[:]),
			"senderAddress", hex.EncodeToString(batch.SenderAddress[:]),
			"batchIdentifierHash", hex.EncodeToString(batchIdentifierHash[:]))

		batchesSet[batchIdentifierHash] = struct{}{}
		newTaskCreatedChan <- batch

		// Remove the batch from the set after RemoveBatchFromSetInterval time
		go func() {
			time.Sleep(RemoveBatchFromSetInterval)
			newBatchMutex.Lock()
			delete(batchesSet, batchIdentifierHash)
			newBatchMutex.Unlock()
		}()
	}
}

func (s *AvsSubscriber) processNewBatchV3(batch *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3, batchesSet map[[32]byte]struct{}, newBatchMutex *sync.Mutex, newTaskCreatedChan chan<- *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3) {
	newBatchMutex.Lock()
	defer newBatchMutex.Unlock()

	batchIdentifier := append(batch.BatchMerkleRoot[:], batch.SenderAddress[:]...)
	var batchIdentifierHash = *(*[32]byte)(crypto.Keccak256(batchIdentifier))

	if _, ok := batchesSet[batchIdentifierHash]; !ok {
		s.logger.Info("Received new task",
			"batchMerkleRoot", hex.EncodeToString(batch.BatchMerkleRoot[:]),
			"senderAddress", hex.EncodeToString(batch.SenderAddress[:]),
			"batchIdentifierHash", hex.EncodeToString(batchIdentifierHash[:]))

		batchesSet[batchIdentifierHash] = struct{}{}
		newTaskCreatedChan <- batch

		// Remove the batch from the set after RemoveBatchFromSetInterval time
		go func() {
			time.Sleep(RemoveBatchFromSetInterval)
			newBatchMutex.Lock()
			delete(batchesSet, batchIdentifierHash)
			newBatchMutex.Unlock()
		}()
	}
}

// getLatestNotRespondedTaskFromEthereum queries the blockchain for the latest not responded task using the FilterNewBatch method.
func (s *AvsSubscriber) getLatestNotRespondedTaskFromEthereumV2() (*servicemanager.ContractAlignedLayerServiceManagerNewBatchV2, error) {

	latestBlock, err := s.BlockNumberRetryable(context.Background(), retry.NetworkRetryParams())
	if err != nil {
		return nil, err
	}

	var fromBlock uint64

	if latestBlock < BlockInterval {
		fromBlock = 0
	} else {
		fromBlock = latestBlock - BlockInterval
	}

	logs, err := s.FilterBatchV2Retryable(&bind.FilterOpts{Start: fromBlock, End: nil, Context: context.Background()}, nil, retry.NetworkRetryParams())
	if err != nil {
		return nil, err
	}

	var lastLog *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2

	// Iterate over the logs until the end
	for logs.Next() {
		lastLog = logs.Event
	}

	if err := logs.Error(); err != nil {
		return nil, err
	}

	if lastLog == nil {
		return nil, nil
	}

	batchIdentifier := append(lastLog.BatchMerkleRoot[:], lastLog.SenderAddress[:]...)
	batchIdentifierHash := *(*[32]byte)(crypto.Keccak256(batchIdentifier))
	state, err := s.BatchesStateRetryable(nil, batchIdentifierHash, retry.NetworkRetryParams())
	if err != nil {
		return nil, err
	}

	if state.Responded {
		return nil, nil
	}

	return lastLog, nil
}

// getLatestNotRespondedTaskFromEthereum queries the blockchain for the latest not responded task using the FilterNewBatch method.
func (s *AvsSubscriber) getLatestNotRespondedTaskFromEthereumV3() (*servicemanager.ContractAlignedLayerServiceManagerNewBatchV3, error) {
	latestBlock, err := s.BlockNumberRetryable(context.Background(), retry.NetworkRetryParams())
	if err != nil {
		return nil, err
	}

	var fromBlock uint64

	if latestBlock < BlockInterval {
		fromBlock = 0
	} else {
		fromBlock = latestBlock - BlockInterval
	}

	logs, err := s.FilterBatchV3Retryable(&bind.FilterOpts{Start: fromBlock, End: nil, Context: context.Background()}, nil, retry.NetworkRetryParams())
	if err != nil {
		return nil, err
	}

	var lastLog *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3

	// Iterate over the logs until the end
	for logs.Next() {
		lastLog = logs.Event
	}

	if err := logs.Error(); err != nil {
		return nil, err
	}

	if lastLog == nil {
		return nil, nil
	}

	batchIdentifier := append(lastLog.BatchMerkleRoot[:], lastLog.SenderAddress[:]...)
	batchIdentifierHash := *(*[32]byte)(crypto.Keccak256(batchIdentifier))
	state, err := s.BatchesStateRetryable(nil, batchIdentifierHash, retry.NetworkRetryParams())
	if err != nil {
		return nil, err
	}

	if state.Responded {
		return nil, nil
	}

	return lastLog, nil
}

func (s *AvsSubscriber) WaitForOneBlock(startBlock uint64) error {
	currentBlock, err := s.BlockNumberRetryable(context.Background(), retry.NetworkRetryParams())
	if err != nil {
		return err
	}

	if currentBlock <= startBlock { // should really be == but just in case
		// Subscribe to new head
		c := make(chan *types.Header)
		sub, err := s.SubscribeNewHeadRetryable(context.Background(), c, retry.NetworkRetryParams())
		if err != nil {
			return err
		}

		// Read channel for the new block
		<-c
		(sub).Unsubscribe()
	}

	return nil
}

// func (s *AvsSubscriber) SubscribeToTaskResponses(taskResponseChan chan *cstaskmanager.ContractAlignedLayerTaskManagerTaskResponded) event.Subscription {
// 	sub, err := s.AvsContractBindings.TaskManager.WatchTaskResponded(
// 		&bind.WatchOpts{}, taskResponseChan,
// 	)
// 	if err != nil {
// 		s.logger.Error("Failed to subscribe to TaskResponded events", "err", err)
// 	}
// 	s.logger.Infof("Subscribed to TaskResponded events")
// 	return sub
// }

// func (s *AvsSubscriber) ParseTaskResponded(rawLog types.Log) (*cstaskmanager.ContractAlignedLayerTaskManagerTaskResponded, error) {
// 	return s.AvsContractBindings.TaskManager.ContractAlignedLayerTaskManagerFilterer.ParseTaskResponded(rawLog)
// }
