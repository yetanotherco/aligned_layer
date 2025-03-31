use alloy::{
    providers::{
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
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
    (),
    FillProvider<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        RootProvider,
    >,
>;
