package chainio

import (
	"context"
	"encoding/hex"
	"fmt"
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
	MaxRetries    = 100
	RetryInterval = 1 * time.Second
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
	AvsContractBindings *AvsServiceBindings
	logger              sdklogging.Logger
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
		AvsContractBindings: avsContractBindings,
		logger:              baseConfig.Logger,
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

	// Forward the new tasks to the provided channel
	go func() {
		newBatchMutex := &sync.Mutex{}
		batchesSet := make(map[[32]byte]struct{})
		for {
			newBatch := <-internalChannel
			s.logger.Info("Received new task", "batchMerkleRoot", hex.EncodeToString(newBatch.BatchMerkleRoot[:]))
			newBatchMutex.Lock()

			if _, ok := batchesSet[newBatch.BatchMerkleRoot]; !ok {
				batchesSet[newBatch.BatchMerkleRoot] = struct{}{}
				newTaskCreatedChan <- newBatch

				// Remove the batch from the set after 1 minute
				go func() {
					time.Sleep(time.Minute)
					newBatchMutex.Lock()
					delete(batchesSet, newBatch.BatchMerkleRoot)
					newBatchMutex.Unlock()
				}()
			}

			newBatchMutex.Unlock()
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
