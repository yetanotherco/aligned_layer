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
    // EVENTS
    event PaymentReceived(address indexed sender, uint256 amount);
    event FundsWithdrawn(address indexed recipient, uint256 amount);

    struct SignatureData {
        uint8 v;
        bytes32 r;
        bytes32 s;
        uint256 nonce;
    }

    // STORAGE
    address public AlignedLayerServiceManager;
    address public BatcherWallet;

    mapping(address => uint256) public UserBalances;

    // map to check signature is only submitted once
    mapping(address => uint256) public UserNonces;

    // storage gap for upgradeability
    uint256[25] private __GAP;

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
        UserBalances[msg.sender] += msg.value;
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

        checkMerkleRootAndVerifySignatures(
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

    function withdraw(uint256 amount) external whenNotPaused {
        require(
            UserBalances[msg.sender] >= amount,
            "Payer has insufficient balance"
        );
        UserBalances[msg.sender] -= amount;
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

    function checkMerkleRootAndVerifySignatures(
        bytes32[] calldata leaves,
        bytes32 batchMerkleRoot,
        SignatureData[] calldata signatures,
        uint256 feePerProof
    ) public {
        uint256 numNodesInLayer = leaves.length / 2;
        bytes32[] memory layer = new bytes32[](numNodesInLayer);

        uint32 i = 0;

        // Calculate the hash of the next layer of the Merkle tree
        // and verify the signatures up to numNodesInLayer
        for (i = 0; i < numNodesInLayer; i++) {
            layer[i] = keccak256(
                abi.encodePacked(leaves[2 * i], leaves[2 * i + 1])
            );

            verifySignatureAndNonce(leaves[i], signatures[i], feePerProof);
        }

        // Verify the rest of the signatures
        for (; i < signatures.length; i++) {
            verifySignatureAndNonce(leaves[i], signatures[i], feePerProof);
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

        require(layer[0] == batchMerkleRoot, "Invalid merkle root");
    }

    function verifySignatureAndNonce(
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

        require(UserNonces[signer] == signatureData.nonce, "Invalid Nonce");
        UserNonces[signer]++;

        require(
            UserBalances[signer] >= feePerProof,
            "Signer has insufficient balance"
        );
        UserBalances[signer] -= feePerProof;
    }
}
