pragma solidity =0.8.12;

import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin-upgrades/contracts/security/PausableUpgradeable.sol";

contract BatcherPayments is Initializable, OwnableUpgradeable, PausableUpgradeable {

    // EVENTS
    event PaymentReceived(address indexed sender, uint256 amount);
    event FundsWithdrawn(address indexed recipient, uint256 amount);


    // STORAGE
    address public AlignedLayerServiceManager;
    address public BatcherWallet;

    mapping(address => uint256) public UserBalances;

    uint256 public THIS_TX_BASE_GAS_COST; 
    uint256 public CREATE_TASK_GAS_PRICE;
    uint256 public EXTRA_USER_TX_GAS_COST; //As we must iterate over the proofSubmitters, there is an extra gas cost per extra user

    // storage gap for upgradeability
    uint256[25] private __GAP;

    // CONSTRUCTOR & INITIALIZER
    constructor() {
        _disableInitializers();
    }
    
    function initialize (
        address _AlignedLayerServiceManager,
        address _BatcherWallet, 
        uint256 _ThisTxBaseGasCost, 
        uint256 _CreateTaskGasPrice,
        uint256 _ExtraUserTxGasCost
    ) public initializer {
        __Ownable_init(); // default is msg.sender

        AlignedLayerServiceManager = _AlignedLayerServiceManager;
        BatcherWallet = _BatcherWallet;
        THIS_TX_BASE_GAS_COST = _ThisTxBaseGasCost;
        CREATE_TASK_GAS_PRICE = _CreateTaskGasPrice;
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
        uint256 costOfRespondToTask // TODO hardcode gas cost? It is variable because of signature sdk. could have upper bound and multiply by current gas cost + x%. 
    ) external onlyBatcher whenNotPaused {
        uint256 amountOfSubmitters = proofSubmitters.length;
        require(amountOfSubmitters > 0, "No proof submitters");
        
        // each user must pay its fraction of the gas cost of this transaction back to the batcher
        // + 10% for increments in gas price
        uint256 cost_of_this_tx = ((THIS_TX_BASE_GAS_COST + CREATE_TASK_GAS_PRICE + (EXTRA_USER_TX_GAS_COST * amountOfSubmitters)) * tx.gasprice * 11) / 10;

        // divide the price by the amount of submitters
        uint256 totalCostPerProof = (costOfRespondToTask + cost_of_this_tx) / amountOfSubmitters;

        // discount from each payer
        // will revert if one of them has insufficient balance
        for(uint256 i=0; i < amountOfSubmitters; i++){
            address payer = proofSubmitters[i];
            require(UserBalances[payer] >= totalCostPerProof, "Payer has insufficient balance");
            UserBalances[payer] -= totalCostPerProof;
        }

        // call alignedLayerServiceManager
        // with value to fund the task's response
        (bool success, ) = AlignedLayerServiceManager.call{value: costOfRespondToTask}(
            abi.encodeWithSignature(
                "createNewTask(bytes32,string)",
                batchMerkleRoot,
                batchDataPointer
            )
        );

        require(success, "createNewTask call failed");

        payable(BatcherWallet).transfer(cost_of_this_tx);
    }

    function withdraw(uint256 amount) external whenNotPaused {
        require(UserBalances[msg.sender] >= amount, "Payer has insufficient balance");
        UserBalances[msg.sender] -= amount;
        payable(msg.sender).transfer(amount);
        emit FundsWithdrawn(msg.sender, amount);
    }

    function setThisTxBaseGasCost(uint256 amount) external onlyOwner whenNotPaused () {
        THIS_TX_BASE_GAS_COST = amount;
    }

    function setCreateTaskGasPrice(uint256 amount) external onlyOwner whenNotPaused () {
        CREATE_TASK_GAS_PRICE = amount;
    }

    function setExtraUserTxGasCost(uint256 amount) external onlyOwner whenNotPaused () {
        EXTRA_USER_TX_GAS_COST = amount;
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    // MODIFIERS
    modifier onlyBatcher() {
        require(
            msg.sender == BatcherWallet,
            "Only Batcher can call this function"
        );
        _;
    }
}
