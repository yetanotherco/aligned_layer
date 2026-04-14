// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {MerkleProof} from "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";
import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";

/// @title Claimable Airdrop
/// @notice This contract is the implementation of the Claimable Airdrop
/// @dev This contract is upgradeable and should be used only through the proxy contract
/// @custom:security-contact security@alignedfoundation.org
contract ClaimableAirdrop is
    Initializable,
    ReentrancyGuardUpgradeable,
    PausableUpgradeable,
    Ownable2StepUpgradeable
{
    /// @notice Address of the token contract to claim.
    address public tokenProxy;

    /// @notice Address of the wallet that has the tokens to distribute to the claimants.
    address public tokenDistributor;

    /// @notice Timestamp until which the claimants can claim the tokens.
    uint256 public limitTimestampToClaim;

    /// @notice Merkle root of the claimants.
    bytes32 public claimMerkleRoot;

    /// @notice Mapping tracking claimed leaves.
    /// @dev Key is the Merkle leaf itself.
    mapping(bytes32 => bool) public hasClaimed;

    /// @notice Event emitted when a claimant claims the tokens.
    /// @param to address of the claimant.
    /// @param amount amount of tokens claimed.
    event TokensClaimed(address indexed to, uint256 indexed amount);

    /// @notice Event emitted when the Merkle root is updated.
    /// @param newRoot new Merkle root.
    event MerkleRootUpdated(bytes32 indexed newRoot);

    /// @notice Event emitted when the claim period is extended.
    /// @param newTimestamp new timestamp until which the claimants can claim the tokens.
    event ClaimPeriodExtended(uint256 indexed newTimestamp);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract.
    /// @dev This initializer should be called only once.
    /// @param _foundation address of the Aligned foundation.
    /// @param _tokenProxy address of the token contract.
    /// @param _tokenDistributor address of the wallet that has the tokens to distribute to the claimants.
    function initialize(
        address _foundation,
        address _tokenProxy,
        address _tokenDistributor
    ) external initializer {
        require(_foundation != address(0), "Invalid foundation address");
        require(_tokenProxy != address(0), "Invalid token contract address");
        require(_tokenDistributor != address(0), "Invalid token owner address");

        __Ownable_init(_foundation);
        __Pausable_init();
        __ReentrancyGuard_init();

        tokenProxy = _tokenProxy;
        tokenDistributor = _tokenDistributor;
        limitTimestampToClaim = 0;
        claimMerkleRoot = 0;

        _pause();
    }

    /// @notice Claim tokens for a single vesting stage.
    /// @param amount amount of tokens to claim for this stage.
    /// @param validFrom timestamp from which this stage is claimable.
    /// @param merkleProof Merkle proof of the claim.
    function claim(
        uint256 amount,
        uint256 validFrom,
        bytes32[] calldata merkleProof
    ) external nonReentrant whenNotPaused {
        require(
            block.timestamp <= limitTimestampToClaim,
            "Drop is no longer claimable"
        );

        _verifyAndMark(amount, validFrom, merkleProof);

        bool success = IERC20(tokenProxy).transferFrom(
            tokenDistributor,
            msg.sender,
            amount
        );
        require(success, "Failed to transfer funds");

        emit TokensClaimed(msg.sender, amount);
    }

    /// @notice Claim tokens for multiple vesting stages in a single transaction.
    /// @param amounts array of token amounts per stage.
    /// @param validFroms array of timestamps from which each stage is claimable.
    /// @param merkleProofs array of Merkle proofs, one per stage.
    function claimBatch(
        uint256[] calldata amounts,
        uint256[] calldata validFroms,
        bytes32[][] calldata merkleProofs
    ) external nonReentrant whenNotPaused {
        require(
            block.timestamp <= limitTimestampToClaim,
            "Drop is no longer claimable"
        );
        uint256 length = amounts.length;
        require(
            length == validFroms.length && length == merkleProofs.length,
            "Array length mismatch"
        );

        uint256 totalClaimable = 0;

        for (uint256 i = 0; i < length; i++) {
            _verifyAndMark(amounts[i], validFroms[i], merkleProofs[i]);
            totalClaimable += amounts[i];
        }

        require(totalClaimable > 0, "Nothing to claim");

        bool success = IERC20(tokenProxy).transferFrom(
            tokenDistributor,
            msg.sender,
            totalClaimable
        );
        require(success, "Failed to transfer funds");

        emit TokensClaimed(msg.sender, totalClaimable);
    }

    /// @notice Verify a single-stage Merkle proof and mark the stage as claimed.
    /// @dev Shared by `claim` and `claimBatch`. Does not perform the token transfer.
    /// @param amount amount of tokens for this stage.
    /// @param validFrom timestamp from which this stage is claimable.
    /// @param merkleProof Merkle proof for this stage.
    function _verifyAndMark(
        uint256 amount,
        uint256 validFrom,
        bytes32[] calldata merkleProof
    ) internal {
        require(
            block.timestamp >= validFrom,
            "Stage not yet claimable"
        );

        bytes32 leaf = keccak256(
            bytes.concat(
                keccak256(abi.encode(msg.sender, amount, validFrom))
            )
        );

        require(!hasClaimed[leaf], "Stage already claimed");

        bool verifies = MerkleProof.verify(
            merkleProof,
            claimMerkleRoot,
            leaf
        );
        require(verifies, "Invalid Merkle proof");

        hasClaimed[leaf] = true;
    }

    /// @notice Update the Merkle root.
    /// @param newRoot new Merkle root.
    function updateMerkleRoot(bytes32 newRoot) external whenPaused onlyOwner {
        require(newRoot != 0 && newRoot != claimMerkleRoot, "Invalid root");
        claimMerkleRoot = newRoot;
        emit MerkleRootUpdated(newRoot);
    }

    /// @notice Extend the claim period.
    /// @param newTimestamp new timestamp until which the claimants can claim the tokens.
    function extendClaimPeriod(
        uint256 newTimestamp
    ) external whenPaused onlyOwner {
        require(
            newTimestamp > limitTimestampToClaim &&
                newTimestamp > block.timestamp,
            "Can only extend from current timestamp"
        );
        limitTimestampToClaim = newTimestamp;
        emit ClaimPeriodExtended(newTimestamp);
    }

    /// @notice Pause the contract.
    function pause() external onlyOwner {
        _pause();
    }

    /// @notice Unpause the contract.
    function unpause() external onlyOwner {
        _unpause();
    }
}
