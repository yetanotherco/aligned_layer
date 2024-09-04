pragma solidity =0.8.12;

import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin-upgrades/contracts/security/PausableUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin-upgrades/contracts/proxy/utils/UUPSUpgradeable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {IAlignedLayerServiceManager} from "./IAlignedLayerServiceManager.sol";

contract BatcherPaymentService is
    Initializable,
    OwnableUpgradeable,
    PausableUpgradeable,
    UUPSUpgradeable
{
    using ECDSA for bytes32;

    // CONSTANTS
    uint256 public constant UNLOCK_BLOCK_COUNT = 100;

    // EVENTS
    event PaymentReceived(address indexed sender, uint256 amount);
    event FundsWithdrawn(address indexed recipient, uint256 amount);
    event BalanceLocked(address indexed user);
    event BalanceUnlocked(address indexed user, uint256 unlockBlock);
    event TaskCreated(bytes32 indexed batchMerkleRoot, uint256 feePerProof);

    // ERRORS
    error OnlyBatcherAllowed(address caller); // 152bc288
    error NoLeavesSubmitted(); // e5180e03
    error NoProofSubmitterSignatures(); // 32742c04
    error NotEnoughLeaves(uint256 leavesQty, uint256 signaturesQty); // 320f0a1b
    error LeavesNotPowerOfTwo(uint256 leavesQty); // 6b1651e1
    error NoGasForAggregator(); // ea46d6a4
    error NoGasPerProof(); // 459e386d
    error InsufficientGasForAggregator(uint256 required, uint256 available); // ca3b0e0f
    error UserHasNoFundsToUnlock(address user); // b38340cf
    error UserHasNoFundsToLock(address user); // 6cc12bc2
    error PayerInsufficientBalance(uint256 balance, uint256 amount); // 21c3d50f
    error FundsLocked(uint256 unlockBlock, uint256 currentBlock); // bedc4e5a
    error InvalidSignature(); // 8baa579f
    error InvalidNonce(uint256 expected, uint256 actual); // 06427aeb
    error InvalidMaxFee(uint256 maxFee, uint256 actualFee); // f59adf4a
    error SignerInsufficientBalance(
        address signer,
        uint256 balance,
        uint256 required
    ); // 955c0664
    error InvalidMerkleRoot(bytes32 expected, bytes32 actual); // 9f13b65c

    struct SignatureData {
        bytes signature;
        uint256 nonce;
        uint256 maxFee;
    }

    struct UserInfo {
        uint256 balance;
        uint256 unlockBlock;
        uint256 nonce;
    }

    // STORAGE
    IAlignedLayerServiceManager public alignedLayerServiceManager;

    address public batcherWallet;

    // map to user data
    mapping(address => UserInfo) public userData;

    // storage gap for upgradeability
    // solhint-disable-next-line var-name-mixedcase
    uint256[24] private __GAP;

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
        __Ownable_init(); // default is msg.sender
        __UUPSUpgradeable_init();
        _transferOwnership(_batcherPaymentServiceOwner);

        alignedLayerServiceManager = _alignedLayerServiceManager;
        batcherWallet = _batcherWallet;
    }

    // PAYABLE FUNCTIONS
    receive() external payable {
        userData[msg.sender].balance += msg.value;
        emit PaymentReceived(msg.sender, msg.value);
    }

    // PUBLIC FUNCTIONS
    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer,
        bytes32[] calldata leaves, // padded to the next power of 2
        SignatureData[] calldata signatures, // actual length (proof sumbitters == proofs submitted)
        uint256 feeForAggregator,
        uint256 feePerProof
    ) external onlyBatcher whenNotPaused {
        uint256 leavesQty = leaves.length;
        uint256 signaturesQty = signatures.length;

        if (leavesQty == 0) {
            revert NoLeavesSubmitted();
        }

        if (signaturesQty == 0) {
            revert NoProofSubmitterSignatures();
        }

        if (leavesQty < signaturesQty) {
            revert NotEnoughLeaves(leavesQty, signaturesQty);
        }

        if ((leavesQty & (leavesQty - 1)) != 0) {
            revert LeavesNotPowerOfTwo(leavesQty);
        }

        if (feeForAggregator == 0) {
            revert NoGasForAggregator();
        }

        if (feePerProof == 0) {
            revert NoGasPerProof();
        }

        if (feePerProof * signaturesQty <= feeForAggregator) {
            revert InsufficientGasForAggregator(
                feeForAggregator,
                feePerProof * signaturesQty
            );
        }

        _checkMerkleRootAndVerifySignatures(
            leaves,
            batchMerkleRoot,
            signatures,
            feePerProof
        );

        // call alignedLayerServiceManager
        // with value to fund the task's response
        alignedLayerServiceManager.createNewTask{value: feeForAggregator}(
            batchMerkleRoot,
            batchDataPointer
        );

        emit TaskCreated(batchMerkleRoot, feePerProof);

        payable(batcherWallet).transfer(
            (feePerProof * signaturesQty) - feeForAggregator
        );
    }

    function unlock() external whenNotPaused {
        if (userData[msg.sender].balance == 0) {
            revert UserHasNoFundsToUnlock(msg.sender);
        }

        userData[msg.sender].unlockBlock = block.number + UNLOCK_BLOCK_COUNT;
        emit BalanceUnlocked(msg.sender, userData[msg.sender].unlockBlock);
    }

    function lock() external whenNotPaused {
        if (userData[msg.sender].balance == 0) {
            revert UserHasNoFundsToLock(msg.sender);
        }
        userData[msg.sender].unlockBlock = 0;
        emit BalanceLocked(msg.sender);
    }

    function withdraw(uint256 amount) external whenNotPaused {
        UserInfo storage senderData = userData[msg.sender];
        if (senderData.balance < amount) {
            revert PayerInsufficientBalance(senderData.balance, amount);
        }

        if (
            senderData.unlockBlock == 0 || senderData.unlockBlock > block.number
        ) {
            revert FundsLocked(senderData.unlockBlock, block.number);
        }

        senderData.balance -= amount;
        senderData.unlockBlock = 0;
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

    function _checkMerkleRootAndVerifySignatures(
        bytes32[] calldata leaves,
        bytes32 batchMerkleRoot,
        SignatureData[] calldata signatures,
        uint256 feePerProof
    ) private {
        uint256 numNodesInLayer = leaves.length / 2;
        bytes32[] memory layer = new bytes32[](numNodesInLayer);

        uint32 i = 0;

        // Calculate the hash of the next layer of the Merkle tree
        // and verify the signatures up to numNodesInLayer
        for (i = 0; i < numNodesInLayer; i++) {
            layer[i] = keccak256(
                abi.encodePacked(leaves[2 * i], leaves[2 * i + 1])
            );

            _verifySignatureAndDecreaseBalance(
                leaves[i],
                signatures[i],
                feePerProof
            );
        }

        // Verify the rest of the signatures
        for (; i < signatures.length; i++) {
            _verifySignatureAndDecreaseBalance(
                leaves[i],
                signatures[i],
                feePerProof
            );
        }

        // The next layer above has half as many nodes
        numNodesInLayer /= 2;

        // Continue calculating Merkle root for remaining layers
        while (numNodesInLayer != 0) {
            // Overwrite the first numNodesInLayer nodes in layer with the pairwise hashes of their children
            for (i = 0; i < numNodesInLayer; i++) {
                layer[i] = keccak256(
                    abi.encodePacked(layer[2 * i], layer[2 * i + 1])
                );
            }

            // The next layer above has half as many nodes
            numNodesInLayer /= 2;
        }

        if (leaves.length == 1) {
            if (leaves[0] != batchMerkleRoot) {
                revert InvalidMerkleRoot(batchMerkleRoot, leaves[0]);
            }
        } else if (layer[0] != batchMerkleRoot) {
            revert InvalidMerkleRoot(batchMerkleRoot, layer[0]);
        }
    }

    function _verifySignatureAndDecreaseBalance(
        bytes32 hash,
        SignatureData calldata signatureData,
        uint256 feePerProof
    ) private {
        if (signatureData.maxFee < feePerProof) {
            revert InvalidMaxFee(signatureData.maxFee, feePerProof);
        }

        bytes32 noncedHash = keccak256(
            abi.encodePacked(
                hash,
                signatureData.nonce,
                signatureData.maxFee,
                block.chainid
            )
        );

        address signer = noncedHash.recover(signatureData.signature);

        if (signer == address(0)) {
            revert InvalidSignature();
        }

        UserInfo storage signerData = userData[signer];

        if (signerData.nonce != signatureData.nonce) {
            revert InvalidNonce(signerData.nonce, signatureData.nonce);
        }
        signerData.nonce++;

        if (signerData.balance < feePerProof) {
            revert SignerInsufficientBalance(
                signer,
                signerData.balance,
                feePerProof
            );
        }

        signerData.balance -= feePerProof;
    }

    function user_balances(address account) public view returns (uint256) {
        return userData[account].balance;
    }

    function user_nonces(address account) public view returns (uint256) {
        return userData[account].nonce;
    }

    function user_unlock_block(address account) public view returns (uint256) {
        return userData[account].unlockBlock;
    }
}
