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
    /// @notice for how much time the payment is valid in seconds
    uint256 public paymentExpirationTimeSeconds;

    /// @notice The amount to pay for a subscription in wei.
    uint256 public amountToPayInWei;

    /// @notice The address where the payment funds will be sent.
    address public paymentFundsRecipient;

    /**
     * @notice Emitted when a user deposits funds to purchase service time.
     * @param user Address that sent the payment.
     * @param amount Native token amount paid.
     * @param from Timestamp when the payment was recorded.
     * @param until Timestamp until when the payment is valid.
     */
    event UserPayment(address user, uint256 indexed amount, uint256 indexed from, uint256 indexed until);

    /// @notice Event emitted when the payment expiration time is updated
    /// @param newExpirationTime the new expiration time in seconds
    event PaymentExpirationTimeUpdated(uint256 indexed newExpirationTime);

    /// @notice Event emitted when the amount to pay for subscription is updated
    /// @param newAmountToPay the new amount to pay for a subscription in wei.
    event AmountToPayUpdated(uint256 indexed newAmountToPay);

    /// @notice Event emitted when the funds recipient is updated
    /// @param newFundsRecipient the new address for receiving the funds on withdrawal.
    event FundsRecipientUpdated(address indexed newFundsRecipient);

    /// @notice Event emitted when the balance is withdrawn to the recipient address
    /// @param recipient the address where the funds will be sent
    /// @param amount the amont send to the recipient address
    event FundsWithdrawn(address indexed recipient, uint256 amount);

    error InvalidDepositAmount(uint256 amountReceived, uint256 amountRequired);

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
    function initialize(address _owner, address _paymentFundsRecipient, uint256 _amountToPayInWei, uint256 _paymentExpirationTimeSeconds) public initializer {
        __Ownable_init();
        __UUPSUpgradeable_init();
        _transferOwnership(_owner);

        paymentExpirationTimeSeconds = _paymentExpirationTimeSeconds;
        amountToPayInWei = _amountToPayInWei;
        paymentFundsRecipient = _paymentFundsRecipient;
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
     * @notice Sets the new expiration time. Only callable by the owner
     * @param newExpirationTimeInSeconds The new expiration time for the users payments in seconds.
     */
    function setPaymentExpirationTimeSeconds(uint256 newExpirationTimeInSeconds) public onlyOwner() {
        paymentExpirationTimeSeconds = newExpirationTimeInSeconds;

        emit PaymentExpirationTimeUpdated(newExpirationTimeInSeconds);
    }

    /**
     * @notice Sets the new amount to pay. Only callable by the owner
     * @param newRecipient The new address for receiving the funds on withdrawal.
     */
    function setFundsRecipientAddress(address newRecipient) public onlyOwner() {
        paymentFundsRecipient = newRecipient;

        emit FundsRecipientUpdated(newRecipient);
    }

    /**
     * @notice Sets the new amount to pay. Only callable by the owner
     * @param newAmountToPay The new amount to pay for subscription in wei.
     */
    function setAmountToPay(uint256 newAmountToPay) public onlyOwner() {
        amountToPayInWei = newAmountToPay;

        emit AmountToPayUpdated(newAmountToPay);
    }

    /**
     * @notice Accepts payments and validates they meet the minimum requirement.
     */
    receive() external payable {
        uint256 amount = msg.value;

        if (amount < amountToPayInWei) {
            revert InvalidDepositAmount(amount, amountToPayInWei);
        }

        emit UserPayment(msg.sender, amount, block.timestamp, block.timestamp + paymentExpirationTimeSeconds);
    }

    /**
     * @notice Withdraws the contract balance to the recipient address.
     */
    function withdraw() external onlyOwner {
        uint256 balance = address(this).balance;
        payable(paymentFundsRecipient).transfer(balance);
        emit FundsWithdrawn(paymentFundsRecipient, balance);
    }
}
