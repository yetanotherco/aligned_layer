pragma solidity ^0.8.12;

import "eigenlayer-middleware/interfaces/IServiceManager.sol";

contract AlignedLayerServiceManagerStorage {
    struct BatchState {
        uint32 taskCreatedBlock;
        bool responded;
        address batcherAddress;
    }

    /* STORAGE */
    mapping(bytes32 => BatchState) public batchesState;

    // Storage for batchers balances. Used by aggregator to pay for respondToTask
    mapping(address => uint256) public batchersBalances;

    // storage gap for upgradeability
    uint256[48] private __GAP;
}
