// SPDX-License-Identifier: MIT
pragma solidity 0.8.35;

import {Test} from "forge-std/Test.sol";
import {ClaimableAirdrop} from "../src/ClaimableAirdrop.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {ERC20ReturnFalseMock} from "@openzeppelin/contracts/mocks/token/ERC20ReturnFalseMock.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";

/// @dev Concrete instance of the abstract OZ mock that returns `false` from
///      transfer/transferFrom instead of reverting. Used to exercise the
///      SafeERC20 path (FL-AL-2).
contract ERC20ReturnFalse is ERC20ReturnFalseMock {
    constructor() ERC20("ReturnFalse", "RF") {}
}

contract ClaimableAirdropTest is Test {
    ClaimableAirdrop internal airdrop;
    ERC20Mock internal token;

    address internal foundation = makeAddr("foundation");
    address internal distributor = makeAddr("distributor");
    address internal claimant;
    uint256 internal claimantPk;

    uint256 internal constant AMOUNT = 1_000 ether;
    uint256 internal constant DEADLINE = 1_000_000;

    function setUp() public {
        (claimant, claimantPk) = makeAddrAndKey("claimant");

        token = new ERC20Mock();

        ClaimableAirdrop impl = new ClaimableAirdrop();
        bytes memory initData = abi.encodeCall(
            ClaimableAirdrop.initialize,
            (foundation, address(token), distributor)
        );
        airdrop = ClaimableAirdrop(address(new ERC1967Proxy(address(impl), initData)));

        // Fund the distributor and approve the airdrop to pull tokens.
        token.mint(distributor, 1_000_000 ether);
        vm.prank(distributor);
        token.approve(address(airdrop), type(uint256).max);
    }

    /* -------------------------------------------------------------------------- */
    /*                                 Merkle utils                               */
    /* -------------------------------------------------------------------------- */

    function _leaf(address to, uint256 amount, uint256 validFrom) internal pure returns (bytes32) {
        return keccak256(bytes.concat(keccak256(abi.encode(to, amount, validFrom))));
    }

    /// @dev Sets a single-leaf tree (root == leaf, empty proof) as the active root.
    ///      A single-leaf StandardMerkleTree has the leaf itself as its root and an
    ///      empty proof, so no proof generation is needed for the single-claim cases.
    function _setSingleLeafRoot(address to, uint256 amount, uint256 validFrom)
        internal
        returns (bytes32[] memory proof)
    {
        bytes32 root = _leaf(to, amount, validFrom);
        vm.prank(foundation);
        airdrop.updateMerkleRoot(root);
        proof = new bytes32[](0);
    }

    /* -------------------------------------------------------------------------- */
    /*                          Merkle fixture (generated)                        */
    /* -------------------------------------------------------------------------- */

    /// @dev One leaf and its proof, as produced by the Rust fixture generator
    ///      (test/fixtures/generator) which uses the same merkle-tree-rs library and
    ///      leaf encoding as the production proof generator. Proofs are NOT computed
    ///      in Solidity; the tests only consume the generator's output.
    struct FixtureLeaf {
        address account;
        uint256 amount;
        uint256 validFrom;
        bytes32[] proof;
    }

    string internal constant FIXTURE_PATH = "test/fixtures/proofs.json";

    /// @dev Loads the generated Merkle root and every leaf+proof from the JSON fixture.
    function _loadFixture() internal view returns (bytes32 root, FixtureLeaf[] memory leaves) {
        string memory json = vm.readFile(FIXTURE_PATH);
        root = vm.parseJsonBytes32(json, ".root");

        uint256 count = vm.parseJsonUint(json, ".count");
        leaves = new FixtureLeaf[](count);
        for (uint256 i = 0; i < count; i++) {
            string memory base = string.concat(".leaves[", vm.toString(i), "]");
            leaves[i] = FixtureLeaf({
                account: vm.parseJsonAddress(json, string.concat(base, ".account")),
                amount: vm.parseJsonUint(json, string.concat(base, ".amount")),
                validFrom: vm.parseJsonUint(json, string.concat(base, ".validFrom")),
                proof: vm.parseJsonBytes32Array(json, string.concat(base, ".proof"))
            });
        }
    }

    /// @dev Loads the fixture, installs its root, opens the claim period and unpauses.
    function _armFixture() internal returns (FixtureLeaf[] memory leaves) {
        bytes32 root;
        (root, leaves) = _loadFixture();
        vm.prank(foundation);
        airdrop.updateMerkleRoot(root);
        vm.prank(foundation);
        airdrop.extendClaimPeriod(DEADLINE);
        vm.prank(foundation);
        airdrop.unpause();
    }

    /* -------------------------------------------------------------------------- */
    /*                                Initialization                              */
    /* -------------------------------------------------------------------------- */

    function test_initialize_setsState() public view {
        assertEq(airdrop.tokenProxy(), address(token));
        assertEq(airdrop.tokenDistributor(), distributor);
        assertEq(airdrop.owner(), foundation);
        assertEq(airdrop.limitTimestampToClaim(), 0);
        assertEq(airdrop.claimMerkleRoot(), bytes32(0));
        assertTrue(airdrop.paused());
    }

    function test_initialize_cannotBeCalledTwice() public {
        vm.expectRevert();
        airdrop.initialize(foundation, address(token), distributor);
    }

    /* -------------------------------------------------------------------------- */
    /*                                    claim                                    */
    /* -------------------------------------------------------------------------- */

    function _arm(uint256 validFrom) internal returns (bytes32[] memory proof) {
        proof = _setSingleLeafRoot(claimant, AMOUNT, validFrom);
        vm.prank(foundation);
        airdrop.extendClaimPeriod(DEADLINE);
        vm.prank(foundation);
        airdrop.unpause();
    }

    function test_claim_success() public {
        bytes32[] memory proof = _arm(0);

        vm.prank(claimant);
        airdrop.claim(AMOUNT, 0, proof);

        assertEq(token.balanceOf(claimant), AMOUNT);
        assertTrue(airdrop.hasClaimed(_leaf(claimant, AMOUNT, 0)));
    }

    function test_claim_revertsWhenPaused() public {
        bytes32[] memory proof = _setSingleLeafRoot(claimant, AMOUNT, 0);
        vm.prank(foundation);
        airdrop.extendClaimPeriod(DEADLINE);
        // still paused

        vm.prank(claimant);
        vm.expectRevert(PausableUpgradeable.EnforcedPause.selector);
        airdrop.claim(AMOUNT, 0, proof);
    }

    function test_claim_revertsAfterDeadline() public {
        bytes32[] memory proof = _arm(0);
        vm.warp(DEADLINE + 1);

        vm.prank(claimant);
        vm.expectRevert("Drop is no longer claimable");
        airdrop.claim(AMOUNT, 0, proof);
    }

    function test_claim_revertsWhenStageNotYetClaimable() public {
        uint256 validFrom = 500;
        bytes32[] memory proof = _arm(validFrom);
        vm.warp(validFrom - 1);

        vm.prank(claimant);
        vm.expectRevert("Stage not yet claimable");
        airdrop.claim(AMOUNT, validFrom, proof);
    }

    function test_claim_revertsOnDoubleClaim() public {
        bytes32[] memory proof = _arm(0);

        vm.prank(claimant);
        airdrop.claim(AMOUNT, 0, proof);

        vm.prank(claimant);
        vm.expectRevert("Stage already claimed");
        airdrop.claim(AMOUNT, 0, proof);
    }

    function test_claim_revertsOnInvalidProof() public {
        _arm(0);
        bytes32[] memory badProof = new bytes32[](1);
        badProof[0] = keccak256("garbage");

        vm.prank(claimant);
        vm.expectRevert("Invalid Merkle proof");
        airdrop.claim(AMOUNT, 0, badProof);
    }

    function test_claim_revertsForWrongAmount() public {
        bytes32[] memory proof = _arm(0);

        vm.prank(claimant);
        vm.expectRevert("Invalid Merkle proof");
        airdrop.claim(AMOUNT + 1, 0, proof);
    }

    function test_claim_revertsForNonEntitledCaller() public {
        bytes32[] memory proof = _arm(0);
        address attacker = makeAddr("attacker");

        vm.prank(attacker);
        vm.expectRevert("Invalid Merkle proof");
        airdrop.claim(AMOUNT, 0, proof);
    }

    /* -------------------------------------------------------------------------- */
    /*                 claim against the generated Merkle tree                     */
    /* -------------------------------------------------------------------------- */

    /// @dev Every leaf in the generated tree can be claimed with its generator
    ///      proof. Validates the on-chain verifier against real merkle-tree-rs output
    ///      (multi-level proofs), not a Solidity reimplementation.
    function test_claim_fixtureProofs_allClaim() public {
        FixtureLeaf[] memory leaves = _armFixture();
        vm.warp(2000); // >= every leaf's validFrom, <= DEADLINE

        for (uint256 i = 0; i < leaves.length; i++) {
            FixtureLeaf memory lf = leaves[i];
            assertGt(lf.proof.length, 1, "expected a multi-level proof");

            uint256 balanceBefore = token.balanceOf(lf.account);
            vm.prank(lf.account);
            airdrop.claim(lf.amount, lf.validFrom, lf.proof);

            assertEq(token.balanceOf(lf.account), balanceBefore + lf.amount);
            assertTrue(airdrop.hasClaimed(_leaf(lf.account, lf.amount, lf.validFrom)));
        }
    }

    /// @dev A valid leaf cannot be claimed with a different leaf's generator proof.
    function test_claim_fixtureProofs_wrongProofReverts() public {
        FixtureLeaf[] memory leaves = _armFixture();
        vm.warp(2000);

        // leaves[3] is claimed with leaves[4]'s proof.
        FixtureLeaf memory victim = leaves[3];
        bytes32[] memory wrongProof = leaves[4].proof;

        vm.prank(victim.account);
        vm.expectRevert("Invalid Merkle proof");
        airdrop.claim(victim.amount, victim.validFrom, wrongProof);
    }

    /* -------------------------------------------------------------------------- */
    /*                                 claimBatch                                 */
    /* -------------------------------------------------------------------------- */

    function test_claimBatch_success() public {
        FixtureLeaf[] memory leaves = _armFixture();
        vm.warp(2000); // >= every stage's validFrom, <= DEADLINE

        // The fixture gives the first account multiple vesting stages; claim them all
        // in a single batch using the generator proofs.
        address account = leaves[0].account;
        uint256 n;
        for (uint256 i = 0; i < leaves.length; i++) {
            if (leaves[i].account == account) n++;
        }
        require(n > 1, "fixture must have a multi-stage account");

        uint256[] memory amounts = new uint256[](n);
        uint256[] memory validFroms = new uint256[](n);
        bytes32[][] memory proofs = new bytes32[][](n);
        uint256 total;
        uint256 k;
        for (uint256 i = 0; i < leaves.length; i++) {
            if (leaves[i].account != account) continue;
            amounts[k] = leaves[i].amount;
            validFroms[k] = leaves[i].validFrom;
            proofs[k] = leaves[i].proof;
            total += leaves[i].amount;
            k++;
        }

        vm.prank(account);
        airdrop.claimBatch(amounts, validFroms, proofs);

        assertEq(token.balanceOf(account), total);
        for (uint256 i = 0; i < n; i++) {
            assertTrue(airdrop.hasClaimed(_leaf(account, amounts[i], validFroms[i])));
        }
    }

    function test_claimBatch_revertsOnArrayLengthMismatch() public {
        bytes32[] memory proof = _arm(0);

        uint256[] memory amounts = new uint256[](1);
        amounts[0] = AMOUNT;
        uint256[] memory validFroms = new uint256[](2); // mismatched
        bytes32[][] memory proofs = new bytes32[][](1);
        proofs[0] = proof;

        vm.prank(claimant);
        vm.expectRevert("Array length mismatch");
        airdrop.claimBatch(amounts, validFroms, proofs);
    }

    /// @dev An empty batch sums to zero and must revert rather than be a no-op.
    function test_claimBatch_revertsOnEmptyBatch() public {
        _arm(0); // open the claim period

        uint256[] memory amounts = new uint256[](0);
        uint256[] memory validFroms = new uint256[](0);
        bytes32[][] memory proofs = new bytes32[][](0);

        vm.prank(claimant);
        vm.expectRevert("Nothing to claim");
        airdrop.claimBatch(amounts, validFroms, proofs);
    }

    /// @dev The same leaf cannot be claimed twice within a single batch: the first
    ///      entry marks it claimed, so the second hits the double-claim guard.
    function test_claimBatch_revertsOnDuplicateLeaf() public {
        FixtureLeaf[] memory leaves = _armFixture();
        vm.warp(2000);

        FixtureLeaf memory lf = leaves[0];

        uint256[] memory amounts = new uint256[](2);
        amounts[0] = lf.amount;
        amounts[1] = lf.amount;
        uint256[] memory validFroms = new uint256[](2);
        validFroms[0] = lf.validFrom;
        validFroms[1] = lf.validFrom;
        bytes32[][] memory proofs = new bytes32[][](2);
        proofs[0] = lf.proof;
        proofs[1] = lf.proof;

        vm.prank(lf.account);
        vm.expectRevert("Stage already claimed");
        airdrop.claimBatch(amounts, validFroms, proofs);
    }

    /* -------------------------------------------------------------------------- */
    /*                              Access control                                */
    /* -------------------------------------------------------------------------- */

    function test_updateMerkleRoot_onlyOwner() public {
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, address(this)));
        airdrop.updateMerkleRoot(keccak256("root"));
    }

    function test_updateMerkleRoot_requiresPaused() public {
        _arm(0); // leaves the contract unpaused
        vm.prank(foundation);
        vm.expectRevert(PausableUpgradeable.ExpectedPause.selector);
        airdrop.updateMerkleRoot(keccak256("new"));
    }

    function test_extendClaimPeriod_onlyOwner() public {
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, address(this)));
        airdrop.extendClaimPeriod(DEADLINE);
    }

    function test_extendClaimPeriod_mustMoveForward() public {
        vm.prank(foundation);
        airdrop.extendClaimPeriod(DEADLINE);
        vm.prank(foundation);
        vm.expectRevert("Can only extend from current timestamp");
        airdrop.extendClaimPeriod(DEADLINE); // not greater than current
    }

    function test_pause_unpause_onlyOwner() public {
        vm.prank(foundation);
        airdrop.unpause();
        assertFalse(airdrop.paused());

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, address(this)));
        airdrop.pause();

        vm.prank(foundation);
        airdrop.pause();
        assertTrue(airdrop.paused());
    }

    /// @dev FL-AL-1: renouncing ownership must be impossible.
    function test_renounceOwnership_reverts() public {
        vm.prank(foundation);
        vm.expectRevert("Cannot renounce ownership");
        airdrop.renounceOwnership();
        assertEq(airdrop.owner(), foundation);
    }

    /* -------------------------------------------------------------------------- */
    /*                               SafeERC20 path                               */
    /* -------------------------------------------------------------------------- */

    /// @dev FL-AL-2: a token that returns `false` (instead of reverting) must still
    ///      cause the claim to revert via SafeERC20, leaving no tokens transferred.
    function test_claim_revertsWhenTokenReturnsFalse() public {
        ERC20ReturnFalse badToken = new ERC20ReturnFalse();

        ClaimableAirdrop impl = new ClaimableAirdrop();
        bytes memory initData = abi.encodeCall(
            ClaimableAirdrop.initialize,
            (foundation, address(badToken), distributor)
        );
        ClaimableAirdrop badAirdrop =
            ClaimableAirdrop(address(new ERC1967Proxy(address(impl), initData)));

        bytes32 root = _leaf(claimant, AMOUNT, 0);
        vm.prank(foundation);
        badAirdrop.updateMerkleRoot(root);
        vm.prank(foundation);
        badAirdrop.extendClaimPeriod(DEADLINE);
        vm.prank(foundation);
        badAirdrop.unpause();

        bytes32[] memory proof = new bytes32[](0);
        vm.prank(claimant);
        vm.expectRevert(); // SafeERC20FailedOperation
        badAirdrop.claim(AMOUNT, 0, proof);
    }

    /// @dev FL-AL-2/FL-AL-3: a failed transfer rolls back the whole claim, including
    ///      the `hasClaimed` write, so the leaf is not stranded and the user can retry
    ///      once the distributor is funded/approved again.
    function test_claim_failedTransferDoesNotStrandLeaf() public {
        bytes32[] memory proof = _arm(0);
        bytes32 leaf = _leaf(claimant, AMOUNT, 0);

        // Distributor revokes the approval -> the transfer (and thus the claim) reverts.
        vm.prank(distributor);
        token.approve(address(airdrop), 0);

        vm.prank(claimant);
        vm.expectRevert(); // ERC20InsufficientAllowance, bubbled by SafeERC20
        airdrop.claim(AMOUNT, 0, proof);

        // The leaf must remain unclaimed and no tokens moved.
        assertFalse(airdrop.hasClaimed(leaf));
        assertEq(token.balanceOf(claimant), 0);

        // After the distributor restores the approval, the same claim succeeds.
        vm.prank(distributor);
        token.approve(address(airdrop), type(uint256).max);

        vm.prank(claimant);
        airdrop.claim(AMOUNT, 0, proof);

        assertEq(token.balanceOf(claimant), AMOUNT);
        assertTrue(airdrop.hasClaimed(leaf));
    }
}
