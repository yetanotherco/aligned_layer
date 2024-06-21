pragma solidity =0.8.12;

// import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
// import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
// import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";

// contract BatcherPayments is Initializable, OwnableUpgradeable, PausableUpgradeable {
contract BatcherPayments {

    // EVENTS
    event PaymentReceived(address indexed sender, uint256 amount);
    event PaymentWithdrawn(address indexed recipient, uint256 amount);

    // // STATE VARIABLES
    address public AlignedLayerServiceManager;
    address public BatcherWallet;

    mapping(address => uint256) public PaymentBalances;

    // CONSTRUCTOR
    constructor(address _AlignedLayerServiceManager, address _BatcherWallet) {
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
        address[] calldata proofSubmitters, // one address for each payer proof, 1 user 2 proofs? pass twice that address
        uint256 pricePerProof
    ) external payable onlyBatcher {
        
        // discount from each payer
        // will revert if one of them has insufficient balance
        for(uint256 i=0; i < proofSubmitters.length; i++){
            address payer = proofSubmitters[i];
            discountFromPayer(payer, pricePerProof);
        }

        // call alignedLayerServiceManager
        // with value to fund the task's response
        (bool success, ) = AlignedLayerServiceManager.call{value: msg.value}(
            abi.encodeWithSignature(
                "createNewTask(bytes32,string)",
                batchMerkleRoot,
                batchDataPointer
            )
        );
        require(success, "AlignedLayerServiceManager createNewTask call failed");
    }

    function withdraw(uint256 amount) external {
        discountFromPayer(msg.sender, amount);
        payable(msg.sender).transfer(amount);
    }

    // function pause() public onlyOwner {
    //     _pause();
    // }

    // function unpause() public onlyOwner {
    //     _unpause();
    // }

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