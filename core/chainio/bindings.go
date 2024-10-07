package chainio

import (
	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	"github.com/Layr-Labs/eigensdk-go/logging"

	gethcommon "github.com/ethereum/go-ethereum/common"

	csservicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
)

type AvsServiceBindings struct {
	ServiceManager         *csservicemanager.ContractAlignedLayerServiceManager
	ServiceManagerFallback *csservicemanager.ContractAlignedLayerServiceManager
	ethClient              eth.InstrumentedClient
	ethClientFallback      eth.InstrumentedClient
	logger                 logging.Logger
}

func NewAvsServiceBindings(serviceManagerAddr, blsOperatorStateRetrieverAddr gethcommon.Address, ethClient eth.InstrumentedClient, ethClientFallback eth.InstrumentedClient, logger logging.Logger) (*AvsServiceBindings, error) {
	contractServiceManager, err := csservicemanager.NewContractAlignedLayerServiceManager(serviceManagerAddr, &ethClient)
	if err != nil {
		logger.Error("Failed to fetch AlignedLayerServiceManager contract", "err", err)
		return nil, err
	}

	contractServiceManagerFallback, err := csservicemanager.NewContractAlignedLayerServiceManager(serviceManagerAddr, &ethClientFallback)
	if err != nil {
		logger.Error("Failed to fetch AlignedLayerServiceManager contract", "err", err)
		return nil, err
	}

	return &AvsServiceBindings{
		ServiceManager:         contractServiceManager,
		ServiceManagerFallback: contractServiceManagerFallback,
		ethClient:              ethClient,
		ethClientFallback:      ethClientFallback,
		logger:                 logger,
	}, nil
}
