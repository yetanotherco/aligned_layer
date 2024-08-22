pragma solidity ^0.8.12;

contract AlignedLayerServiceManagerStorage {
    struct BatchState {
        uint32 taskCreatedBlock;
        bool responded;
    }

    /* STORAGE */
    // KEY is keccak256(batchMerkleRoot,senderAddress)
    mapping(bytes32 => BatchState) public batchesState;

    // Storage for batchers balances. Used by aggregator to pay for respondToTask
    mapping(address => uint256) internal batchersBalances;

    // storage gap for upgradeability
    uint256[48] private __GAP;
}
