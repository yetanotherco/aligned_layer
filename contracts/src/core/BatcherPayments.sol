pragma solidity =0.8.12;

import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin-upgrades/contracts/security/PausableUpgradeable.sol";

contract BatcherPayments is Initializable, OwnableUpgradeable, PausableUpgradeable {

    // EVENTS
    event PaymentReceived(address indexed sender, uint256 amount);
    event PaymentWithdrawn(address indexed recipient, uint256 amount);

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
        uint256 priceOfPostNewBatch // TODO this is constant gas, hardcode. also give a 10% extra.
    ) external onlyBatcher whenNotPaused {
        uint256 amountOfSubmitters = proofSubmitters.length;

        require(amountOfSubmitters > 0, "No proof submitters");

        uint32 tx_base_gas_cost = 95000; // base gas cost of this transaction
        uint16 extra_user_tx_gas_cost = 6500; // upper bound of gas cost of adding a payer

        // each user must pay its fraction of the gas cost of this transaction back to the batcher, rounded up
        // plus 10% for increments in gas price
        uint256 cost_of_this_tx = ((tx_base_gas_cost + (extra_user_tx_gas_cost * amountOfSubmitters)) * tx.gasprice * 11) / 10;

        // divide the price by the amount of submitters
        uint256 submit_price_per_proof = priceOfPostNewBatch / amountOfSubmitters;
        uint256 tx_price_per_proof = cost_of_this_tx / amountOfSubmitters;
        
        uint256 totalCostPerProof = submit_price_per_proof + tx_price_per_proof;

        // discount from each payer
        // will revert if one of them has insufficient balance
        for(uint256 i=0; i < amountOfSubmitters; i++){
            address payer = proofSubmitters[i];
            discountFromPayer(payer, totalCostPerProof);
        }

        // call alignedLayerServiceManager
        // with value to fund the task's response
        (bool success, ) = AlignedLayerServiceManager.call{value: priceOfPostNewBatch}( // TODO add payable to createNewTask in marians pr
            abi.encodeWithSignature(
                "createNewTask(bytes32,string)",
                batchMerkleRoot,
                batchDataPointer
            )
        );
        require(success, "AlignedLayerServiceManager createNewTask call failed");

        payable(BatcherWallet).transfer(cost_of_this_tx);
    }

    function withdraw(uint256 amount) external whenNotPaused {
        discountFromPayer(msg.sender, amount);
        payable(msg.sender).transfer(amount);
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    // INTERNAL FUNCTIONS
    function discountFromPayer(address payer, uint256 amount) internal {
        require(PaymentBalances[payer] >= amount, "Insufficient balance");
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