package chainio

import (
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/avsregistry"
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	operatorStateRetriever "github.com/Layr-Labs/eigensdk-go/contracts/bindings/OperatorStateRetriever"
	"github.com/Layr-Labs/eigensdk-go/logging"
	gethcommon "github.com/ethereum/go-ethereum/common"

	csservicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
)

type AvsServiceBindings struct {
	ServiceManager         *csservicemanager.ContractAlignedLayerServiceManager
	ServiceManagerFallback *csservicemanager.ContractAlignedLayerServiceManager
	OperatorStateRetriever *operatorStateRetriever.ContractOperatorStateRetriever
	ContractBindings       *avsregistry.ContractBindings
	ethClient              eth.Client
	ethClientFallback      eth.Client
	logger                 logging.Logger
}

func NewAvsServiceBindings(
	serviceManagerAddr, registryCoordinatorAddr, blsOperatorStateRetrieverAddr gethcommon.Address,
	ethClient, ethClientFallback eth.Client,
	logger logging.Logger,
) (*AvsServiceBindings, error) {
	contractServiceManager, err := csservicemanager.NewContractAlignedLayerServiceManager(serviceManagerAddr, ethClient)
	if err != nil {
		logger.Error("Failed to fetch AlignedLayerServiceManager contract", "err", err)
		return nil, err
	}

	contractServiceManagerFallback, err := csservicemanager.NewContractAlignedLayerServiceManager(serviceManagerAddr, ethClientFallback)
	if err != nil {
		logger.Error("Failed to fetch AlignedLayerServiceManager contract", "err", err)
		return nil, err
	}

	contractBindings, err := avsregistry.NewBindingsFromConfig(avsregistry.Config{
		RegistryCoordinatorAddress:    registryCoordinatorAddr,
		OperatorStateRetrieverAddress: blsOperatorStateRetrieverAddr,
	}, ethClient, logger)
	if err != nil {
		return nil, err
	}

	return &AvsServiceBindings{
		ServiceManager:         contractServiceManager,
		ServiceManagerFallback: contractServiceManagerFallback,
		OperatorStateRetriever: contractBindings.OperatorStateRetriever,
		ContractBindings:       contractBindings,
		ethClient:              ethClient,
		ethClientFallback:      ethClientFallback,
		logger:                 logger,
	}, nil
}
