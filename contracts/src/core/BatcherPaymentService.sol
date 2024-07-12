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
        bytes32 r;
        bytes32 s;
        uint8 v;
    }

    // STORAGE
    address public AlignedLayerServiceManager;
    address public BatcherWallet;

    mapping(address => uint256) public UserBalances;

    // map to check signature is only submitted once
    mapping(bytes32 => bool) public submittedSignatures;

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
        SignatureData[] calldata signatures, // keep actual length
        uint256 gasForAggregator,
        uint256 gasPerProof
    ) external onlyBatcher whenNotPaused {
        uint256 leavesQty = leaves.length;
        uint256 signaturesQty = signatures.length;

        uint256 feeForAggregator = gasForAggregator * tx.gasprice;
        uint256 feePerProof = gasPerProof * tx.gasprice;

        require(leavesQty > 0, "No leaves submitted");
        require(signaturesQty > 0, "No proof submitter signatures");
        require(leavesQty >= signatures.length, "Not enough leaves");
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

        checkMerkleRoot(leaves, batchMerkleRoot);
        verifySignatures(leaves, signatures, feePerProof);

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

    function checkMerkleRoot(
        bytes32[] calldata leaves,
        bytes32 batchMerkleRoot
    ) public pure {
        //there are half as many nodes in the layer above the leaves
        uint256 numNodesInLayer = leaves.length / 2;
        //create a layer to store the internal nodes
        bytes32[] memory layer = new bytes32[](numNodesInLayer);
        //fill the layer with the pairwise hashes of the leaves
        for (uint256 i = 0; i < numNodesInLayer; i++) {
            layer[i] = keccak256(
                abi.encodePacked(leaves[2 * i], leaves[2 * i + 1])
            );
        }
        //the next layer above has half as many nodes
        numNodesInLayer /= 2;
        //while we haven't computed the root
        while (numNodesInLayer != 0) {
            //overwrite the first numNodesInLayer nodes in layer with the pairwise hashes of their children
            for (uint256 i = 0; i < numNodesInLayer; i++) {
                layer[i] = keccak256(
                    abi.encodePacked(layer[2 * i], layer[2 * i + 1])
                );
            }
            //the next layer above has half as many nodes
            numNodesInLayer /= 2;
        }

        //the first node in the layer is the root
        require(layer[0] == batchMerkleRoot, "Invalid merkle root");
    }

    function verifySignatures(
        bytes32[] calldata hashes, // merkle tree leaves
        SignatureData[] calldata signatures,
        uint256 feePerProof
    ) private {
        address signer;
        for (uint256 i = 0; i < signatures.length; i++) {
            require(
                !submittedSignatures[hashes[i]],
                "Signature already submitted"
            );

            submittedSignatures[hashes[i]] = true;

            signer = ecrecover(
                hashes[i],
                signatures[i].v,
                signatures[i].r,
                signatures[i].s
            );
            require(
                UserBalances[signer] >= feePerProof,
                "Payer has insufficient balance"
            );
            UserBalances[signer] -= feePerProof;
        }
    }
}
