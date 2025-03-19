// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {ERC20Upgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import {ERC20PermitUpgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20PermitUpgradeable.sol";
import {ERC20BurnableUpgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20BurnableUpgradeable.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";

/// @title Aligned Token
/// @custom:security-contact If you find a security bug, please contact us at security@alignedfoundation.org
/// @notice This contract is the implementation of the Aligned Token
/// @dev This contract is upgradeable and should be used only through the proxy contract
contract AlignedToken is
    Initializable,
    ERC20Upgradeable,
    ERC20PermitUpgradeable,
    ERC20BurnableUpgradeable,
    Ownable2StepUpgradeable
{
    /// @notice Name of the token.
    string public constant NAME = "Aligned Token";

    /// @notice Symbol of the token.
    string public constant SYMBOL = "ALIGN";

    /// @notice Supply of tokens for the foundation.
    uint256 public constant INITIAL_FOUNDATION_SUPPLY = 7_400_000_000e18; // 7.4 billion

    /// @notice Supply of tokens for the token distributor.
    uint256 public constant INITIAL_TOKEN_DISTRIBUTOR_SUPPLY = 2_600_000_000e18; // 2.6 billion

    /// @notice Event emitted when tokens are minted.
    /// @param to address to which the tokens are minted.
    /// @param amount amount of tokens minted.
    event TokensMinted(address indexed to, uint256 indexed amount);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract.
    /// @dev This initializer should be called only once.
    /// @param _foundation address of the foundation.
    /// @param _tokenDistributor address of the token distributor. This is the address
    /// that will give the tokens to the users that claim them.
    function initialize(
        address _foundation,
        address _tokenDistributor
    ) external initializer {
        require(
            _foundation != address(0) && _tokenDistributor != address(0),
            "Invalid _foundation or _tokenDistributor"
        );
        __ERC20_init(NAME, SYMBOL);
        __ERC20Permit_init(NAME);
        __ERC20Burnable_init();
        __Ownable_init(_foundation);
        _mint(_foundation, INITIAL_FOUNDATION_SUPPLY);
        _mint(_tokenDistributor, INITIAL_TOKEN_DISTRIBUTOR_SUPPLY);
    }

    /// @notice Mints `amount` of tokens.
    /// @param to address to which the tokens will be minted.
    /// @param amount amount of tokens to mint.
    function mint(address to, uint256 amount) external onlyOwner {
        _mint(to, amount);
        emit TokensMinted(to, amount);
    }

    /// @notice Prevents the owner from renouncing ownership.
    function renounceOwnership() public view override onlyOwner {
        revert("Cannot renounce ownership");
    }
}
