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

    // storage gap for upgradeability
    // solhint-disable-next-line var-name-mixedcase
    uint256[47] private __GAP;
}
