pragma solidity ^0.8.12;

import {IAlignedLayerServiceManager} from "./IAlignedLayerServiceManager.sol";

abstract contract BatcherPaymentServiceStorage {
    struct UserInfo {
        uint256 balance;
        uint256 unlockBlockTime;
        uint256 nonce;
    }

    IAlignedLayerServiceManager public alignedLayerServiceManager;

    address public batcherWallet;

    // map to user data
    mapping(address => UserInfo) public userData;

    // storage gap for upgradeability
    // solhint-disable-next-line var-name-mixedcase
    uint256[24] private __GAP;
}
