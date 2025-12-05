#![allow(clippy::too_many_arguments)]
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
use AlignedProofAggregationService::AlignedProofAggregationServiceInstance;

sol!(
    #[sol(rpc)]
    AlignedProofAggregationService,
    "abi/AlignedProofAggregationService.json"
);

pub type AlignedProofAggregationServiceContract = AlignedProofAggregationServiceInstance<
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
