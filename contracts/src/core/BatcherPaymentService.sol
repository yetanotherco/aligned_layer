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

    mapping(uint256 => bool) public BatchWasSubmitted;

    // storage gap for upgradeability
    uint256[25] private __GAP;

    struct ProofSubmitterData {
        //user signs batch_id + merkle_root + amount_of_proofs_in_batch
        uint256 amount_of_proofs_in_batch;
        //signature:
        uint8 v;
        bytes32 r;
        bytes32 s;
    }

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
        uint256 batchId,
        bytes32 batchMerkleRoot,
        ProofSubmitterData[] calldata proofSubmitters, // one address for each payer proof, 1 user has 2 proofs? send twice that address
        string calldata batchDataPointer,
        uint256 gasForAggregator,
        uint256 gasPerProof
    ) external onlyBatcher whenNotPaused {
        uint256 feeForAggregator = gasForAggregator * tx.gasprice;
        uint256 feePerProof = gasPerProof * tx.gasprice;

        uint256 amountOfSubmitters = proofSubmitters.length;

        require(amountOfSubmitters > 0, "No proof submitters");
        require(BatchWasSubmitted(batchId) == false, "Batch already submitted"); // stops exploit of batcher making a user sign many times the same batch. only one of those proofs can be submitted

        // This check was moved to after the for loop below, since 
        // can't be exactly the same because new struct enables 1 submitter to have >1 proofs.
        // require(feePerProof * amountOfSubmitters > feeForAggregator, "Not enough gas to pay the batcher");

        // discount from each payer
        // will revert if one of them has insufficient balance
        ProofSubmitterData payerData;
        address signer;
        bytes32 hash_of_message;
        uint256 totalFee = 0;
        for (uint256 i = 0; i < amountOfSubmitters; i++) {
            payerData = proofSubmitters[i];
            hash_of_message = keccak256(abi.encodePacked(batchId, batchMerkleRoot, payerData.amount_of_proofs_in_batch));
            // If user signed for another batchId, or another batchMerkleRoot, or another amount_of_proofs_in_batch, it would have a different signer.
            // If wrong data was proportioned, it would have a random signer, and it won't have balance, because you can't precompute to get a desired signer. % of getting a signer with funds is almost 0.
            // Because of this, I don't think we need to compare with an "expected signer"
            signer = ecrecover(hash_of_message, payerData.v, payerData.r, payerData.s);
            require(
                UserBalances[signer] >= (feePerProof * payerData.amount_of_proofs_in_batch,
                "Payer has insufficient balance"
            );
            UserBalances[payer] -= feePerProof * payerData.amount_of_proofs_in_batch;
            totalFee += feePerProof * payerData.amount_of_proofs_in_batch; // accum of total fee
        }

        require(totalFee > feeForAggregator, "Not enough fee for aggregator and batcher");

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

        uint256 feeForBatcher = (totalFee) - feeForAggregator;

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
}
