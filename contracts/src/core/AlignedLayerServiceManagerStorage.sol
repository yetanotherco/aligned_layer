pragma solidity ^0.8.12;

import "eigenlayer-middleware/interfaces/IServiceManager.sol";

contract AlignedLayerServiceManagerStorage {
    struct BatchState {
        address batcherAddress;
        uint32 taskCreatedBlock;
        bool responded;
    }

    /* STORAGE */
    mapping(bytes32 => BatchState) public batchesState;

    // Storage for batchers balances. Used by aggregator to pay for respondToTask
    mapping(address => uint256) internal batchersBalances;

    // storage gap for upgradeability
    uint256[48] private __GAP;
}
