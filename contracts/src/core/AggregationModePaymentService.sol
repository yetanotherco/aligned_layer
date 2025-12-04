// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin-upgrades/contracts/proxy/utils/UUPSUpgradeable.sol";

/**
 * @title AggregationModePaymentService
 * @author Aligned Layer
 * @notice Handles deposits that grant time-limited access to aggregation services.
 */
contract AggregationModePaymentService is Initializable, OwnableUpgradeable, UUPSUpgradeable {
    /// @notice for how much time the payment is valid in seconds (86400s = 24hs)
    uint256 public constant PAYMENT_VALID_UNTIL_SECONDS = 86400;

    /**
     * @notice Emitted when a user deposits funds to purchase service time.
     * @param user Address that sent the payment.
     * @param amount Native token amount paid.
     * @param from Timestamp when the payment was recorded.
     * @param until Timestamp until when the payment is valid.
     */
    event UserPayment(address user, uint256 indexed amount, uint256 indexed from, uint256 indexed until);

    error InvalidDepositAmount(uint256 amount);

    /**
     * @notice Disables initializers for the implementation contract.
     */
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the contract and transfers ownership to the provided address.
     * @param _owner Address that becomes the contract owner.
     */
    function initialize(address _owner) public initializer {
        __Ownable_init();
        __UUPSUpgradeable_init();
        _transferOwnership(_owner);
    }

    /**
     * @notice Ensures only the owner can authorize upgrades.
     * @param newImplementation Address of the new implementation contract.
     */
    function _authorizeUpgrade(address newImplementation)
        internal
        override
        onlyOwner // solhint-disable-next-line no-empty-blocks
    {}

    /**
     * @notice Accepts payments and validates they meet the minimum requirement.
     */
    receive() external payable {
        uint256 amount = msg.value;

        // 1 eth
        if (amount < 1000000000000000000) {
            revert InvalidDepositAmount(amount);
        }

        emit UserPayment(msg.sender, amount, block.timestamp, block.timestamp + PAYMENT_VALID_UNTIL_SECONDS);
    }
}
