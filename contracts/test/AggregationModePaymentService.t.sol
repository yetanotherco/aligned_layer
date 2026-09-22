// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import {AggregationModePaymentService} from "../src/core/AggregationModePaymentService.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract AggregationModePaymentServiceTest is Test {
    AggregationModePaymentService service;

    address owner = address(0x1);
    address admin = address(0x2);
    address fundsRecipient = address(0x3);
    address user = address(0x4);

    uint256 constant AMOUNT_TO_PAY = 1 ether;
    uint256 constant EXPIRATION_SECONDS = 30 days;
    uint256 constant SUBSCRIPTION_LIMIT = 100;
    uint256 constant MAX_TIME_AHEAD = 365 days;

    event UserPayment(address user, uint256 indexed amount, uint256 indexed from, uint256 indexed until);

    function setUp() public {
        AggregationModePaymentService implementation = new AggregationModePaymentService();

        bytes memory initData = abi.encodeWithSelector(
            AggregationModePaymentService.initialize.selector,
            owner,
            admin,
            fundsRecipient,
            AMOUNT_TO_PAY,
            EXPIRATION_SECONDS,
            SUBSCRIPTION_LIMIT,
            MAX_TIME_AHEAD
        );

        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        service = AggregationModePaymentService(payable(address(proxy)));

        vm.deal(user, 10 ether);
    }

    /// @notice New subscription (no prior active period): `until` should equal
    /// `block.timestamp + paymentExpirationTimeSeconds`, i.e. `newExpiration`.
    /// This already matches the buggy emission by coincidence, so it does not
    /// by itself prove the fix -- see the extension test below for that.
    function test_UserPayment_emitsCorrectUntil_onFreshSubscription() public {
        uint256 startTime = block.timestamp;
        uint256 expectedUntil = startTime + EXPIRATION_SECONDS;

        vm.expectEmit(true, true, true, true, address(service));
        emit UserPayment(user, AMOUNT_TO_PAY, startTime, expectedUntil);

        vm.prank(user);
        (bool ok,) = address(service).call{value: AMOUNT_TO_PAY}("");
        assertTrue(ok);

        assertEq(service.subscribedAddresses(user), expectedUntil);
    }

    /// @notice Extending an already-active subscription: the emitted `until`
    /// must be the *actual* new expiration (extended from the current expiry),
    /// not `block.timestamp + paymentExpirationTimeSeconds` computed from the
    /// second payment's timestamp. Before the fix, this event carries a wrong
    /// (earlier) value than what was actually stored in `subscribedAddresses`.
    function test_UserPayment_emitsCorrectUntil_onExtendedSubscription() public {
        // First payment: starts a fresh subscription.
        vm.prank(user);
        (bool ok1,) = address(service).call{value: AMOUNT_TO_PAY}("");
        assertTrue(ok1);

        uint256 firstExpiration = service.subscribedAddresses(user);

        // Move forward, but stay within the still-active subscription window.
        vm.warp(block.timestamp + 5 days);

        uint256 secondPaymentTime = block.timestamp;
        uint256 realNewExpiration = firstExpiration + EXPIRATION_SECONDS;

        // The buggy code would instead emit secondPaymentTime + EXPIRATION_SECONDS,
        // which is 5 days earlier than the real stored expiration.
        assertTrue(realNewExpiration != secondPaymentTime + EXPIRATION_SECONDS);

        vm.expectEmit(true, true, true, true, address(service));
        emit UserPayment(user, AMOUNT_TO_PAY, secondPaymentTime, realNewExpiration);

        vm.prank(user);
        (bool ok2,) = address(service).call{value: AMOUNT_TO_PAY}("");
        assertTrue(ok2);

        // The event must match what's actually persisted in storage.
        assertEq(service.subscribedAddresses(user), realNewExpiration);
    }
}
