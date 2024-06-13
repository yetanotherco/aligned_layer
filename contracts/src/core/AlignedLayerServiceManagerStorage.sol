import "eigenlayer-middleware/interfaces/IServiceManager.sol";

contract AlignedLayerServiceManagerStorage {
    struct BatchState {
        uint32 taskCreatedBlock;
        bool responded;
    }

    /* STORAGE */
    mapping(bytes32 => BatchState) public batchesState;

    // storage gap for upgradeability
    uint256[49] private __GAP;
}