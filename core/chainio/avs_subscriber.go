package chainio

import (
	"context"
	"encoding/hex"
	"fmt"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	ethcommon "github.com/ethereum/go-ethereum/common"
	"math/big"
	"strings"
	"sync"
	"time"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"github.com/yetanotherco/aligned_layer/core/config"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
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

func (s *AvsSubscriber) SubscribeToNewTasks(newTaskCreatedChan chan *servicemanager.ContractAlignedLayerServiceManagerNewBatch) (chan error, error) {
	// Create a new channel to receive new tasks
	internalChannel := make(chan *servicemanager.ContractAlignedLayerServiceManagerNewBatch)

	// Subscribe to new tasks
	sub, err := subscribeToNewTasks(s.AvsContractBindings.ServiceManager, internalChannel, s.logger)
	if err != nil {
		s.logger.Error("Failed to subscribe to new AlignedLayer tasks", "err", err)
		return nil, err
	}

	subFallback, err := subscribeToNewTasks(s.AvsContractBindings.ServiceManagerFallback, internalChannel, s.logger)
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
				s.processNewBatch(newBatch, batchesSet, newBatchMutex, newTaskCreatedChan)
			case <-pollLatestBatchTicker.C:
				latestBatch, err := s.getLatestTaskFromEthereum()
				if err != nil {
					s.logger.Debug("Failed to get latest task from blockchain", "err", err)
					continue
				}
				s.processNewBatch(latestBatch, batchesSet, newBatchMutex, newTaskCreatedChan)
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
				sub, err = subscribeToNewTasks(s.AvsContractBindings.ServiceManager, internalChannel, s.logger)
				if err != nil {
					errorChannel <- err
				}
			case err := <-subFallback.Err():
				s.logger.Warn("Error in fallback new task subscription", "err", err)
				subFallback.Unsubscribe()
				subFallback, err = subscribeToNewTasks(s.AvsContractBindings.ServiceManagerFallback, internalChannel, s.logger)
				if err != nil {
					errorChannel <- err
				}
			}
		}
	}()

	return errorChannel, nil
}

func subscribeToNewTasks(
	serviceManager *servicemanager.ContractAlignedLayerServiceManager,
	newTaskCreatedChan chan *servicemanager.ContractAlignedLayerServiceManagerNewBatch,
	logger sdklogging.Logger,
) (event.Subscription, error) {
	for i := 0; i < MaxRetries; i++ {
		sub, err := serviceManager.WatchNewBatch(
			&bind.WatchOpts{}, newTaskCreatedChan, nil,
		)
		if err != nil {
			logger.Warn("Failed to subscribe to new AlignedLayer tasks", "err", err)
			time.Sleep(RetryInterval)
			continue
		}

		logger.Info("Subscribed to new AlignedLayer tasks")
		return sub, nil
	}

	return nil, fmt.Errorf("Failed to subscribe to new AlignedLayer tasks after %d retries", MaxRetries)
}

func (s *AvsSubscriber) processNewBatch(batch *servicemanager.ContractAlignedLayerServiceManagerNewBatch, batchesSet map[[32]byte]struct{}, newBatchMutex *sync.Mutex, newTaskCreatedChan chan<- *servicemanager.ContractAlignedLayerServiceManagerNewBatch) {
	newBatchMutex.Lock()
	defer newBatchMutex.Unlock()

	if _, ok := batchesSet[batch.BatchMerkleRoot]; !ok {
		s.logger.Info("Received new task", "batchMerkleRoot", hex.EncodeToString(batch.BatchMerkleRoot[:]))
		batchesSet[batch.BatchMerkleRoot] = struct{}{}
		newTaskCreatedChan <- batch

		// Remove the batch from the set after RemoveBatchFromSetInterval time
		go func() {
			time.Sleep(RemoveBatchFromSetInterval)
			newBatchMutex.Lock()
			delete(batchesSet, batch.BatchMerkleRoot)
			newBatchMutex.Unlock()
		}()
	}
}

// getLatestTaskFromEthereum queries the blockchain for the latest task using the FilterLogs method.
// The alternative to this is using the FilterNewBatch method from the contract's filterer, but it requires
// to iterate over all the logs, which is not efficient and not needed since we only need the latest task.
func (s *AvsSubscriber) getLatestTaskFromEthereum() (*servicemanager.ContractAlignedLayerServiceManagerNewBatch, error) {
	latestBlock, err := s.AvsContractBindings.ethClient.BlockNumber(context.Background())
	if err != nil {
		latestBlock, err = s.AvsContractBindings.ethClientFallback.BlockNumber(context.Background())
		if err != nil {
			return nil, fmt.Errorf("failed to get latest block number: %w", err)
		}
	}

	var fromBlock uint64

	if latestBlock < BlockInterval {
		fromBlock = 0
	} else {
		fromBlock = latestBlock - BlockInterval
	}

	alignedLayerServiceManagerABI, err := abi.JSON(strings.NewReader(servicemanager.ContractAlignedLayerServiceManagerMetaData.ABI))
	if err != nil {
		return nil, fmt.Errorf("failed to parse ABI: %w", err)
	}

	// We just care about the NewBatch event
	newBatchEvent := alignedLayerServiceManagerABI.Events["NewBatch"]
	if newBatchEvent.ID == (ethcommon.Hash{}) {
		return nil, fmt.Errorf("NewBatch event not found in ABI")
	}

	query := ethereum.FilterQuery{
		FromBlock: big.NewInt(int64(fromBlock)),
		ToBlock:   big.NewInt(int64(latestBlock)),
		Addresses: []ethcommon.Address{s.AlignedLayerServiceManagerAddr},
		Topics:    [][]ethcommon.Hash{{newBatchEvent.ID, {}}},
	}

	logs, err := s.AvsContractBindings.ethClient.FilterLogs(context.Background(), query)
	if err != nil {
		logs, err = s.AvsContractBindings.ethClientFallback.FilterLogs(context.Background(), query)
		if err != nil {
			return nil, fmt.Errorf("failed to get logs: %w", err)
		}
	}

	if len(logs) == 0 {
		return nil, fmt.Errorf("no logs found")
	}

	lastLog := logs[len(logs)-1]

	var latestTask servicemanager.ContractAlignedLayerServiceManagerNewBatch
	err = alignedLayerServiceManagerABI.UnpackIntoInterface(&latestTask, "NewBatch", lastLog.Data)
	if err != nil {
		return nil, fmt.Errorf("failed to unpack log data: %w", err)
	}

	// The second topic is the batch merkle root, as it is an indexed variable in the contract
	latestTask.BatchMerkleRoot = lastLog.Topics[1]

	return &latestTask, nil

}

func (s *AvsSubscriber) WaitForOneBlock(startBlock uint64) error {
	currentBlock, err := s.AvsContractBindings.ethClient.BlockNumber(context.Background())
	if err != nil {
		// try with the fallback client
		currentBlock, err = s.AvsContractBindings.ethClientFallback.BlockNumber(context.Background())
		if err != nil {
			return err
		}
	}

	if currentBlock <= startBlock { // should really be == but just in case
		// Subscribe to new head
		c := make(chan *types.Header)
		sub, err := s.AvsContractBindings.ethClient.SubscribeNewHead(context.Background(), c)
		if err != nil {
			sub, err = s.AvsContractBindings.ethClientFallback.SubscribeNewHead(context.Background(), c)
			if err != nil {
				return err
			}
		}

		// Read channel for the new block
		<-c
		sub.Unsubscribe()
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
