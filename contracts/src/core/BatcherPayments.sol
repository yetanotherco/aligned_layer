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
        __Ownable_init();

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
        uint256 priceOfPostNewBatch
    ) external onlyBatcher whenNotPaused {

        uint256 initialGasLeft = gasleft();

        uint256 amountOfSubmitters = proofSubmitters.length;

        require(amountOfSubmitters > 0, "No proof submitters");

        // divide the price of the task by the amount of submitters, rounding up
        uint256 pricePerProof = priceOfPostNewBatch / amountOfSubmitters + (priceOfPostNewBatch % amountOfSubmitters == 0 ? 0 : 1);
        
        // discount from each payer
        // will revert if one of them has insufficient balance
        for(uint256 i=0; i < proofSubmitters.length; i++){
            address payer = proofSubmitters[i];
            discountFromPayer(payer, pricePerProof);
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

        // Calculate estimation of gas used
        // and send transaction cost back to Batcher
        uint256 finalGasLeft = gasleft();

        uint256 txCost = ((initialGasLeft - finalGasLeft + 2300) * tx.gasprice); // TODO where is this money coming from? should discount again from users

        payable(BatcherWallet).transfer(txCost);

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