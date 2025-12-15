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

    /// @notice The limit of subscriptions for different addresses per month
    uint256 public subscriptionLimit;

    /// @notice The amount of subscriptions for the current month
    uint256 public monthlySubscriptionsAmount;

    /// @notice The amount of addresses currently subscribed. expirationTime is UTC seconds, to be
    /// compared against block timestamps
    mapping(address subscriber => uint256 expirationTime) public subscribedAddresses;

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

    /// @notice Event emitted when the subscription limit is updated
    /// @param newSubscriptionLimit the new monthly subscription limit.
    event SubscriptionLimitUpdated(uint256 indexed newSubscriptionLimit);

    /// @notice Event emitted when the funds recipient is updated
    /// @param newFundsRecipient the new address for receiving the funds on withdrawal.
    event FundsRecipientUpdated(address indexed newFundsRecipient);

    /// @notice Event emitted when the balance is withdrawn to the recipient address
    /// @param recipient the address where the funds will be sent
    /// @param amount the amont send to the recipient address
    event FundsWithdrawn(address indexed recipient, uint256 amount);

    error InvalidDepositAmount(uint256 amountReceived, uint256 amountRequired);

    error SubscriptionLimitReached(uint256 subscriptionLimit);

    error SubscriptionNotExpired(uint256 expiration, uint256 currentTime);

    /**
     * @notice Disables initializers for the implementation contract.
     */
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the contract and transfers ownership to the provided address.
     * @param _owner Address that becomes the contract owner.
     * @param _paymentFundsRecipient Address that will receive the withdrawal funds.
     * @param _amountToPayInWei Amount to pay in wei for the subscription.
     * @param _paymentExpirationTimeSeconds The time in seconds that the subscription takes to expire.
     * @param _subscriptionLimit The maximum subscribers that can be subscribed at the same time.
     * 
     */
    function initialize(
        address _owner,
        address _paymentFundsRecipient,
        uint256 _amountToPayInWei,
        uint256 _paymentExpirationTimeSeconds,
        uint256 _subscriptionLimit
    ) public initializer {
        __Ownable_init();
        __UUPSUpgradeable_init();
        _transferOwnership(_owner);

        paymentExpirationTimeSeconds = _paymentExpirationTimeSeconds;
        amountToPayInWei = _amountToPayInWei;
        paymentFundsRecipient = _paymentFundsRecipient;
        subscriptionLimit = _subscriptionLimit;
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
     * @notice Sets the new subscription limit. Only callable by the owner
     * @param newSubscriptionLimit The new monthly subscription limit.
     */
    function setSubscriptionLimit(uint256 newSubscriptionLimit) public onlyOwner() {
        subscriptionLimit = newSubscriptionLimit;

        emit SubscriptionLimitUpdated(newSubscriptionLimit);
    }

    /**
     * @notice Resets the monthly subscriptions mapping counter to zero.
     */
    function resetSubscriptions() public onlyOwner() {
        monthlySubscriptionsAmount = 0;
    }

    /**
     * @notice Adds an array of addresses to the payment map and emits the Payment event.
     * @param addressesToAdd the addresses to be subscribed
     * @param expirationTimestamp the expiration timestamp (UTC seconds) for that subscriptions
     */
    function addArbitraryExpirationSubscriptions(address[] memory addressesToAdd, uint256 expirationTimestamp) public onlyOwner() {
        for (uint256 i=0; i < addressesToAdd.length; ++i) {
            address addressToAdd = addressesToAdd[i];

            subscribedAddresses[addressToAdd] = expirationTimestamp;

            emit UserPayment(addressToAdd, amountToPayInWei, block.timestamp, expirationTimestamp);
        }
    }

    /**
     * @notice Accepts payments and validates they meet the minimum requirement.
     */
    receive() external payable {
        uint256 amount = msg.value;

        if (amount < amountToPayInWei) {
            revert InvalidDepositAmount(amount, amountToPayInWei);
        }

        if (monthlySubscriptionsAmount == subscriptionLimit) {
            revert SubscriptionLimitReached(subscriptionLimit);
        }

        // Check if the user has already payed a subscription for this month
        uint256 insertedExpiration = subscribedAddresses[msg.sender];
        if (insertedExpiration == 0) {
            // this means the sender has not payed for a subscription before
            subscribedAddresses[msg.sender] = block.timestamp + paymentExpirationTimeSeconds;
        } else if (insertedExpiration > block.timestamp) {
            // this means the sender has an expired subscription
            subscribedAddresses[msg.sender] = block.timestamp + paymentExpirationTimeSeconds;
        } else {
            // this means the sender has a subscription that has not expired yet
            revert SubscriptionNotExpired(insertedExpiration, block.timestamp);
        }

        ++monthlySubscriptionsAmount;

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
