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

    // Bitmap representing blacklisted verifiers
    // Each verifier is blacklisted if its corresponding bit is set to 1
    // The verifier index follows its corresponding value in the `ProvingSystemId` enum being 0 the first verifier.
    uint256 public blacklistedVerifiers;

    // Storage for batchers balances. Used by aggregator to pay for respondToTask
    mapping(address => uint256) public batchersBalances;

    address public alignedAggregator;

    // storage gap for upgradeability
    // solhint-disable-next-line var-name-mixedcase
    uint256[47] private __GAP;
}
