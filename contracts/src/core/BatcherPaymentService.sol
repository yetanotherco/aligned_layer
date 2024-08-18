pragma solidity =0.8.12;

import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin-upgrades/contracts/security/PausableUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin-upgrades/contracts/proxy/utils/UUPSUpgradeable.sol";

contract BatcherPaymentService is
    Initializable,
    OwnableUpgradeable,
    PausableUpgradeable,
    UUPSUpgradeable
{
    // CONSTANTS
    uint256 public constant UNLOCK_BLOCK_COUNT = 100;

    // EVENTS
    event PaymentReceived(address indexed sender, uint256 amount);
    event FundsWithdrawn(address indexed recipient, uint256 amount);

    struct SignatureData {
        uint8 v;
        bytes32 r;
        bytes32 s;
        uint256 nonce;
    }

    struct UserInfo {
        uint256 balance;
        uint256 unlockBlock;
        uint256 nonce;
    }

    // STORAGE
    address public AlignedLayerServiceManager;
    address public BatcherWallet;

    // map to user data
    mapping(address => UserInfo) public UserData;

    // storage gap for upgradeability
    uint256[24] private __GAP;

    // CONSTRUCTOR & INITIALIZER
    constructor() {
        _disableInitializers();
    }

    function initialize(
        address _AlignedLayerServiceManager,
        address _BatcherPaymentServiceOwner,
        address _BatcherWallet
    ) public initializer {
        __Ownable_init(); // default is msg.sender
        __UUPSUpgradeable_init();
        _transferOwnership(_BatcherPaymentServiceOwner);

        AlignedLayerServiceManager = _AlignedLayerServiceManager;
        BatcherWallet = _BatcherWallet;
    }

    // PAYABLE FUNCTIONS
    receive() external payable {
        UserData[msg.sender].balance += msg.value;
        emit PaymentReceived(msg.sender, msg.value);
    }

    // PUBLIC FUNCTIONS
    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer,
        bytes32[] calldata leaves, // padded to the next power of 2
        SignatureData[] calldata signatures, // actual length (proof sumbitters == proofs submitted)
        uint256 gasForAggregator,
        uint256 gasPerProof
    ) external onlyBatcher whenNotPaused {
        uint256 leavesQty = leaves.length;
        uint256 signaturesQty = signatures.length;

        uint256 feeForAggregator = gasForAggregator * tx.gasprice;
        uint256 feePerProof = gasPerProof * tx.gasprice;

        require(leavesQty > 0, "No leaves submitted");
        require(signaturesQty > 0, "No proof submitter signatures");
        require(leavesQty >= signaturesQty, "Not enough leaves");
        require(
            (leavesQty & (leavesQty - 1)) == 0,
            "Leaves length is not a power of 2"
        );

        require(feeForAggregator > 0, "No gas for aggregator");
        require(feePerProof > 0, "No gas per proof");
        require(
            feePerProof * signaturesQty > feeForAggregator,
            "Not enough gas to pay the aggregator"
        );

        _checkMerkleRootAndVerifySignatures(
            leaves,
            batchMerkleRoot,
            signatures,
            feePerProof
        );

        // call alignedLayerServiceManager
        // with value to fund the task's response
        (bool success, ) = AlignedLayerServiceManager.call{
            value: feeForAggregator
        }(
            abi.encodeWithSignature(
                "createNewTask(bytes32,string)",
                batchMerkleRoot,
                batchDataPointer
            )
        );

        require(success, "createNewTask call failed");

        payable(BatcherWallet).transfer(
            (feePerProof * signaturesQty) - feeForAggregator
        );
    }

    function unlock() external whenNotPaused {
        require(
            UserData[msg.sender].balance > 0,
            "User has no funds to unlock"
        );

        UserData[msg.sender].unlockBlock = block.number + UNLOCK_BLOCK_COUNT;
    }

    function lock() external whenNotPaused {
        require(UserData[msg.sender].balance > 0, "User has no funds to lock");
        UserData[msg.sender].unlockBlock = 0;
    }

    function withdraw(uint256 amount) external whenNotPaused {
        UserInfo storage user_data = UserData[msg.sender];
        require(user_data.balance >= amount, "Payer has insufficient balance");

        require(
            user_data.unlockBlock != 0 && user_data.unlockBlock <= block.number,
            "Funds are locked"
        );

        user_data.balance -= amount;
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
    ) internal override onlyOwner {}

    // MODIFIERS
    modifier onlyBatcher() {
        require(
            msg.sender == BatcherWallet,
            "Only Batcher can call this function"
        );
        _;
    }

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

            verifySignatureAndDecreaseBalance(
                leaves[i],
                signatures[i],
                feePerProof
            );
        }

        // Verify the rest of the signatures
        for (; i < signatures.length; i++) {
            verifySignatureAndDecreaseBalance(
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
            require(leaves[0] == batchMerkleRoot, "Invalid merkle root");
        } else {
            require(layer[0] == batchMerkleRoot, "Invalid merkle root");
        }
    }

    function verifySignatureAndDecreaseBalance(
        bytes32 hash,
        SignatureData calldata signatureData,
        uint256 feePerProof
    ) private {
        bytes32 noncedHash = keccak256(
            abi.encodePacked(hash, signatureData.nonce)
        );

        address signer = ecrecover(
            noncedHash,
            signatureData.v,
            signatureData.r,
            signatureData.s
        );

        UserInfo storage user_data = UserData[signer];

        require(user_data.nonce == signatureData.nonce, "Invalid Nonce");
        user_data.nonce++;

        require(
            user_data.balance >= feePerProof,
            "Signer has insufficient balance"
        );

        user_data.balance -= feePerProof;
    }

    function user_balances(address account) public view returns (uint256) {
        return UserData[account].balance;
    }

    function user_nonces(address account) public view returns (uint256) {
        return UserData[account].nonce;
    }

    function user_unlock_block(address account) public view returns (uint256) {
        return UserData[account].unlockBlock;
    }
}
