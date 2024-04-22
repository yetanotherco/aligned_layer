package chainio

import (
	"github.com/ethereum/go-ethereum/accounts/abi/bind"

	sdkavsregistry "github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	logging "github.com/Layr-Labs/eigensdk-go/logging"

	"github.com/yetanotherco/aligned_layer/core/config"
)

type AvsReader struct {
	sdkavsregistry.AvsRegistryReader
	AvsServiceBindings *AvsServiceBindings
	logger             logging.Logger
}

func NewAvsReaderFromConfig(c *config.Config) (*AvsReader, error) {
	avsContractBindings, err := NewAvsServiceBindings(c.AlignedLayerServiceManagerAddr, c.BlsOperatorStateRetrieverAddr, c.EthHttpClient, c.Logger)
	if err != nil {
		return nil, err
	}
	blsRegistryCoordinatorAddr, err := avsContractBindings.ServiceManager.RegistryCoordinator(&bind.CallOpts{})
	if err != nil {
		return nil, err
	}

	// NOTE(marian): Still not sure if we need these, we should check and remove them if necessary.
	// stakeRegistryAddr, err := avsContractBindings.ServiceManager.StakeRegistry(&bind.CallOpts{})
	// if err != nil {
	// 	return nil, err
	// }
	// blsPubkeyRegistryAddr, err := avsContractBindings.ServiceManager.BlsPubkeyRegistry(&bind.CallOpts{})
	// if err != nil {
	// 	return nil, err
	// }

	ethClient, err := eth.NewClient(c.EthRpcUrl)
	if err != nil {
		return nil, err
	}

	avsRegistryReader, err := sdkavsregistry.BuildAvsRegistryChainReader(blsRegistryCoordinatorAddr, c.BlsOperatorStateRetrieverAddr, ethClient, c.Logger)
	if err != nil {
		return nil, err
	}

	// NOTE(marian): Same as the above commented code.
	// avsRegistryContractClient, err := sdkclients.NewAvsRegistryContractsChainClient(blsRegistryCoordinatorAddr, c.BlsOperatorStateRetrieverAddr, stakeRegistryAddr, blsPubkeyRegistryAddr, ethClient, c.Logger)
	// if err != nil {
	// 	return nil, err
	// }
	// avsRegistryReader, err := sdkavsregistry.NewAvsRegistryReader(avsRegistryContractClient, c.Logger, ethClient)
	// if err != nil {
	// 	return nil, err
	// }

	avsServiceBindings, err := NewAvsServiceBindings(c.AlignedLayerServiceManagerAddr, c.BlsOperatorStateRetrieverAddr, c.EthHttpClient, c.Logger)
	if err != nil {
		return nil, err
	}

	return NewAvsReader(avsRegistryReader, avsServiceBindings, c.Logger)
}

func NewAvsReader(avsRegistryReader sdkavsregistry.AvsRegistryReader, avsServiceBindings *AvsServiceBindings, logger logging.Logger) (*AvsReader, error) {
	return &AvsReader{
		AvsRegistryReader:  avsRegistryReader,
		AvsServiceBindings: avsServiceBindings,
		logger:             logger,
	}, nil
}
