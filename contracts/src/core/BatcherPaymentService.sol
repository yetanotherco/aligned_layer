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
        address _BatcherWallet,
        uint256 _PaymentServiceCreateTaskGasCost,
        uint256 _ServiceManagerCreateTaskGasCost,
        uint256 _ExtraUserTxGasCost
    ) public initializer {
        __Ownable_init(); // default is msg.sender
        __UUPSUpgradeable_init();
        _transferOwnership(_BatcherPaymentServiceOwner);

        AlignedLayerServiceManager = _AlignedLayerServiceManager;
        BatcherWallet = _BatcherWallet;
        PAYMENT_SERVICE_CREATE_TASK_GAS_COST = _PaymentServiceCreateTaskGasCost;
        SERVICE_MANAGER_CREATE_TASK_GAS_COST = _ServiceManagerCreateTaskGasCost;
        EXTRA_USER_TX_GAS_COST = _ExtraUserTxGasCost;
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

        require(feePerProof * amountOfSubmitters > feeForAggregator, "Not enough gas to pay the batcher")

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

    function setPaymentServiceCreateTaskGasCost(
        uint256 amount
    ) external onlyOwner whenNotPaused {
        PAYMENT_SERVICE_CREATE_TASK_GAS_COST = amount;
    }

    function setServiceManagerCreateTaskGasCost(
        uint256 amount
    ) external onlyOwner whenNotPaused {
        SERVICE_MANAGER_CREATE_TASK_GAS_COST = amount;
    }

    function setExtraUserTxGasCost(
        uint256 amount
    ) external onlyOwner whenNotPaused {
        EXTRA_USER_TX_GAS_COST = amount;
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
