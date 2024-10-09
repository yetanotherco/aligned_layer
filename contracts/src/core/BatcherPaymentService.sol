pragma solidity ^0.8.12;

import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin-upgrades/contracts/security/PausableUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin-upgrades/contracts/proxy/utils/UUPSUpgradeable.sol";
import {IAlignedLayerServiceManager} from "./IAlignedLayerServiceManager.sol";
import {BatcherPaymentServiceStorage} from "./BatcherPaymentServiceStorage.sol";

contract BatcherPaymentService is
    Initializable,
    OwnableUpgradeable,
    PausableUpgradeable,
    UUPSUpgradeable,
    BatcherPaymentServiceStorage
{
    // CONSTANTS = 100 Blocks * 12 second block time.
    uint256 public constant UNLOCK_BLOCK_TIME = 3600 seconds;

    // EVENTS
    event PaymentReceived(address indexed sender, uint256 amount);
    event FundsWithdrawn(address indexed recipient, uint256 amount);
    event BalanceLocked(address indexed user);
    event BalanceUnlocked(address indexed user, uint256 unlockBlockTime);
    event TaskCreated(bytes32 indexed batchMerkleRoot, uint256 feePerProof);

    // ERRORS
    error OnlyBatcherAllowed(address caller); // 152bc288
    error NoProofSubmitters(); // c43ac290
    error NoFeePerProof(); // a3a8658a
    error InsufficientFeeForAggregator(uint256 required, uint256 available); // 7899ec71
    error UserHasNoFundsToUnlock(address user); // b38340cf
    error UserHasNoFundsToLock(address user); // 6cc12bc2
    error PayerInsufficientBalance(uint256 balance, uint256 amount); // 21c3d50f
    error FundsLocked(uint256 unlockBlockTime, uint256 currentBlockTime); // bedc4e5a
    error SubmissionInsufficientBalance(
        address sender,
        uint256 balance,
        uint256 required
    ); // 4f779ceb
    error InvalidAddress(string param); // 161eb542

    // CONSTRUCTOR & INITIALIZER
    constructor() {
        _disableInitializers();
    }

    // MODIFIERS
    modifier onlyBatcher() {
        if (msg.sender != batcherWallet) {
            revert OnlyBatcherAllowed(msg.sender);
        }
        _;
    }

    function initialize(
        IAlignedLayerServiceManager _alignedLayerServiceManager,
        address _batcherPaymentServiceOwner,
        address _batcherWallet
    ) public initializer {
        if (address(_alignedLayerServiceManager) == address(0)) {
            revert InvalidAddress("alignedServiceManager");
        }
        if (_batcherPaymentServiceOwner == address(0)) {
            revert InvalidAddress("batcherPaymentServiceOwner");
        }
        if (_batcherWallet == address(0)) {
            revert InvalidAddress("batcherWallet");
        }
        __Ownable_init(); // default is msg.sender
        __UUPSUpgradeable_init();
        __Pausable_init();
        _transferOwnership(_batcherPaymentServiceOwner);

        alignedLayerServiceManager = _alignedLayerServiceManager;
        batcherWallet = _batcherWallet;
    }

    // PAYABLE FUNCTIONS
    receive() external payable {
        userData[msg.sender].balance += msg.value;
        userData[msg.sender].unlockBlockTime = 0;
        emit PaymentReceived(msg.sender, msg.value);
    }

    // PUBLIC FUNCTIONS
    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer,
        address[] calldata proofSubmitters, 
        uint256 feeForAggregator,
        uint256 feePerProof,
        uint256 respondToTaskFeeLimit
    ) external onlyBatcher whenNotPaused {
        uint256 proofSubmittersQty = proofSubmitters.length;

        if (proofSubmittersQty == 0) {
            revert NoProofSubmitters();
        }

        if (feePerProof == 0) {
            revert NoFeePerProof();
        }

        if (feePerProof * proofSubmittersQty <= feeForAggregator) {
            revert InsufficientFeeForAggregator(
                feeForAggregator,
                feePerProof * proofSubmittersQty 
            );
        }

        // decrease user balances
        for (uint32 i = 0; i < proofSubmittersQty; i++) {
            address proofSubmitter = proofSubmitters[i]; 
            UserInfo storage user = userData[proofSubmitter];

            // if one user does not have enough balance, the whole batch fails
            if (user.balance < feePerProof) {
                revert SubmissionInsufficientBalance(proofSubmitter, user.balance, feePerProof);
            }

            user.nonce++;
    
            user.balance -= feePerProof;
        }

        // call alignedLayerServiceManager
        // with value to fund the task's response
        alignedLayerServiceManager.createNewTask{value: feeForAggregator}(
            batchMerkleRoot,
            batchDataPointer,
            respondToTaskFeeLimit
        );

        emit TaskCreated(batchMerkleRoot, feePerProof);

        payable(batcherWallet).transfer(
            (feePerProof * proofSubmittersQty) - feeForAggregator
        );
    }

    function unlock() external whenNotPaused {
        if (userData[msg.sender].balance == 0) {
            revert UserHasNoFundsToUnlock(msg.sender);
        }

        userData[msg.sender].unlockBlockTime =
            block.timestamp +
            UNLOCK_BLOCK_TIME;
        emit BalanceUnlocked(msg.sender, userData[msg.sender].unlockBlockTime);
    }

    function lock() external whenNotPaused {
        if (userData[msg.sender].balance == 0) {
            revert UserHasNoFundsToLock(msg.sender);
        }
        userData[msg.sender].unlockBlockTime = 0;
        emit BalanceLocked(msg.sender);
    }

    function withdraw(uint256 amount) external whenNotPaused {
        UserInfo storage senderData = userData[msg.sender];
        if (senderData.balance < amount) {
            revert PayerInsufficientBalance(senderData.balance, amount);
        }

        if (
            senderData.unlockBlockTime == 0 ||
            senderData.unlockBlockTime > block.timestamp
        ) {
            revert FundsLocked(senderData.unlockBlockTime, block.timestamp);
        }

        senderData.balance -= amount;
        senderData.unlockBlockTime = 0;
        emit BalanceLocked(msg.sender);
        payable(msg.sender).transfer(amount);
        emit FundsWithdrawn(msg.sender, amount);
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    function _authorizeUpgrade(
        address newImplementation
    )
        internal
        override
        onlyOwner // solhint-disable-next-line no-empty-blocks
    {}

    function user_balances(address account) public view returns (uint256) {
        return userData[account].balance;
    }

    function user_nonces(address account) public view returns (uint256) {
        return userData[account].nonce;
    }

    function user_unlock_block(address account) public view returns (uint256) {
        return userData[account].unlockBlockTime;
    }
}
