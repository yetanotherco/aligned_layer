pragma solidity ^0.8.12;
import {IAlignedLayerServiceManager} from "./IAlignedLayerServiceManager.sol";

abstract contract BatcherPaymentServiceStorage {
    struct SignatureData {
        bytes signature;
        uint256 nonce;
        uint256 maxFee;
    }

    struct UserInfo {
        uint256 balance;
        uint256 unlockBlock;
        uint256 nonce;
    }

    IAlignedLayerServiceManager public alignedLayerServiceManager;

    address public batcherWallet;

    // map to user data
    mapping(address => UserInfo) public userData;

    bytes32 public noncedVerificationDataTypeHash;

    // storage gap for upgradeability
    // solhint-disable-next-line var-name-mixedcase
    uint256[23] private __GAP;
}
