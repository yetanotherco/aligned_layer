// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin-upgrades/contracts/proxy/utils/UUPSUpgradeable.sol";
import {AccessControlUpgradeable} from "@openzeppelin-upgrades/contracts/access/AccessControlUpgradeable.sol";

/**
 * @title AggregationModePaymentService
 * @author Aligned Layer
 * @notice Handles deposits that grant time-limited access to aggregation services.
 */
contract AggregationModePaymentService is Initializable, UUPSUpgradeable, AccessControlUpgradeable {

    bytes32 public constant OWNER_ROLE = keccak256("OWNER_ROLE");
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");

    /// @notice for how much time the payment is valid in seconds
    uint256 public paymentExpirationTimeSeconds;

    /// @notice The amount to pay for a subscription in wei.
    uint256 public amountToPayInWei;

    /// @notice The address where the payment funds will be sent.
    address public paymentFundsRecipient;

    /// @notice The limit of subscriptions for different addresses
    uint256 public subscriptionLimit;

    /// @notice Number of current subscriptions
    uint256 public activeSubscriptionsAmount;

    /// @notice Maximum amount of time (in seconds) an address can be subscribed ahead of the current block timestamp.
    /// Prevents stacking multiple short subscriptions and paying them over an extended period.
    uint256 public maxSubscriptionTimeAhead;

    /// @notice Number of addresses currently subscribed. 
    /// @dev `expirationTime` is a Unix timestamp (UTC seconds) compared against block timestamps.
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
    /// @param newSubscriptionLimit the new subscription limit.
    event SubscriptionLimitUpdated(uint256 indexed newSubscriptionLimit);

    /// @notice Event emitted when the subscription amount is updated
    /// @param newSubscriptionsAmount the new subscriptions amount.
    event ActiveSubscriptionsAmountUpdated(uint256 indexed newSubscriptionsAmount);

    /// @notice Event emitted when the max subscription time ahead is updated
    /// @param newMaxSubscriptionTimeAhead the max time allowed to subscribe ahead the current timestamp.
    event MaxSubscriptionTimeAheadUpdated(uint256 indexed newMaxSubscriptionTimeAhead);

    /// @notice Event emitted when the funds recipient is updated
    /// @param newFundsRecipient the new address for receiving the funds on withdrawal.
    event FundsRecipientUpdated(address indexed newFundsRecipient);

    /// @notice Event emitted when the balance is withdrawn to the recipient address
    /// @param recipient the address where the funds will be sent
    /// @param amount the amont send to the recipient address
    event FundsWithdrawn(address indexed recipient, uint256 amount);

    error InvalidDepositAmount(uint256 amountReceived, uint256 amountRequired);

    error SubscriptionLimitReached(uint256 subscriptionLimit);

    error SubscriptionTimeExceedsLimit(uint256 newSubscriptionTime, uint256 timeLimit);

    /**
     * @notice Disables initializers for the implementation contract.
     */
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the contract and transfers ownership to the provided address.
     * @param _owner Address that becomes the contract owner.
     * @param _admin Address that becomes the contract admin.
     * @param _paymentFundsRecipient Address that will receive the withdrawal funds.
     * @param _amountToPayInWei Amount to pay in wei for the subscription.
     * @param _paymentExpirationTimeSeconds The time in seconds that the subscription takes to expire.
     * @param _subscriptionLimit The maximum subscribers that can be subscribed at the same time.
     * 
     */
    function initialize(
        address _owner,
        address _admin,
        address _paymentFundsRecipient,
        uint256 _amountToPayInWei,
        uint256 _paymentExpirationTimeSeconds,
        uint256 _subscriptionLimit,
        uint256 _maxSubscriptionTimeAhead
    ) public initializer {
        __UUPSUpgradeable_init();
        _grantRole(OWNER_ROLE, _owner);
        _grantRole(ADMIN_ROLE, _admin);

        paymentExpirationTimeSeconds = _paymentExpirationTimeSeconds;
        amountToPayInWei = _amountToPayInWei;
        paymentFundsRecipient = _paymentFundsRecipient;
        subscriptionLimit = _subscriptionLimit;
        maxSubscriptionTimeAhead = _maxSubscriptionTimeAhead;
    }

    /**
     * @notice Ensures only the owner can authorize upgrades.
     * @param newImplementation Address of the new implementation contract.
     */
    function _authorizeUpgrade(address newImplementation)
        internal
        override
        onlyRole(OWNER_ROLE) // solhint-disable-next-line no-empty-blocks
    {}

    /**
     * @notice Sets the new expiration time. Only callable by the owner
     * @param newExpirationTimeInSeconds The new expiration time for the users payments in seconds.
     */
    function setPaymentExpirationTimeSeconds(uint256 newExpirationTimeInSeconds) public onlyRole(OWNER_ROLE) {
        paymentExpirationTimeSeconds = newExpirationTimeInSeconds;

        emit PaymentExpirationTimeUpdated(newExpirationTimeInSeconds);
    }

    /**
     * @notice Sets the new amount to pay. Only callable by the owner
     * @param newRecipient The new address for receiving the funds on withdrawal.
     */
    function setFundsRecipientAddress(address newRecipient) public onlyRole(OWNER_ROLE) {
        paymentFundsRecipient = newRecipient;

        emit FundsRecipientUpdated(newRecipient);
    }

    /**
     * @notice Sets the new amount to pay. Only callable by the owner
     * @param newAmountToPay The new amount to pay for subscription in wei.
     */
    function setAmountToPay(uint256 newAmountToPay) public onlyRole(OWNER_ROLE) {
        amountToPayInWei = newAmountToPay;

        emit AmountToPayUpdated(newAmountToPay);
    }

    /**
     * @notice Sets the new subscription limit. Only callable by the owner
     * @param newSubscriptionLimit The new subscription limit.
     */
    function setSubscriptionLimit(uint256 newSubscriptionLimit) public onlyRole(OWNER_ROLE) {
        subscriptionLimit = newSubscriptionLimit;

        emit SubscriptionLimitUpdated(newSubscriptionLimit);
    }

    /**
     * @notice Sets the subscriptions counter to the value received by parameter. Only callable by the owner
     * @param newSubscriptionsAmount The new subscriptions amount.
     */
    function setActiveSubscriptionsAmount(uint256 newSubscriptionsAmount) public onlyRole(ADMIN_ROLE) {
        activeSubscriptionsAmount = newSubscriptionsAmount;

        emit ActiveSubscriptionsAmountUpdated(newSubscriptionsAmount);
    }
    
    /**
     * @notice Sets the  max subscription time ahead to the value received by parameter. Only callable by the owner
     * @param newMaxSubscriptionTimeAhead max time allowed to subscribe ahead the current timestamp.
     */
    function setMaxSubscriptionTimeAhead(uint256 newMaxSubscriptionTimeAhead) public onlyRole(OWNER_ROLE) {
        maxSubscriptionTimeAhead = newMaxSubscriptionTimeAhead;

        emit MaxSubscriptionTimeAheadUpdated(newMaxSubscriptionTimeAhead);
    }

    /**
     * @notice Adds an array of addresses to the payment map and emits the Payment event.
     * @param addressesToAdd the addresses to be subscribed
     * @param expirationTimestamp the expiration timestamp (UTC seconds) for that subscriptions
     * Note: this method adds the subscriptions without checking if the final amount of subscriptions surpasses
     * the subscriptionLimit
     */
    function addSubscriptions(address[] memory addressesToAdd, uint256 expirationTimestamp) public onlyRole(ADMIN_ROLE) {
        for (uint256 i=0; i < addressesToAdd.length; ++i) {
            address addressToAdd = addressesToAdd[i];

            subscribedAddresses[addressToAdd] = expirationTimestamp;

            ++activeSubscriptionsAmount;

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

        if (activeSubscriptionsAmount >= subscriptionLimit) {
            revert SubscriptionLimitReached(subscriptionLimit);
        }

        if (subscribedAddresses[msg.sender] < block.timestamp) {
            // Subscription is inactive/expired: start a new period from now.
            subscribedAddresses[msg.sender] = block.timestamp + paymentExpirationTimeSeconds;
        } else {
            // Subscription is still active: extend the current expiry by one period.
            subscribedAddresses[msg.sender] = subscribedAddresses[msg.sender] + paymentExpirationTimeSeconds;
        }

        uint256 newExpiration = subscribedAddresses[msg.sender];

        if (newExpiration - block.timestamp > maxSubscriptionTimeAhead) {
            revert SubscriptionTimeExceedsLimit(newExpiration, maxSubscriptionTimeAhead);
        }

        ++activeSubscriptionsAmount;

        emit UserPayment(msg.sender, amount, block.timestamp, block.timestamp + paymentExpirationTimeSeconds);
    }

    /**
     * @notice Withdraws the contract balance to the recipient address.
     */
    function withdraw() external onlyRole(OWNER_ROLE) {
        uint256 balance = address(this).balance;
        payable(paymentFundsRecipient).transfer(balance);
        emit FundsWithdrawn(paymentFundsRecipient, balance);
    }
}
