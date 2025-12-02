// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin-upgrades/contracts/proxy/utils/UUPSUpgradeable.sol";

contract AggregationModePaymentService is Initializable, OwnableUpgradeable, UUPSUpgradeable {
    // 24hs
    uint256 constant paymentValidUntilSeconds = 86400;

    event UserPayment(address user, uint256 amount, uint256 from, uint256 until);

    error InvalidDepositAmount(uint256 amount);

    constructor() {
        _disableInitializers();
    }

    function initialize(address _owner) public initializer {
        __Ownable_init();
        __UUPSUpgradeable_init();
        _transferOwnership(_owner);
    }

    function _authorizeUpgrade(address newImplementation)
        internal
        override
        onlyOwner // solhint-disable-next-line no-empty-blocks
    {}

    receive() external payable {
        uint256 amount = msg.value;

        if (amount < 1) {
            revert InvalidDepositAmount(amount);
        }

        emit UserPayment(msg.sender, amount, block.timestamp, block.timestamp + paymentValidUntilSeconds * amount);
    }
}
