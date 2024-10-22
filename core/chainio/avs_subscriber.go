package chainio

import (
	"context"
	"encoding/hex"
	"fmt"
	"math/big"
	"sync"
	"time"

	"github.com/ethereum/go-ethereum"
	ethcommon "github.com/ethereum/go-ethereum/common"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/config"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/ethereum/go-ethereum/crypto"
	connection "github.com/yetanotherco/aligned_layer/core"
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

func (s *AvsSubscriber) SubscribeToNewTasksV2Retrayable(newTaskCreatedChan chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2) (chan error, error) {
	// Create a new channel to receive new tasks
	internalChannel := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2)

	// Subscribe to new tasks
	//TODO: Retry()
	sub, err := subscribeToNewTasksV2Retrayable(s.AvsContractBindings.ServiceManager, internalChannel, s.logger)
	if err != nil {
		s.logger.Error("Failed to subscribe to new AlignedLayer tasks", "err", err)
		return nil, err
	}

	//TODO: Retry()
	subFallback, err := subscribeToNewTasksV2Retrayable(s.AvsContractBindings.ServiceManagerFallback, internalChannel, s.logger)
	if err != nil {
		s.logger.Error("Failed to subscribe to new AlignedLayer tasks", "err", err)
		return nil, err
	}

	// create a new channel to foward errors
	errorChannel := make(chan error)

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
				//TODO: Retry()
				latestBatch, err := s.getLatestNotRespondedTaskFromEthereumV2Retryable()
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
		for {
			select {
			case err := <-sub.Err():
				s.logger.Warn("Error in new task subscription", "err", err)
				sub.Unsubscribe()
				//TODO: Retry()
				sub, err = subscribeToNewTasksV2Retrayable(s.AvsContractBindings.ServiceManager, internalChannel, s.logger)
				if err != nil {
					errorChannel <- err
				}
			case err := <-subFallback.Err():
				s.logger.Warn("Error in fallback new task subscription", "err", err)
				subFallback.Unsubscribe()
				//TODO: Retry()
				subFallback, err = subscribeToNewTasksV2Retrayable(s.AvsContractBindings.ServiceManagerFallback, internalChannel, s.logger)
				if err != nil {
					errorChannel <- err
				}
			}
		}
	}()

	return errorChannel, nil
}

func (s *AvsSubscriber) SubscribeToNewTasksV3Retryable(newTaskCreatedChan chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3) (chan error, error) {
	// Create a new channel to receive new tasks
	internalChannel := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3)

	// Subscribe to new tasks
	//TODO: Retry()
	sub, err := subscribeToNewTasksV3Retryable(s.AvsContractBindings.ServiceManager, internalChannel, s.logger)
	if err != nil {
		s.logger.Error("Failed to subscribe to new AlignedLayer tasks", "err", err)
		return nil, err
	}

	//TODO: Retry()
	subFallback, err := subscribeToNewTasksV3Retryable(s.AvsContractBindings.ServiceManagerFallback, internalChannel, s.logger)
	if err != nil {
		s.logger.Error("Failed to subscribe to new AlignedLayer tasks", "err", err)
		return nil, err
	}

	// create a new channel to foward errors
	errorChannel := make(chan error)

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
				//TODO: Retry()
				latestBatch, err := s.getLatestNotRespondedTaskFromEthereumV3Retryable()
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
		for {
			select {
			case err := <-sub.Err():
				s.logger.Warn("Error in new task subscription", "err", err)
				sub.Unsubscribe()
				//TODO: Retry()
				sub, err = subscribeToNewTasksV3Retryable(s.AvsContractBindings.ServiceManager, internalChannel, s.logger)
				if err != nil {
					errorChannel <- err
				}
			case err := <-subFallback.Err():
				s.logger.Warn("Error in fallback new task subscription", "err", err)
				subFallback.Unsubscribe()
				//TODO: Retry()
				subFallback, err = subscribeToNewTasksV3Retryable(s.AvsContractBindings.ServiceManagerFallback, internalChannel, s.logger)
				if err != nil {
					errorChannel <- err
				}
			}
		}
	}()

	return errorChannel, nil
}

func subscribeToNewTasksV2Retrayable(
	serviceManager *servicemanager.ContractAlignedLayerServiceManager,
	newTaskCreatedChan chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV2,
	logger sdklogging.Logger,
) (event.Subscription, error) {
	//TODO: Retry()
	subscribe_func := func() (*event.Subscription, error) {
		sub, err := serviceManager.WatchNewBatchV2(&bind.WatchOpts{}, newTaskCreatedChan, nil)
		return &sub, err
	}
	sub, err := connection.RetryWithData(subscribe_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
	if err != nil {
		// If the retries stop early we propogate the Permanent error to the caller.
		// TODO: ask if this is the behavior we want.
		return nil, fmt.Errorf("failed to subscribe to new AlignedLayer tasks after %d retries", MaxRetries)
	}
	logger.Info("Subscribed to new AlignedLayer tasks")
	return *sub, nil
}

func subscribeToNewTasksV3Retryable(
	serviceManager *servicemanager.ContractAlignedLayerServiceManager,
	newTaskCreatedChan chan *servicemanager.ContractAlignedLayerServiceManagerNewBatchV3,
	logger sdklogging.Logger,
) (event.Subscription, error) {
	//TODO: Retry()
	subscribe_func := func() (*event.Subscription, error) {
		sub, err := serviceManager.WatchNewBatchV3(&bind.WatchOpts{}, newTaskCreatedChan, nil)
		return &sub, err
	}
	sub, err := connection.RetryWithData(subscribe_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
	if err != nil {
		// If the retries stop early we propogate the Permanent error to the caller.
		return nil, fmt.Errorf("failed to subscribe to new AlignedLayer tasks after %d retries", MaxRetries)
	}
	logger.Info("Subscribed to new AlignedLayer tasks")
	return *sub, nil
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
func (s *AvsSubscriber) getLatestNotRespondedTaskFromEthereumV2Retryable() (*servicemanager.ContractAlignedLayerServiceManagerNewBatchV2, error) {

	//TODO: Retry()
	latestBlock_func := func() (*uint64, error) {
		sub, err := s.AvsContractBindings.ethClient.BlockNumber(context.Background())
		return &sub, err
	}
	latestBlock, err := connection.RetryWithData(latestBlock_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
	if err != nil {
		//TODO: Retry()
		latestBlock, err = connection.RetryWithData(latestBlock_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
		if err != nil {
			return nil, err
		}
	}

	var fromBlock uint64

	if *latestBlock < BlockInterval {
		fromBlock = 0
	} else {
		fromBlock = *latestBlock - BlockInterval
	}

	//TODO: Retry()
	filterNewBatchV2_func := func() (*servicemanager.ContractAlignedLayerServiceManagerNewBatchV2Iterator, error) {
		return s.AvsContractBindings.ServiceManager.FilterNewBatchV2(&bind.FilterOpts{Start: fromBlock, End: nil, Context: context.Background()}, nil)
	}
	//logs, err := s.AvsContractBindings.ServiceManager.FilterNewBatchV2(&bind.FilterOpts{Start: fromBlock, End: nil, Context: context.Background()}, nil)
	logs, err := connection.RetryWithData(filterNewBatchV2_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
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
	//TODO: Retry()
	batchState_func := func() (*struct {
		TaskCreatedBlock      uint32
		Responded             bool
		RespondToTaskFeeLimit *big.Int
	}, error) {
		state, err := s.AvsContractBindings.ServiceManager.ContractAlignedLayerServiceManagerCaller.BatchesState(nil, batchIdentifierHash)
		return &state, err
	}

	state, err := connection.RetryWithData(batchState_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
	if err != nil {
		return nil, err
	}

	if state.Responded {
		return nil, nil
	}

	return lastLog, nil
}

// getLatestNotRespondedTaskFromEthereum queries the blockchain for the latest not responded task using the FilterNewBatch method.
func (s *AvsSubscriber) getLatestNotRespondedTaskFromEthereumV3Retryable() (*servicemanager.ContractAlignedLayerServiceManagerNewBatchV3, error) {
	//TODO: Retry()
	latestBlock_func := func() (*uint64, error) {
		latestBlock, err := s.AvsContractBindings.ethClient.BlockNumber(context.Background())
		return &latestBlock, err
	}
	//latestBlock, err := s.AvsContractBindings.ethClient.BlockNumber(context.Background())
	latestBlock, err := connection.RetryWithData(latestBlock_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
	if err != nil {
		//TODO: Retry()
		//s.AvsContractBindings.ethClientFallback.BlockNumber(context.Background())
		latestBlock, err = connection.RetryWithData(latestBlock_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
		if err != nil {
			return nil, err
		}
	}

	var fromBlock uint64

	if *latestBlock < BlockInterval {
		fromBlock = 0
	} else {
		fromBlock = *latestBlock - BlockInterval
	}

	//TODO: Retry()
	filterNewBatchV3_func := func() (*servicemanager.ContractAlignedLayerServiceManagerNewBatchV3Iterator, error) {
		return s.AvsContractBindings.ServiceManager.FilterNewBatchV3(&bind.FilterOpts{Start: fromBlock, End: nil, Context: context.Background()}, nil)
	}
	logs, err := connection.RetryWithData(filterNewBatchV3_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
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
	//TODO: Retry()
	batchesState_func := func() (*struct {
		TaskCreatedBlock      uint32
		Responded             bool
		RespondToTaskFeeLimit *big.Int
	}, error) {
		state, err := s.AvsContractBindings.ServiceManager.ContractAlignedLayerServiceManagerCaller.BatchesState(nil, batchIdentifierHash)
		return &state, err
	}
	//state, err := s.AvsContractBindings.ServiceManager.ContractAlignedLayerServiceManagerCaller.BatchesState(nil, batchIdentifierHash)
	state, err := connection.RetryWithData(batchesState_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
	if err != nil {
		return nil, err
	}

	if state.Responded {
		return nil, nil
	}

	return lastLog, nil
}

func (s *AvsSubscriber) WaitForOneBlockRetryable(startBlock uint64) error {
	//TODO: Retry()
	blockNum_func := func() (*uint64, error) {
		block_num, err := s.AvsContractBindings.ethClient.BlockNumber(context.Background())
		return &block_num, err
	}
	currentBlock, err := connection.RetryWithData(blockNum_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
	if err != nil {
		// try with the fallback client
		//TODO: Retry()
		currentBlock, err = connection.RetryWithData(blockNum_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
		if err != nil {
			return err
		}
	}

	if *currentBlock <= startBlock { // should really be == but just in case
		// Subscribe to new head
		c := make(chan *types.Header)
		//TODO: Retry()
		subscribeNewHead_func := func() (*ethereum.Subscription, error) {
			sub, err := s.AvsContractBindings.ethClient.SubscribeNewHead(context.Background(), c)
			return &sub, err
		}
		sub, err := connection.RetryWithData(subscribeNewHead_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
		if err != nil {
			//TODO: Retry()
			sub, err = connection.RetryWithData(subscribeNewHead_func, connection.MinDelay, connection.RetryFactor, connection.NumRetries)
			if err != nil {
				return err
			}
		}

		// Read channel for the new block
		<-c
		(*sub).Unsubscribe()
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
