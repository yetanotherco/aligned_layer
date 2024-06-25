pragma solidity =0.8.12;

import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin-upgrades/contracts/security/PausableUpgradeable.sol";

contract BatcherPayments is Initializable, OwnableUpgradeable, PausableUpgradeable {

    // EVENTS
    event PaymentReceived(address indexed sender, uint256 amount);
    event PaymentWithdrawn(address indexed recipient, uint256 amount);
    event CreatedNewTask(
        bytes32 batchMerkleRoot,
        string batchDataPointer,
        address[] proofSubmitters,
        uint256 costOfRespondToTask
    );

    // STORAGE
    address public AlignedLayerServiceManager;
    address public BatcherWallet;

    mapping(address => uint256) public PaymentBalances;

    // storage gap for upgradeability
    uint256[25] private __GAP;

    // CONSTRUCTOR & INITIALIZER
    constructor() {
        _disableInitializers();
    }
    function initialize (address _AlignedLayerServiceManager, address _BatcherWallet) public initializer {
        __Ownable_init(); // default is msg.sender

        AlignedLayerServiceManager = _AlignedLayerServiceManager;
        BatcherWallet = _BatcherWallet;
    }

    // PAYABLE FUNCTIONS
    receive() external payable {
        PaymentBalances[msg.sender] += msg.value;
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
        
        uint256 this_tx_base_gas_cost = 42000; // base gas cost of this transaction, without createNewTask or users to iterate
        uint256 create_task_gas_price = 60000; // gas price of createNewTask in in AlignedLayerServiceManager
        // uint256 respond_task_gas_price = 250000; // gas price of respondToTask in in AlignedLayerServiceManager
        uint16 extra_user_tx_gas_cost = 6500; // upper bound of gas cost of adding a user

        // each user must pay its fraction of the gas cost of this transaction back to the batcher
        // + 10% for increments in gas price
        uint256 cost_of_this_tx = ((this_tx_base_gas_cost + create_task_gas_price + (extra_user_tx_gas_cost * amountOfSubmitters)) * tx.gasprice * 11) / 10;

        // divide the price by the amount of submitters
        uint256 totalCostPerProof = (costOfRespondToTask + cost_of_this_tx) / amountOfSubmitters;

        // discount from each payer
        // will revert if one of them has insufficient balance
        for(uint256 i=0; i < amountOfSubmitters; i++){
            address payer = proofSubmitters[i];
            discountFromPayer(payer, totalCostPerProof);
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

        emit CreatedNewTask(batchMerkleRoot, batchDataPointer, proofSubmitters, costOfRespondToTask);
    }

    function withdraw(uint256 amount) external whenNotPaused {
        discountFromPayer(msg.sender, amount);
        payable(msg.sender).transfer(amount);
        emit PaymentWithdrawn(msg.sender, amount);
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    // INTERNAL FUNCTIONS
    function discountFromPayer(address payer, uint256 amount) internal {
        require(PaymentBalances[payer] >= amount, "Payer has insufficient balance");
        PaymentBalances[payer] -= amount;
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