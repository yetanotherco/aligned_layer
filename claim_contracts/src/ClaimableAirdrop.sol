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

    /// @notice Mapping of the claimants that have claimed the tokens.
    /// @dev true if the claimant has claimed the tokens.
    mapping(address claimer => bool claimed) public hasClaimed;

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

    /// @notice Claim the tokens.
    /// @param amount amount of tokens to claim.
    /// @param merkleProof Merkle proof of the claim.
    function claim(
        uint256 amount,
        bytes32[] calldata merkleProof
    ) external nonReentrant whenNotPaused {
        require(
            !hasClaimed[msg.sender],
            "Account has already claimed the drop"
        );
        require(
            block.timestamp <= limitTimestampToClaim,
            "Drop is no longer claimable"
        );

        bytes32 leaf = keccak256(
            bytes.concat(keccak256(abi.encode(msg.sender, amount)))
        );
        bool verifies = MerkleProof.verify(merkleProof, claimMerkleRoot, leaf);

        require(verifies, "Invalid Merkle proof");

        // Done before the transfer call to make sure the reentrancy bug is not possible
        hasClaimed[msg.sender] = true;

        bool success = IERC20(tokenProxy).transferFrom(
            tokenDistributor,
            msg.sender,
            amount
        );

        require(success, "Failed to transfer funds");

        emit TokensClaimed(msg.sender, amount);
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
