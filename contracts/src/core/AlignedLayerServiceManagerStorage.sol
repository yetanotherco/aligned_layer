pragma solidity ^0.8.12;

abstract contract AlignedLayerServiceManagerStorage {
    struct BatchState {
        uint32 taskCreatedBlock;
        bool responded;
        uint256 respondToTaskFeeLimit;
    }

    /* STORAGE */
    // KEY is keccak256(batchMerkleRoot,senderAddress)
    mapping(bytes32 => BatchState) public batchesState;

    // Storage for batchers balances. Used by aggregator to pay for respondToTask
    mapping(address => uint256) public batchersBalances;

    address public alignedAggregator;

    // Bitmap representing disabled verifiers
    // Each verifier is disabled if its corresponding bit is set to 1
    // The verifier index follows its corresponding value in the `ProvingSystemId` enum being 0 the first verifier.
    uint256 public disabledVerifiers;

    // storage gap for upgradeability
    // solhint-disable-next-line var-name-mixedcase
    uint256[46] private __GAP;
}
