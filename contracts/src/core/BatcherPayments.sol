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

// cast call 0x2bdCC0de6bE1f7D2ee689a0342D76F52E8EFABa3 "createNewTask(bytes32, string, address[], uint256)" 0x402e72e03fc4285de6fe4513151f15675fdccc1846d7633b11116e97be27dc37 "hola" "[0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC]" 123 --private-key 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d

    // PUBLIC FUNCTIONS
    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer,
        address[] calldata proofSubmitters, // one address for each payer proof, 1 user 2 proofs? pass twice that address
        uint256 pricePerProof
    ) external payable onlyBatcher {
        
        // discount from each payer
        // will revert if one of them has insufficient balance
        require(proofSubmitters[0] >= pricePerProof, "Insufficient balance");
        PaymentBalances[proofSubmitters[0]] -= pricePerProof
        // for(uint256 i=0; i < proofSubmitters.length; i++){
        //     address payer = proofSubmitters[i];
        //     require(PaymentBalances[payer] >= pricePerProof, "Insufficient balance");
        //     PaymentBalances[payer] -= pricePerProof;
        //     // discountFromPayer(user, pricePerProof);
        // }

        // // call alignedLayerServiceManager
        // // with value to fund the task's response
        // (bool success, ) = AlignedLayerServiceManager.call{value: msg.value}(
        //     abi.encodeWithSignature(
        //         "createNewTask(bytes32,string)",
        //         batchMerkleRoot,
        //         batchDataPointer
        //     )
        // );
        // require(success, "AlignedLayerServiceManager createNewTask call failed");
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