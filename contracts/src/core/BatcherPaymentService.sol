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
        bytes32 r;
        bytes32 s;
        uint8 v;
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

// first
// cast keccak "0x0000000000000000000000000000000000000000000000000000000000000001aeab82486c6c23487b4c475218db19e33e88bc21543ca2f625d185fddd3d26df0000000000000000000000000000000000000000000000000000000000000001"
// : 0x5a843f6bc5c050067cae5625d51fbd9fb53adad732da202c7502bf1e23d4efeb

// then
// cast wallet sign \
// 0x5a843f6bc5c050067cae5625d51fbd9fb53adad732da202c7502bf1e23d4efeb \
// --private-key 0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a --no-hash
// : 0xfc0e029250892062253ccc7634cd870021ed5a2c2e52889d57985012af3cdd221816aff451979c4d7a7915587030ce9e852e59f23acce859b6b2b9836fa72e0b1c
// fc0e029250892062253ccc7634cd870021ed5a2c2e52889d57985012af3cdd22
// 1816aff451979c4d7a7915587030ce9e852e59f23acce859b6b2b9836fa72e0b
// 1c

//finally
// cast send 0x7969c5eD335650692Bc04293B07F5BF2e7A673C0 \
// "createNewTask(uint256,bytes32,(uint256,bytes32,bytes32,uint8)[],string,uint256,uint256)" \
// 1 0xaeab82486c6c23487b4c475218db19e33e88bc21543ca2f625d185fddd3d26df \
// "[(1,0xfc0e029250892062253ccc7634cd870021ed5a2c2e52889d57985012af3cdd22,0x1816aff451979c4d7a7915587030ce9e852e59f23acce859b6b2b9836fa72e0b,0x1c)]" \
// "http://storage.alignedlayer.com/aeab82486c6c23487b4c475218db19e33e88bc21543ca2f625d185fddd3d26df.json" 1 10 \
// --private-key 0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba


//now user 2 signature:
// 0xf0650584f85750ce5b87e52a203cfaf7ceee7e4ff7cc507a854b511998a9fede11aab3984c327498a4b271932a75035043d3b5320c7365482369637e441cca6a1c
// f0650584f85750ce5b87e52a203cfaf7ceee7e4ff7cc507a854b511998a9fede
// 11aab3984c327498a4b271932a75035043d3b5320c7365482369637e441cca6a
// 1c

// cast send 0x7969c5eD335650692Bc04293B07F5BF2e7A673C0 \
// "createNewTask(uint256,bytes32,(uint256,bytes32,bytes32,uint8)[],string,uint256,uint256)" \
// 1 0xaeab82486c6c23487b4c475218db19e33e88bc21543ca2f625d185fddd3d26df \
// "[(1,0xfc0e029250892062253ccc7634cd870021ed5a2c2e52889d57985012af3cdd22,0x1816aff451979c4d7a7915587030ce9e852e59f23acce859b6b2b9836fa72e0b,0x1c),(1,0xf0650584f85750ce5b87e52a203cfaf7ceee7e4ff7cc507a854b511998a9fede,0x11aab3984c327498a4b271932a75035043d3b5320c7365482369637e441cca6a,0x1c)]" \
// "http://storage.alignedlayer.com/aeab82486c6c23487b4c475218db19e33e88bc21543ca2f625d185fddd3d26df.json" 1 10 \
// --private-key 0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba


    // PUBLIC FUNCTIONS
    function createNewTask(
        uint256 batchId,
        bytes32 batchMerkleRoot,
        // todo is there a way to make this a set? of non repeated values
        ProofSubmitterData[] calldata proofSubmitters, // one address for each payer proof, 1 user has 2 proofs? send twice that address
        string calldata batchDataPointer,
        uint256 gasForAggregator,
        uint256 gasPerProof
    ) external onlyBatcher whenNotPaused {
        uint256 feeForAggregator = gasForAggregator * tx.gasprice;
        uint256 feePerProof = gasPerProof * tx.gasprice;

        uint256 amountOfSubmitters = proofSubmitters.length;

        // mapping(address => bool) memory userSubmittedInBatch;


        require(amountOfSubmitters > 0, "No proof submitters");
        require(BatchWasSubmitted[batchId] == false, "Batch already submitted"); // stops exploit of batcher making a user sign many times the same batch. only one of those proofs can be submitted

        // discount from each payer
        // will revert if one of them has insufficient balance
        uint256 accumulatedFee = 0;
        for (uint256 i = 0; i < amountOfSubmitters; i++) {
            ProofSubmitterData memory user = proofSubmitters[i];

            uint256 userFee = feePerProof * user.amount_of_proofs_in_batch;

            bytes32 messageHash = keccak256(abi.encodePacked(batchId, batchMerkleRoot, user.amount_of_proofs_in_batch));

            // If user signed for another batchId, or another batchMerkleRoot, or another amount_of_proofs_in_batch, it would have a different signer.
            // If wrong data was proportioned, it would have a random signer, and it won't have balance, because you can't precompute to get a desired signer. % of getting a signer with funds is almost 0.
            // Because of this, I don't think we need to compare with an "expected signer"
            address signer = ecrecover(messageHash, user.v, user.r, user.s);
            require(userSubmittedInBatch[signer] == false, "User already submitted in batch");
            userSubmittedInBatch[signer] = true;

            require(
                UserBalances[signer] >= (userFee),
                "Payer has insufficient balance"
            );
            UserBalances[signer] -= userFee;
            accumulatedFee += userFee; // accum of total fee
        }

        require(accumulatedFee > feeForAggregator, "Not enough fee for aggregator and batcher");
        
        BatchWasSubmitted[batchId] = true; // Before calling AlignedLayerServiceManager, to follow CEI, to prevent reentrancy

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

        uint256 feeForBatcher = (accumulatedFee) - feeForAggregator;

        payable(BatcherWallet).transfer(feeForBatcher);
    }



// cast send 0x7969c5eD335650692Bc04293B07F5BF2e7A673C0 \
// "createNewTask(uint256,bytes32,(uint256,uint8,bytes32,bytes32)[],string,uint256,uint256)" \
// 1 \
// 0xaeab82486c6c23487b4c475218db19e33e88bc21543ca2f625d185fddd3d26df \
// "[(1,0x1c,0x3fbde0481c48a7d9408aab36a32666aa8572cd854fb88379da71b1dda95c9593,0xcf1b9774b6c47e0f0b98c99794636f1f790feb1a6e0073dea9b64a387f783a8e)]" \
// "http://storage.alignedlayer.com/aeab82486c6c23487b4c475218db19e33e88bc21543ca2f625d185fddd3d26df.json" \
// 1 10 \
// --private-key 0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba

// cast wallet sign \
// "0000000000000000000000000000000000000000000000000000000000000001aeab82486c6c23487b4c475218db19e33e88bc21543ca2f625d185fddd3d26df0000000000000000000000000000000000000000000000000000000000000001" \
// --private-key 0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a

// signature:
// 0xa1ab85d36c4acca489cf89c512970147e3469dde7e7410bf94abf8c316f9e1b80e9c29e88dbab0d0021bd3caf03eac7ac4005f800087e60aa3975d929219b0fc1b
// :
// a1ab85d36c4acca489cf89c512970147e3469dde7e7410bf94abf8c316f9e1b8
// 0e9c29e88dbab0d0021bd3caf03eac7ac4005f800087e60aa3975d929219b0fc
// 1b

// 0x9cbb5b624b022bcfb0523d10c229c3c9ced007ac9c3aa00c627760b41d2b03fa066dc100a8b79ac78819639fd8d994aa70e161e48fa13f005d32ec77651b7f9f1b
// 9cbb5b624b022bcfb0523d10c229c3c9ced007ac9c3aa00c627760b41d2b03fa
// 066dc100a8b79ac78819639fd8d994aa70e161e48fa13f005d32ec77651b7f9f
// 1b
    function recoverSigner(uint256 batchId, bytes32 batchMerkleRoot, uint256 amount_of_proofs_in_batch, bytes32 r, bytes32 s, uint8 v) public pure returns (address) {
        bytes32 messageHash = keccak256(abi.encodePacked(batchId, batchMerkleRoot, amount_of_proofs_in_batch));
        bytes32 ethSignedMessageHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash));
        
        return ecrecover(ethSignedMessageHash, v, r, s);
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


// signature:
// d5c1b37094ae4c9204617e47b6800afc7835773c635b463f354c7c683e1fcb06
// 70b1c92f03665fcf20a94bee1e0456b475bc626ff6eec8c65695a26f4f37998b
// 1b