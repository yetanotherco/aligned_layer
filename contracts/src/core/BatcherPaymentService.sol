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

    // STORAGE
    address public AlignedLayerServiceManager;
    address public BatcherWallet;

    mapping(address => uint256) public UserBalances;

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
        address[] calldata proofSubmitters, // one address for each payer proof, 1 user has 2 proofs? send twice that address
        uint256 gasForAggregator,
        uint256 gasPerProof
    ) external onlyBatcher whenNotPaused {
        uint256 feeForAggregator = gasForAggregator * tx.gasprice;
        uint256 feePerProof = gasPerProof * tx.gasprice;

        uint256 amountOfSubmitters = proofSubmitters.length;

        require(amountOfSubmitters > 0, "No proof submitters");

        require(feePerProof * amountOfSubmitters > feeForAggregator, "Not enough gas to pay the batcher");

        // discount from each payer
        // will revert if one of them has insufficient balance
        for (uint256 i = 0; i < amountOfSubmitters; i++) {
            address payer = proofSubmitters[i];
            require(
                UserBalances[payer] >= feePerProof,
                "Payer has insufficient balance"
            );
            UserBalances[payer] -= feePerProof;
        }

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

        uint256 feeForBatcher = (feePerProof * amountOfSubmitters) - feeForAggregator;

        payable(BatcherWallet).transfer(feeForBatcher);
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

    // Chores of 555-
    function calculateMerkleRoot(bytes32[] calldata leaves) public pure returns (bytes32) {
        //there are half as many nodes in the layer above the leaves
        uint256 numNodesInLayer = leaves.length / 2;
        //create a layer to store the internal nodes
        bytes32[] memory layer = new bytes32[](numNodesInLayer);
        //fill the layer with the pairwise hashes of the leaves
        for (uint256 i = 0; i < numNodesInLayer; i++) {
            layer[i] = keccak256(abi.encodePacked(leaves[2 * i], leaves[2 * i + 1]));
        }
        //the next layer above has half as many nodes
        numNodesInLayer /= 2;
        //while we haven't computed the root
        while (numNodesInLayer != 0) {
            //overwrite the first numNodesInLayer nodes in layer with the pairwise hashes of their children
            for (uint256 i = 0; i < numNodesInLayer; i++) {
                layer[i] = keccak256(abi.encodePacked(layer[2 * i], layer[2 * i + 1]));
            }
            //the next layer above has half as many nodes
            numNodesInLayer /= 2;
        }
        //the first node in the layer is the root
        return layer[0];
    }

    function verifySignatures(bytes32[] calldata msgHashes, bytes32[] calldata r, bytes32[] calldata s, uint8[] calldata v) public pure {
        for (uint256 i = 0; i < msgHashes.length; i++) {
            address signer = ecrecover(msgHashes[i], v[i], r[i], s[i]);
        }
    }
}

// cast send 0x7969c5eD335650692Bc04293B07F5BF2e7A673C0 "verifySignatures(bytes32[],bytes32[],bytes32[],uint8[])" \
// "[0x5a843f6bc5c050067cae5625d51fbd9fb53adad732da202c7502bf1e23d4efeb,0x5a843f6bc5c050067cae5625d51fbd9fb53adad732da202c7502bf1e23d4efeb,0x5a843f6bc5c050067cae5625d51fbd9fb53adad732da202c7502bf1e23d4efeb,0x5a843f6bc5c050067cae5625d51fbd9fb53adad732da202c7502bf1e23d4efeb]" \
// "[0xfc0e029250892062253ccc7634cd870021ed5a2c2e52889d57985012af3cdd22,0xfc0e029250892062253ccc7634cd870021ed5a2c2e52889d57985012af3cdd22,0xfc0e029250892062253ccc7634cd870021ed5a2c2e52889d57985012af3cdd22,0xfc0e029250892062253ccc7634cd870021ed5a2c2e52889d57985012af3cdd22]" \
// "[0x1816aff451979c4d7a7915587030ce9e852e59f23acce859b6b2b9836fa72e0b,0x1816aff451979c4d7a7915587030ce9e852e59f23acce859b6b2b9836fa72e0b,0x1816aff451979c4d7a7915587030ce9e852e59f23acce859b6b2b9836fa72e0b,0x1816aff451979c4d7a7915587030ce9e852e59f23acce859b6b2b9836fa72e0b]" \
// "[0x1c,0x1c,0x1c,0x1c]" \
// --private-key 0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba