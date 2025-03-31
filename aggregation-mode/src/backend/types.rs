use alloy::{
    network::EthereumWallet,
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, RootProvider,
    },
    sol,
};
use AlignedLayerServiceManager::AlignedLayerServiceManagerInstance;
use AlignedProofAggregationService::AlignedProofAggregationServiceInstance;

sol!(
    #[sol(rpc)]
    AlignedProofAggregationService,
    "abi/AlignedProofAggregationService.json"
);

pub type AlignedProofAggregationServiceContract = AlignedProofAggregationServiceInstance<
    (),
    FillProvider<
        JoinFill<
            JoinFill<
                Identity,
                JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
            >,
            WalletFiller<EthereumWallet>,
        >,
        RootProvider,
    >,
>;

sol!(
    #[sol(rpc)]
    AlignedLayerServiceManager,
    "abi/AlignedLayerServiceManager.json"
);

pub type AlignedLayerServiceManagerContract = AlignedLayerServiceManagerInstance<
    (),
    FillProvider<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        RootProvider,
    >,
>;
