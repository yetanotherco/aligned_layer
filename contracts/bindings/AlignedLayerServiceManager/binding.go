// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package contractAlignedLayerServiceManager

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
	_ = abi.ConvertType
)

// BN254G1Point is an auto generated low-level Go binding around an user-defined struct.
type BN254G1Point struct {
	X *big.Int
	Y *big.Int
}

// BN254G2Point is an auto generated low-level Go binding around an user-defined struct.
type BN254G2Point struct {
	X [2]*big.Int
	Y [2]*big.Int
}

// IBLSSignatureCheckerNonSignerStakesAndSignature is an auto generated low-level Go binding around an user-defined struct.
type IBLSSignatureCheckerNonSignerStakesAndSignature struct {
	NonSignerQuorumBitmapIndices []uint32
	NonSignerPubkeys             []BN254G1Point
	QuorumApks                   []BN254G1Point
	ApkG2                        BN254G2Point
	Sigma                        BN254G1Point
	QuorumApkIndices             []uint32
	TotalStakeIndices            []uint32
	NonSignerStakeIndices        [][]uint32
}

// IBLSSignatureCheckerQuorumStakeTotals is an auto generated low-level Go binding around an user-defined struct.
type IBLSSignatureCheckerQuorumStakeTotals struct {
	SignedStakeForQuorum []*big.Int
	TotalStakeForQuorum  []*big.Int
}

// IRewardsCoordinatorRewardsSubmission is an auto generated low-level Go binding around an user-defined struct.
type IRewardsCoordinatorRewardsSubmission struct {
	StrategiesAndMultipliers []IRewardsCoordinatorStrategyAndMultiplier
	Token                    common.Address
	Amount                   *big.Int
	StartTimestamp           uint32
	Duration                 uint32
}

// IRewardsCoordinatorStrategyAndMultiplier is an auto generated low-level Go binding around an user-defined struct.
type IRewardsCoordinatorStrategyAndMultiplier struct {
	Strategy   common.Address
	Multiplier *big.Int
}

// ISignatureUtilsSignatureWithSaltAndExpiry is an auto generated low-level Go binding around an user-defined struct.
type ISignatureUtilsSignatureWithSaltAndExpiry struct {
	Signature []byte
	Salt      [32]byte
	Expiry    *big.Int
}

// ContractAlignedLayerServiceManagerMetaData contains all meta data concerning the ContractAlignedLayerServiceManager contract.
var ContractAlignedLayerServiceManagerMetaData = &bind.MetaData{
	ABI: "[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"__avsDirectory\",\"type\":\"address\",\"internalType\":\"contractIAVSDirectory\"},{\"name\":\"__rewardsCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRewardsCoordinator\"},{\"name\":\"__registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"__stakeRegistry\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"receive\",\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"alignedAggregator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"avsDirectory\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"balanceOf\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"batchesState\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"responded\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"respondToTaskFeeLimit\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsApkRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBLSApkRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkPublicInput\",\"inputs\":[{\"name\":\"publicInput\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"hash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"pure\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"params\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createAVSRewardsSubmission\",\"inputs\":[{\"name\":\"rewardsSubmissions\",\"type\":\"tuple[]\",\"internalType\":\"structIRewardsCoordinator.RewardsSubmission[]\",\"components\":[{\"name\":\"strategiesAndMultipliers\",\"type\":\"tuple[]\",\"internalType\":\"structIRewardsCoordinator.StrategyAndMultiplier[]\",\"components\":[{\"name\":\"strategy\",\"type\":\"address\",\"internalType\":\"contractIStrategy\"},{\"name\":\"multiplier\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"token\",\"type\":\"address\",\"internalType\":\"contractIERC20\"},{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"startTimestamp\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"duration\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"createNewTask\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"batchDataPointer\",\"type\":\"string\",\"internalType\":\"string\"},{\"name\":\"respondToTaskFeeLimit\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"delegation\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIDelegationManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"depositToBatcher\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"deregisterOperatorFromAVS\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"getOperatorRestakedStrategies\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getRestakeableStrategies\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_rewardsInitiator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_alignedAggregator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"initializeAggregator\",\"inputs\":[{\"name\":\"_alignedAggregator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registerOperatorToAVS\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorSignature\",\"type\":\"tuple\",\"internalType\":\"structISignatureUtils.SignatureWithSaltAndExpiry\",\"components\":[{\"name\":\"signature\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"salt\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"expiry\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"registryCoordinator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToTaskV2\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"rewardsInitiator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setAggregator\",\"inputs\":[{\"name\":\"_alignedAggregator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setRewardsInitiator\",\"inputs\":[{\"name\":\"newRewardsInitiator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setStaleStakesForbidden\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stakeRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"staleStakesForbidden\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"trySignatureAndApkVerification\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"apk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[{\"name\":\"pairingSuccessful\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"siganatureIsValid\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"updateAVSMetadataURI\",\"inputs\":[{\"name\":\"_metadataURI\",\"type\":\"string\",\"internalType\":\"string\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"verifyBatchInclusion\",\"inputs\":[{\"name\":\"proofCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pubInputCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"provingSystemAuxDataCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"proofGeneratorAddr\",\"type\":\"bytes20\",\"internalType\":\"bytes20\"},{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"merkleProof\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"verificationDataBatchIndex\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"verifyBatchInclusion\",\"inputs\":[{\"name\":\"proofCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pubInputCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"provingSystemAuxDataCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"proofGeneratorAddr\",\"type\":\"bytes20\",\"internalType\":\"bytes20\"},{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"merkleProof\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"verificationDataBatchIndex\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"withdraw\",\"inputs\":[{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"BatchVerified\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"BatcherBalanceUpdated\",\"inputs\":[{\"name\":\"batcher\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newBalance\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewBatchV2\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"batchDataPointer\",\"type\":\"string\",\"indexed\":false,\"internalType\":\"string\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewBatchV3\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"batchDataPointer\",\"type\":\"string\",\"indexed\":false,\"internalType\":\"string\"},{\"name\":\"respondToTaskFeeLimit\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RewardsInitiatorUpdated\",\"inputs\":[{\"name\":\"prevRewardsInitiator\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"newRewardsInitiator\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"StaleStakesForbiddenUpdate\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"error\",\"name\":\"BatchAlreadyResponded\",\"inputs\":[{\"name\":\"batchIdentifierHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"BatchAlreadySubmitted\",\"inputs\":[{\"name\":\"batchIdentifierHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"BatchDoesNotExist\",\"inputs\":[{\"name\":\"batchIdentifierHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"ExceededMaxRespondFee\",\"inputs\":[{\"name\":\"respondToTaskFeeLimit\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"txCost\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"InsufficientFunds\",\"inputs\":[{\"name\":\"batcher\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"required\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"available\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"InvalidDepositAmount\",\"inputs\":[{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"InvalidQuorumThreshold\",\"inputs\":[{\"name\":\"signedStake\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"requiredStake\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"SenderIsNotAggregator\",\"inputs\":[{\"name\":\"sender\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"alignedAggregator\",\"type\":\"address\",\"internalType\":\"address\"}]}]",
	Bin: "0x6101806040523480156200001257600080fd5b5060405162005673380380620056738339810160408190526200003591620002e5565b6001600160a01b0380851660805280841660a05280831660c052811660e0528184848284620000636200020a565b50505050806001600160a01b0316610100816001600160a01b031681525050806001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015620000c1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000e791906200034d565b6001600160a01b0316610120816001600160a01b031681525050806001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000140573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200016691906200034d565b6001600160a01b0316610140816001600160a01b031681525050610120516001600160a01b031663df5cf7236040518163ffffffff1660e01b8152600401602060405180830381865afa158015620001c2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001e891906200034d565b6001600160a01b03166101605250620002006200020a565b5050505062000374565b600054610100900460ff1615620002775760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff9081161015620002ca576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b0381168114620002e257600080fd5b50565b60008060008060808587031215620002fc57600080fd5b84516200030981620002cc565b60208601519094506200031c81620002cc565b60408601519093506200032f81620002cc565b60608601519092506200034281620002cc565b939692955090935050565b6000602082840312156200036057600080fd5b81516200036d81620002cc565b9392505050565b60805160a05160c05160e051610100516101205161014051610160516151f1620004826000396000818161064901526117a501526000818161038c01526119b80152600081816103c001528181611ba50152611d9501526000818161042701528181610fd70152818161146b015281816116120152611859015260008181610d0c01528181610e5d01528181610ef401528181612a7601528181612bef0152612c8e015260008181610b3301528181610bc201528181610c420152818161212e015281816121fa015281816129b10152612b4a0152600081816130450152818161310101526131e40152600081816103f1015281816121820152818161225601526122d501526151f16000f3fe6080604052600436106101f25760003560e01c80638da5cb5b1161010d578063c0c53b8b116100a0578063f2fde38b1161006f578063f2fde38b14610680578063f9120af6146106a0578063fa534dc0146106c0578063fc299dee146106e0578063fce36c7d1461070057600080fd5b8063c0c53b8b14610604578063d66eaabd14610624578063df5cf72314610637578063e481af9d1461066b57600080fd5b8063a98fb355116100dc578063a98fb35514610540578063ab21739a14610560578063b099627e14610580578063b98d0908146105ea57600080fd5b80638da5cb5b146104c257806395c6d604146104e05780639926ee7d14610500578063a364f4da1461052057600080fd5b80634ae07c37116101855780636d14a987116101545780636d14a9871461041557806370a0823114610449578063715018a61461048d578063800fb61f146104a257600080fd5b80634ae07c371461034c5780635df459461461037a57806368304835146103ae5780636b3aa72e146103e257600080fd5b80633bc28c8c116101c15780633bc28c8c146102c1578063416c7e5e146102e15780634223d551146103015780634a5bf6321461031457600080fd5b806306045a9114610208578063171f1d5b1461023d5780632e1a7d4d1461027457806333cfb7b71461029457600080fd5b36610203576102013334610720565b005b600080fd5b34801561021457600080fd5b5061022861022336600461415f565b6107b5565b60405190151581526020015b60405180910390f35b34801561024957600080fd5b5061025d6102583660046142b1565b6108ac565b604080519215158352901515602083015201610234565b34801561028057600080fd5b5061020161028f366004614302565b610a36565b3480156102a057600080fd5b506102b46102af36600461431b565b610b0e565b6040516102349190614338565b3480156102cd57600080fd5b506102016102dc36600461431b565b610fc1565b3480156102ed57600080fd5b506102016102fc366004614393565b610fd5565b61020161030f36600461431b565b61110c565b34801561032057600080fd5b5060cb54610334906001600160a01b031681565b6040516001600160a01b039091168152602001610234565b34801561035857600080fd5b5061036c61036736600461466e565b611116565b604051610234929190614709565b34801561038657600080fd5b506103347f000000000000000000000000000000000000000000000000000000000000000081565b3480156103ba57600080fd5b506103347f000000000000000000000000000000000000000000000000000000000000000081565b3480156103ee57600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610334565b34801561042157600080fd5b506103347f000000000000000000000000000000000000000000000000000000000000000081565b34801561045557600080fd5b5061047f61046436600461431b565b6001600160a01b0316600090815260ca602052604090205490565b604051908152602001610234565b34801561049957600080fd5b5061020161204a565b3480156104ae57600080fd5b506102016104bd36600461431b565b61205e565b3480156104ce57600080fd5b506033546001600160a01b0316610334565b3480156104ec57600080fd5b506102286104fb36600461479a565b6120fe565b34801561050c57600080fd5b5061020161051b3660046147e5565b612123565b34801561052c57600080fd5b5061020161053b36600461431b565b6121ef565b34801561054c57600080fd5b5061020161055b366004614890565b6122b6565b34801561056c57600080fd5b5061020161057b3660046148e0565b61230a565b34801561058c57600080fd5b506105c861059b366004614302565b60c9602052600090815260409020805460019091015463ffffffff821691640100000000900460ff169083565b6040805163ffffffff9094168452911515602084015290820152606001610234565b3480156105f657600080fd5b506097546102289060ff1681565b34801561061057600080fd5b5061020161061f366004614907565b6126c9565b610201610632366004614952565b6127b3565b34801561064357600080fd5b506103347f000000000000000000000000000000000000000000000000000000000000000081565b34801561067757600080fd5b506102b46129ab565b34801561068c57600080fd5b5061020161069b36600461431b565b612d57565b3480156106ac57600080fd5b506102016106bb36600461431b565b612dcd565b3480156106cc57600080fd5b506102286106db3660046149a4565b612df7565b3480156106ec57600080fd5b50606554610334906001600160a01b031681565b34801561070c57600080fd5b5061020161071b366004614a21565b612e6c565b8060000361074957604051632097692160e11b8152600481018290526024015b60405180910390fd5b6001600160a01b038216600090815260ca602052604081208054839290610771908490614aab565b90915550506001600160a01b038216600081815260ca602090815260409182902054915191825260008051602061517c833981519152910160405180910390a25050565b6000806001600160a01b0383166107cd5750846107f9565b85836040516020016107e0929190614abe565b6040516020818303038152906040528051906020012090505b600081815260c9602052604081205463ffffffff16900361081e5760009150506108a0565b600081815260c96020526040902054640100000000900460ff166108465760009150506108a0565b60408051602081018c90529081018a9052606081018990526001600160601b03198816608082015260009060940160408051601f198184030181529190528051602082012090915061089a8789838961321b565b93505050505b98975050505050505050565b60008060007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001878760000151886020015188600001516000600281106108f4576108f4614ad9565b60200201518951600160200201518a6020015160006002811061091957610919614ad9565b60200201518b6020015160016002811061093557610935614ad9565b602090810291909101518c518d8301516040516109929a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b6040516020818303038152906040528051906020012060001c6109b59190614aef565b9050610a286109ce6109c78884613233565b86906132c4565b6109d6613359565b610a1e610a0f85610a09604080518082018252600080825260209182015281518083019092526001825260029082015290565b90613233565b610a188c613419565b906132c4565b886201d4c06134a8565b909890975095505050505050565b33600090815260ca6020526040902054811115610a875733600081815260ca602052604090819020549051632e2a182f60e11b81526004810192909252602482018390526044820152606401610740565b33600090815260ca602052604081208054839290610aa6908490614b11565b909155505033600081815260ca602090815260409182902054915191825260008051602061517c833981519152910160405180910390a2604051339082156108fc029083906000818181858888f19350505050158015610b0a573d6000803e3d6000fd5b5050565b6040516309aa152760e11b81526001600160a01b0382811660048301526060916000917f000000000000000000000000000000000000000000000000000000000000000016906313542a4e90602401602060405180830381865afa158015610b7a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b9e9190614b24565b60405163871ef04960e01b8152600481018290529091506000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063871ef04990602401602060405180830381865afa158015610c09573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c2d9190614b3d565b90506001600160c01b0381161580610cc757507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c9e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610cc29190614b66565b60ff16155b15610ce75760408051600080825260208201909252905b50949350505050565b6000610cfb826001600160c01b03166136c2565b90506000805b8251811015610dc7577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316633ca5a5f5848381518110610d4b57610d4b614ad9565b01602001516040516001600160e01b031960e084901b16815260f89190911c6004820152602401602060405180830381865afa158015610d8f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610db39190614b24565b610dbd9083614aab565b9150600101610d01565b506000816001600160401b03811115610de257610de261403b565b604051908082528060200260200182016040528015610e0b578160200160208202803683370190505b5090506000805b8451811015610fb4576000858281518110610e2f57610e2f614ad9565b0160200151604051633ca5a5f560e01b815260f89190911c6004820181905291506000906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633ca5a5f590602401602060405180830381865afa158015610ea4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ec89190614b24565b905060005b81811015610fa9576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa158015610f42573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f669190614b9e565b60000151868681518110610f7c57610f7c614ad9565b6001600160a01b039092166020928302919091019091015284610f9e81614bdf565b955050600101610ecd565b505050600101610e12565b5090979650505050505050565b610fc9613784565b610fd2816137de565b50565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611033573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110579190614bf8565b6001600160a01b0316336001600160a01b0316146111035760405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a401610740565b610fd281613847565b610fd28134610720565b6040805180820190915260608082526020820152600082604001515160405180604001604052806001815260200160008152505114801561117257508260a0015151604051806040016040528060018152602001600081525051145b801561119957508260c0015151604051806040016040528060018152602001600081525051145b80156111c057508260e0015151604051806040016040528060018152602001600081525051145b61122a5760405162461bcd60e51b8152602060048201526041602482015260008051602061519c83398151915260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a401610740565b825151602084015151146112a25760405162461bcd60e51b81526020600482015260446024820181905260008051602061519c833981519152908201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b608482015260a401610740565b4363ffffffff168463ffffffff16106113115760405162461bcd60e51b815260206004820152603c602482015260008051602061519c83398151915260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b000000006064820152608401610740565b60408051808201825260008082526020808301829052835180850185526060808252818301528451808601865260018082529083019390935284518381528086019095529293919082810190803683370190505060208281019190915260408051808201825260018082526000919093015280518281528082019091529081602001602082028036833701905050815260408051808201909152606080825260208201528560200151516001600160401b038111156113d2576113d261403b565b6040519080825280602002602001820160405280156113fb578160200160208202803683370190505b5081526020860151516001600160401b0381111561141b5761141b61403b565b604051908082528060200260200182016040528015611444578160200160208202803683370190505b50816020018190525060006114f060405180604001604052806001815260200160008152507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156114c7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906114eb9190614b66565b61388e565b905060005b8760200151518110156117815761153a8860200151828151811061151b5761151b614ad9565b6020026020010151805160009081526020918201519091526040902090565b8360200151828151811061155057611550614ad9565b60209081029190910101528015611610576020830151611571600183614b11565b8151811061158157611581614ad9565b602002602001015160001c836020015182815181106115a2576115a2614ad9565b602002602001015160001c11611610576040805162461bcd60e51b815260206004820152602481019190915260008051602061519c83398151915260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152608401610740565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166304ec63518460200151838151811061165557611655614ad9565b60200260200101518b8b60000151858151811061167457611674614ad9565b60200260200101516040518463ffffffff1660e01b81526004016116b19392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa1580156116ce573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116f29190614b3d565b6001600160c01b03168360000151828151811061171157611711614ad9565b6020026020010181815250506117776109c761174b848660000151858151811061173d5761173d614ad9565b602002602001015116613921565b8a60200151848151811061176157611761614ad9565b602002602001015161394c90919063ffffffff16565b94506001016114f5565b505061178c83613a2f565b60975490935060ff166000816117a3576000611825565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c448feb86040518163ffffffff1660e01b8152600401602060405180830381865afa158015611801573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118259190614b24565b905060005b604051806040016040528060018152602001600081525051811015611f1b5782156119b6578963ffffffff16827f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663249a0c42604051806040016040528060018152602001600081525085815181106118ae576118ae614ad9565b01602001516040516001600160e01b031960e084901b16815260f89190911c6004820152602401602060405180830381865afa1580156118f2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906119169190614b24565b6119209190614aab565b116119b65760405162461bcd60e51b8152602060048201526066602482015260008051602061519c83398151915260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c401610740565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166368bccaac60405180604001604052806001815260200160008152508381518110611a0d57611a0d614ad9565b602001015160f81c60f81b60f81c8c8c60a001518581518110611a3257611a32614ad9565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015611a8e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ab29190614c15565b6001600160401b031916611ad58a60400151838151811061151b5761151b614ad9565b67ffffffffffffffff191614611b715760405162461bcd60e51b8152602060048201526061602482015260008051602061519c83398151915260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c401610740565b611ba189604001518281518110611b8a57611b8a614ad9565b6020026020010151876132c490919063ffffffff16565b95507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c8294c5660405180604001604052806001815260200160008152508381518110611bfa57611bfa614ad9565b602001015160f81c60f81b60f81c8c8c60c001518581518110611c1f57611c1f614ad9565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015611c7b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c9f9190614c40565b85602001518281518110611cb557611cb5614ad9565b6001600160601b03909216602092830291909101820152850151805182908110611ce157611ce1614ad9565b602002602001015185600001518281518110611cff57611cff614ad9565b60200260200101906001600160601b031690816001600160601b0316815250506000805b8a6020015151811015611f1157611d8e86600001518281518110611d4957611d49614ad9565b602002602001015160405180604001604052806001815260200160008152508581518110611d7957611d79614ad9565b016020015160f81c60ff161c60019081161490565b15611f09577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f2be94ae60405180604001604052806001815260200160008152508581518110611dea57611dea614ad9565b602001015160f81c60f81b60f81c8e89602001518581518110611e0f57611e0f614ad9565b60200260200101518f60e001518881518110611e2d57611e2d614ad9565b60200260200101518781518110611e4657611e46614ad9565b60209081029190910101516040516001600160e01b031960e087901b16815260ff909416600485015263ffffffff92831660248501526044840191909152166064820152608401602060405180830381865afa158015611eaa573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ece9190614c40565b8751805185908110611ee257611ee2614ad9565b60200260200101818151611ef69190614c5d565b6001600160601b03169052506001909101905b600101611d23565b505060010161182a565b505050600080611f358a868a606001518b608001516108ac565b9150915081611fa65760405162461bcd60e51b8152602060048201526043602482015260008051602061519c83398151915260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a401610740565b806120075760405162461bcd60e51b8152602060048201526039602482015260008051602061519c83398151915260448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608401610740565b50506000878260200151604051602001612022929190614c84565b60408051808303601f1901815291905280516020909101209299929850919650505050505050565b612052613784565b61205c6000613aca565b565b600054600290610100900460ff16158015612080575060005460ff8083169116105b61209c5760405162461bcd60e51b815260040161074090614ccc565b6000805461ffff191660ff8316176101001790556120b982612dcd565b6000805461ff001916905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b6000818484604051612111929190614d1a565b60405180910390201490509392505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461216b5760405162461bcd60e51b815260040161074090614d2a565b604051639926ee7d60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639926ee7d906121b99085908590600401614de8565b600060405180830381600087803b1580156121d357600080fd5b505af11580156121e7573d6000803e3d6000fd5b505050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146122375760405162461bcd60e51b815260040161074090614d2a565b6040516351b27a6d60e11b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a364f4da906024015b600060405180830381600087803b15801561229b57600080fd5b505af11580156122af573d6000803e3d6000fd5b5050505050565b6122be613784565b60405163a98fb35560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a98fb35590612281908490600401614e33565b60cb546001600160a01b0316331461234a5760cb54604051632cbe419560e01b81523360048201526001600160a01b039091166024820152604401610740565b60005a905060008484604051602001612364929190614abe565b60408051601f198184030181529181528151602092830120600081815260c990935290822080549193509163ffffffff90911690036123b9576040516311cb69a760e11b815260048101839052602401610740565b8054640100000000900460ff16156123e757604051634e78d7f960e11b815260048101839052602401610740565b805464ff00000000191664010000000017815560018101546001600160a01b038616600090815260ca6020526040902054101561246a5760018101546001600160a01b038616600081815260ca602052604090819020549051632e2a182f60e11b8152600481019290925260248201929092526044810191909152606401610740565b805460009061248190849063ffffffff1687611116565b509050604360ff1681602001516000815181106124a0576124a0614ad9565b60200260200101516124b29190614e46565b6001600160601b0316606482600001516000815181106124d4576124d4614ad9565b60200260200101516001600160601b03166124ef9190614e69565b1015612582576064816000015160008151811061250e5761250e614ad9565b60200260200101516001600160601b03166125299190614e69565b604360ff16826020015160008151811061254557612545614ad9565b60200260200101516125579190614e46565b60405163530f5c4560e11b815260048101929092526001600160601b03166024820152604401610740565b6040516001600160a01b038716815287907f8511746b73275e06971968773119b9601fc501d7bdf3824d8754042d148940e29060200160405180910390a260003a5a6125ce9087614b11565b6125db9062011170614aab565b6125e59190614e69565b9050826001015481111561261c57600183015460405163437e283f60e11b8152600481019190915260248101829052604401610740565b6001600160a01b038716600090815260ca602052604081208054839290612644908490614b11565b90915550506001600160a01b038716600081815260ca602090815260409182902054915191825260008051602061517c833981519152910160405180910390a260cb546040516001600160a01b039091169082156108fc029083906000818181858888f193505050501580156126be573d6000803e3d6000fd5b505050505050505050565b600054610100900460ff16158080156126e95750600054600160ff909116105b806127035750303b158015612703575060005460ff166001145b61271f5760405162461bcd60e51b815260040161074090614ccc565b6000805460ff191660011790558015612742576000805461ff0019166101001790555b61274c8484613b1c565b60cb80546001600160a01b0319166001600160a01b03841617905580156127ad576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b600084336040516020016127c8929190614abe565b60408051601f198184030181529181528151602092830120600081815260c990935291205490915063ffffffff161561281757604051630c40bc4360e21b815260048101829052602401610740565b34156128745733600090815260ca60205260408120805434929061283c908490614aab565b909155505033600081815260ca602090815260409182902054915191825260008051602061517c833981519152910160405180910390a25b33600090815260ca60205260409020548211156128c55733600081815260ca602052604090819020549051632e2a182f60e11b81526004810192909252602482018490526044820152606401610740565b604080516060810182526000602080830182815263ffffffff43818116865285870189815288865260c99094529386902085518154935115156401000000000264ffffffffff1990941692169190911791909117815590516001909101559151909187917f130d3e81af62e03ed6fff5e3bb343695ec513892cfad24d286486745dcc614379161295b913391908a908a90614ea9565b60405180910390a2857f8801fc966deb2c8f563a103c35c9e80740585c292cd97518587e6e7927e6af55334388888860405161299b959493929190614ee4565b60405180910390a2505050505050565b606060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612a0d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612a319190614b66565b60ff16905080600003612a5257505060408051600081526020810190915290565b6000805b82811015612afd57604051633ca5a5f560e01b815260ff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa158015612ac5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612ae99190614b24565b612af39083614aab565b9150600101612a56565b506000816001600160401b03811115612b1857612b1861403b565b604051908082528060200260200182016040528015612b41578160200160208202803683370190505b5090506000805b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612ba6573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612bca9190614b66565b60ff16811015612d4d57604051633ca5a5f560e01b815260ff821660048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa158015612c3e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612c629190614b24565b905060005b81811015612d43576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa158015612cdc573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612d009190614b9e565b60000151858581518110612d1657612d16614ad9565b6001600160a01b039092166020928302919091019091015283612d3881614bdf565b945050600101612c67565b5050600101612b48565b5090949350505050565b612d5f613784565b6001600160a01b038116612dc45760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610740565b610fd281613aca565b612dd5613784565b60cb80546001600160a01b0319166001600160a01b0392909216919091179055565b6040516306045a9160e01b815260009030906306045a9190612e2b908b908b908b908b908b908b908b908b90600401614f27565b602060405180830381865afa158015612e48573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108a09190614f89565b6065546001600160a01b03163314612f015760405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a401610740565b60005b818110156131cc57828282818110612f1e57612f1e614ad9565b9050602002810190612f309190614fa6565b612f4190604081019060200161431b565b6001600160a01b03166323b872dd3330868686818110612f6357612f63614ad9565b9050602002810190612f759190614fa6565b604080516001600160e01b031960e087901b1681526001600160a01b039485166004820152939092166024840152013560448201526064016020604051808303816000875af1158015612fcc573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612ff09190614f89565b50600083838381811061300557613005614ad9565b90506020028101906130179190614fa6565b61302890604081019060200161431b565b604051636eb1769f60e11b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081166024830152919091169063dd62ed3e90604401602060405180830381865afa158015613096573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906130ba9190614b24565b90508383838181106130ce576130ce614ad9565b90506020028101906130e09190614fa6565b6130f190604081019060200161431b565b6001600160a01b031663095ea7b37f00000000000000000000000000000000000000000000000000000000000000008387878781811061313357613133614ad9565b90506020028101906131459190614fa6565b604001356131539190614aab565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044016020604051808303816000875af115801561319e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906131c29190614f89565b5050600101612f04565b5060405163fce36c7d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063fce36c7d906121b9908590859060040161502c565b600083613229868585613b99565b1495945050505050565b604080518082019091526000808252602082015261324f613f49565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa9050808061327e57fe5b50806132bc5760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401610740565b505092915050565b60408051808201909152600080825260208201526132e0613f67565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa9050808061331b57fe5b50806132bc5760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606401610740565b613361613f85565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b60408051808201909152600080825260208201526000808061344960008051602061515c83398151915286614aef565b90505b61345581613c96565b909350915060008051602061515c833981519152828309830361348e576040805180820190915290815260208101919091529392505050565b60008051602061515c83398151915260018208905061344c565b6040805180820182528681526020808201869052825180840190935286835282018490526000918291906134da613faa565b60005b60028110156136955760006134f3826006614e69565b905084826002811061350757613507614ad9565b60200201515183613519836000614aab565b600c811061352957613529614ad9565b602002015284826002811061354057613540614ad9565b602002015160200151838260016135579190614aab565b600c811061356757613567614ad9565b602002015283826002811061357e5761357e614ad9565b6020020151515183613591836002614aab565b600c81106135a1576135a1614ad9565b60200201528382600281106135b8576135b8614ad9565b60200201515160016020020151836135d1836003614aab565b600c81106135e1576135e1614ad9565b60200201528382600281106135f8576135f8614ad9565b60200201516020015160006002811061361357613613614ad9565b602002015183613624836004614aab565b600c811061363457613634614ad9565b602002015283826002811061364b5761364b614ad9565b60200201516020015160016002811061366657613666614ad9565b602002015183613677836005614aab565b600c811061368757613687614ad9565b6020020152506001016134dd565b5061369e613fc9565b60006020826101808560088cfa9151919c9115159b50909950505050505050505050565b60606000806136d084613921565b61ffff166001600160401b038111156136eb576136eb61403b565b6040519080825280601f01601f191660200182016040528015613715576020820181803683370190505b5090506000805b82518210801561372d575061010081105b15612d4d576001811b935085841615613774578060f81b83838151811061375657613756614ad9565b60200101906001600160f81b031916908160001a9053508160010191505b61377d81614bdf565b905061371c565b6033546001600160a01b0316331461205c5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610740565b606554604080516001600160a01b03928316815291831660208301527fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6097805460ff19168215159081179091556040519081527f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc9060200160405180910390a150565b60008061389a84613d18565b9050808360ff166001901b116139185760405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c7565006064820152608401610740565b90505b92915050565b6000805b821561391b57613936600184614b11565b90921691806139448161513a565b915050613925565b60408051808201909152600080825260208201526102008261ffff16106139a85760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606401610740565b8161ffff166001036139bb57508161391b565b6040805180820190915260008082526020820181905284906001905b8161ffff168661ffff1610613a2457600161ffff871660ff83161c81169003613a0757613a0484846132c4565b93505b613a1183846132c4565b92506201fffe600192831b1691016139d7565b509195945050505050565b60408051808201909152600080825260208201528151158015613a5457506020820151155b15613a72575050604080518082019091526000808252602082015290565b60405180604001604052808360000151815260200160008051602061515c8339815191528460200151613aa59190614aef565b613abd9060008051602061515c833981519152614b11565b905292915050565b919050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b600054610100900460ff16613b875760405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608401610740565b613b9082613aca565b610b0a816137de565b600060208451613ba99190614aef565b15613c305760405162461bcd60e51b815260206004820152604b60248201527f4d65726b6c652e70726f63657373496e636c7573696f6e50726f6f664b65636360448201527f616b3a2070726f6f66206c656e6774682073686f756c642062652061206d756c60648201526a3a34b836329037b310199960a91b608482015260a401610740565b8260205b85518111610cde57613c47600285614aef565b600003613c6b57816000528086015160205260406000209150600284049350613c84565b8086015160005281602052604060002091506002840493505b613c8f602082614aab565b9050613c34565b6000808060008051602061515c833981519152600360008051602061515c8339815191528660008051602061515c833981519152888909090890506000613d0c827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260008051602061515c833981519152613ea0565b91959194509092505050565b600061010082511115613da15760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a401610740565b8151600003613db257506000919050565b60008083600081518110613dc857613dc8614ad9565b0160200151600160f89190911c81901b92505b8451811015613e9757848181518110613df657613df6614ad9565b0160200151600160f89190911c1b9150828211613e8b5760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a401610740565b91811791600101613ddb565b50909392505050565b600080613eab613fc9565b613eb3613fe7565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280613ef057fe5b5082613f3e5760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606401610740565b505195945050505050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060400160405280613f98614005565b8152602001613fa5614005565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b80356001600160601b031981168114613ac557600080fd5b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156140735761407361403b565b60405290565b60405161010081016001600160401b03811182821017156140735761407361403b565b604051601f8201601f191681016001600160401b03811182821017156140c4576140c461403b565b604052919050565b60006001600160401b038311156140e5576140e561403b565b6140f8601f8401601f191660200161409c565b905082815283838301111561410c57600080fd5b828260208301376000602084830101529392505050565b600082601f83011261413457600080fd5b614143838335602085016140cc565b9392505050565b6001600160a01b0381168114610fd257600080fd5b600080600080600080600080610100898b03121561417c57600080fd5b88359750602089013596506040890135955061419a60608a01614023565b94506080890135935060a08901356001600160401b038111156141bc57600080fd5b6141c88b828c01614123565b93505060c0890135915060e08901356141e08161414a565b809150509295985092959890939650565b60006040828403121561420357600080fd5b61420b614051565b9050813581526020820135602082015292915050565b600082601f83011261423257600080fd5b61423a614051565b80604084018581111561424c57600080fd5b845b8181101561426657803584526020938401930161424e565b509095945050505050565b60006080828403121561428357600080fd5b61428b614051565b90506142978383614221565b81526142a68360408401614221565b602082015292915050565b60008060008061012085870312156142c857600080fd5b843593506142d986602087016141f1565b92506142e88660608701614271565b91506142f78660e087016141f1565b905092959194509250565b60006020828403121561431457600080fd5b5035919050565b60006020828403121561432d57600080fd5b81356139188161414a565b6020808252825182820181905260009190848201906040850190845b818110156143795783516001600160a01b031683529284019291840191600101614354565b50909695505050505050565b8015158114610fd257600080fd5b6000602082840312156143a557600080fd5b813561391881614385565b803563ffffffff81168114613ac557600080fd5b60006001600160401b038211156143dd576143dd61403b565b5060051b60200190565b600082601f8301126143f857600080fd5b8135602061440d614408836143c4565b61409c565b8083825260208201915060208460051b87010193508684111561442f57600080fd5b602086015b8481101561445257614445816143b0565b8352918301918301614434565b509695505050505050565b600082601f83011261446e57600080fd5b8135602061447e614408836143c4565b8083825260208201915060208460061b8701019350868411156144a057600080fd5b602086015b84811015614452576144b788826141f1565b8352918301916040016144a5565b600082601f8301126144d657600080fd5b813560206144e6614408836143c4565b82815260059290921b8401810191818101908684111561450557600080fd5b8286015b848110156144525780356001600160401b038111156145285760008081fd5b6145368986838b01016143e7565b845250918301918301614509565b6000610180828403121561455757600080fd5b61455f614079565b905081356001600160401b038082111561457857600080fd5b614584858386016143e7565b8352602084013591508082111561459a57600080fd5b6145a68583860161445d565b602084015260408401359150808211156145bf57600080fd5b6145cb8583860161445d565b60408401526145dd8560608601614271565b60608401526145ef8560e086016141f1565b608084015261012084013591508082111561460957600080fd5b614615858386016143e7565b60a084015261014084013591508082111561462f57600080fd5b61463b858386016143e7565b60c084015261016084013591508082111561465557600080fd5b50614662848285016144c5565b60e08301525092915050565b60008060006060848603121561468357600080fd5b83359250614693602085016143b0565b915060408401356001600160401b038111156146ae57600080fd5b6146ba86828701614544565b9150509250925092565b60008151808452602080850194506020840160005b838110156146fe5781516001600160601b0316875295820195908201906001016146d9565b509495945050505050565b604081526000835160408084015261472460808401826146c4565b90506020850151603f1984830301606085015261474182826146c4565b925050508260208301529392505050565b60008083601f84011261476457600080fd5b5081356001600160401b0381111561477b57600080fd5b60208301915083602082850101111561479357600080fd5b9250929050565b6000806000604084860312156147af57600080fd5b83356001600160401b038111156147c557600080fd5b6147d186828701614752565b909790965060209590950135949350505050565b600080604083850312156147f857600080fd5b82356148038161414a565b915060208301356001600160401b038082111561481f57600080fd5b908401906060828703121561483357600080fd5b60405160608101818110838211171561484e5761484e61403b565b60405282358281111561486057600080fd5b61486c88828601614123565b82525060208301356020820152604083013560408201528093505050509250929050565b6000602082840312156148a257600080fd5b81356001600160401b038111156148b857600080fd5b8201601f810184136148c957600080fd5b6148d8848235602084016140cc565b949350505050565b6000806000606084860312156148f557600080fd5b8335925060208401356146938161414a565b60008060006060848603121561491c57600080fd5b83356149278161414a565b925060208401356149378161414a565b915060408401356149478161414a565b809150509250925092565b6000806000806060858703121561496857600080fd5b8435935060208501356001600160401b0381111561498557600080fd5b61499187828801614752565b9598909750949560400135949350505050565b600080600080600080600060e0888a0312156149bf57600080fd5b8735965060208801359550604088013594506149dd60608901614023565b93506080880135925060a08801356001600160401b038111156149ff57600080fd5b614a0b8a828b01614123565b92505060c0880135905092959891949750929550565b60008060208385031215614a3457600080fd5b82356001600160401b0380821115614a4b57600080fd5b818501915085601f830112614a5f57600080fd5b813581811115614a6e57600080fd5b8660208260051b8501011115614a8357600080fd5b60209290920196919550909350505050565b634e487b7160e01b600052601160045260246000fd5b8082018082111561391b5761391b614a95565b91825260601b6001600160601b031916602082015260340190565b634e487b7160e01b600052603260045260246000fd5b600082614b0c57634e487b7160e01b600052601260045260246000fd5b500690565b8181038181111561391b5761391b614a95565b600060208284031215614b3657600080fd5b5051919050565b600060208284031215614b4f57600080fd5b81516001600160c01b038116811461391857600080fd5b600060208284031215614b7857600080fd5b815160ff8116811461391857600080fd5b6001600160601b0381168114610fd257600080fd5b600060408284031215614bb057600080fd5b614bb8614051565b8251614bc38161414a565b81526020830151614bd381614b89565b60208201529392505050565b600060018201614bf157614bf1614a95565b5060010190565b600060208284031215614c0a57600080fd5b81516139188161414a565b600060208284031215614c2757600080fd5b815167ffffffffffffffff198116811461391857600080fd5b600060208284031215614c5257600080fd5b815161391881614b89565b6001600160601b03828116828216039080821115614c7d57614c7d614a95565b5092915050565b63ffffffff60e01b8360e01b1681526000600482018351602080860160005b83811015614cbf57815185529382019390820190600101614ca3565b5092979650505050505050565b6020808252602e908201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160408201526d191e481a5b9a5d1a585b1a5e995960921b606082015260800190565b8183823760009101908152919050565b60208082526052908201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360408201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560608201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608082015260a00190565b6000815180845260005b81811015614dc857602081850181015186830182015201614dac565b506000602082860101526020601f19601f83011685010191505092915050565b60018060a01b0383168152604060208201526000825160606040840152614e1260a0840182614da2565b90506020840151606084015260408401516080840152809150509392505050565b6020815260006141436020830184614da2565b6001600160601b038181168382160280821691908281146132bc576132bc614a95565b808202811582820484141761391b5761391b614a95565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6001600160a01b038516815263ffffffff84166020820152606060408201819052600090614eda9083018486614e80565b9695505050505050565b6001600160a01b038616815263ffffffff85166020820152608060408201819052600090614f159083018587614e80565b90508260608301529695505050505050565b60006101008a83528960208401528860408401526001600160601b0319881660608401528660808401528060a0840152614f6381840187614da2565b60c084019590955250506001600160a01b039190911660e0909101529695505050505050565b600060208284031215614f9b57600080fd5b815161391881614385565b60008235609e19833603018112614fbc57600080fd5b9190910192915050565b8035613ac58161414a565b8183526000602080850194508260005b858110156146fe578135614ff48161414a565b6001600160a01b031687528183013561500c81614b89565b6001600160601b0316878401526040968701969190910190600101614fe1565b60208082528181018390526000906040808401600586901b8501820187855b8881101561512c57878303603f190184528135368b9003609e1901811261507157600080fd5b8a0160a0813536839003601e1901811261508a57600080fd5b820188810190356001600160401b038111156150a557600080fd5b8060061b36038213156150b757600080fd5b8287526150c78388018284614fd1565b925050506150d6888301614fc6565b6001600160a01b031688860152818701358786015260606150f88184016143b0565b63ffffffff1690860152608061510f8382016143b0565b63ffffffff1695019490945250928501929085019060010161504b565b509098975050505050505050565b600061ffff80831681810361515157615151614a95565b600101939250505056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd470ea46f246ccfc58f7a93aa09bc6245a6818e97b1a160d186afe78993a3b194a0424c535369676e6174757265436865636b65722e636865636b5369676e617475a26469706673582212205f622ca29664bb0bf8adb3fe637ad4323fffaeffe58a5ec6d1538d9956d8bd4664736f6c63430008180033",
}

// ContractAlignedLayerServiceManagerABI is the input ABI used to generate the binding from.
// Deprecated: Use ContractAlignedLayerServiceManagerMetaData.ABI instead.
var ContractAlignedLayerServiceManagerABI = ContractAlignedLayerServiceManagerMetaData.ABI

// ContractAlignedLayerServiceManagerBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use ContractAlignedLayerServiceManagerMetaData.Bin instead.
var ContractAlignedLayerServiceManagerBin = ContractAlignedLayerServiceManagerMetaData.Bin

// DeployContractAlignedLayerServiceManager deploys a new Ethereum contract, binding an instance of ContractAlignedLayerServiceManager to it.
func DeployContractAlignedLayerServiceManager(auth *bind.TransactOpts, backend bind.ContractBackend, __avsDirectory common.Address, __rewardsCoordinator common.Address, __registryCoordinator common.Address, __stakeRegistry common.Address) (common.Address, *types.Transaction, *ContractAlignedLayerServiceManager, error) {
	parsed, err := ContractAlignedLayerServiceManagerMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(ContractAlignedLayerServiceManagerBin), backend, __avsDirectory, __rewardsCoordinator, __registryCoordinator, __stakeRegistry)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &ContractAlignedLayerServiceManager{ContractAlignedLayerServiceManagerCaller: ContractAlignedLayerServiceManagerCaller{contract: contract}, ContractAlignedLayerServiceManagerTransactor: ContractAlignedLayerServiceManagerTransactor{contract: contract}, ContractAlignedLayerServiceManagerFilterer: ContractAlignedLayerServiceManagerFilterer{contract: contract}}, nil
}

// ContractAlignedLayerServiceManager is an auto generated Go binding around an Ethereum contract.
type ContractAlignedLayerServiceManager struct {
	ContractAlignedLayerServiceManagerCaller     // Read-only binding to the contract
	ContractAlignedLayerServiceManagerTransactor // Write-only binding to the contract
	ContractAlignedLayerServiceManagerFilterer   // Log filterer for contract events
}

// ContractAlignedLayerServiceManagerCaller is an auto generated read-only Go binding around an Ethereum contract.
type ContractAlignedLayerServiceManagerCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractAlignedLayerServiceManagerTransactor is an auto generated write-only Go binding around an Ethereum contract.
type ContractAlignedLayerServiceManagerTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractAlignedLayerServiceManagerFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type ContractAlignedLayerServiceManagerFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractAlignedLayerServiceManagerSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type ContractAlignedLayerServiceManagerSession struct {
	Contract     *ContractAlignedLayerServiceManager // Generic contract binding to set the session for
	CallOpts     bind.CallOpts                       // Call options to use throughout this session
	TransactOpts bind.TransactOpts                   // Transaction auth options to use throughout this session
}

// ContractAlignedLayerServiceManagerCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type ContractAlignedLayerServiceManagerCallerSession struct {
	Contract *ContractAlignedLayerServiceManagerCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts                             // Call options to use throughout this session
}

// ContractAlignedLayerServiceManagerTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type ContractAlignedLayerServiceManagerTransactorSession struct {
	Contract     *ContractAlignedLayerServiceManagerTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts                             // Transaction auth options to use throughout this session
}

// ContractAlignedLayerServiceManagerRaw is an auto generated low-level Go binding around an Ethereum contract.
type ContractAlignedLayerServiceManagerRaw struct {
	Contract *ContractAlignedLayerServiceManager // Generic contract binding to access the raw methods on
}

// ContractAlignedLayerServiceManagerCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type ContractAlignedLayerServiceManagerCallerRaw struct {
	Contract *ContractAlignedLayerServiceManagerCaller // Generic read-only contract binding to access the raw methods on
}

// ContractAlignedLayerServiceManagerTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type ContractAlignedLayerServiceManagerTransactorRaw struct {
	Contract *ContractAlignedLayerServiceManagerTransactor // Generic write-only contract binding to access the raw methods on
}

// NewContractAlignedLayerServiceManager creates a new instance of ContractAlignedLayerServiceManager, bound to a specific deployed contract.
func NewContractAlignedLayerServiceManager(address common.Address, backend bind.ContractBackend) (*ContractAlignedLayerServiceManager, error) {
	contract, err := bindContractAlignedLayerServiceManager(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManager{ContractAlignedLayerServiceManagerCaller: ContractAlignedLayerServiceManagerCaller{contract: contract}, ContractAlignedLayerServiceManagerTransactor: ContractAlignedLayerServiceManagerTransactor{contract: contract}, ContractAlignedLayerServiceManagerFilterer: ContractAlignedLayerServiceManagerFilterer{contract: contract}}, nil
}

// NewContractAlignedLayerServiceManagerCaller creates a new read-only instance of ContractAlignedLayerServiceManager, bound to a specific deployed contract.
func NewContractAlignedLayerServiceManagerCaller(address common.Address, caller bind.ContractCaller) (*ContractAlignedLayerServiceManagerCaller, error) {
	contract, err := bindContractAlignedLayerServiceManager(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerCaller{contract: contract}, nil
}

// NewContractAlignedLayerServiceManagerTransactor creates a new write-only instance of ContractAlignedLayerServiceManager, bound to a specific deployed contract.
func NewContractAlignedLayerServiceManagerTransactor(address common.Address, transactor bind.ContractTransactor) (*ContractAlignedLayerServiceManagerTransactor, error) {
	contract, err := bindContractAlignedLayerServiceManager(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerTransactor{contract: contract}, nil
}

// NewContractAlignedLayerServiceManagerFilterer creates a new log filterer instance of ContractAlignedLayerServiceManager, bound to a specific deployed contract.
func NewContractAlignedLayerServiceManagerFilterer(address common.Address, filterer bind.ContractFilterer) (*ContractAlignedLayerServiceManagerFilterer, error) {
	contract, err := bindContractAlignedLayerServiceManager(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerFilterer{contract: contract}, nil
}

// bindContractAlignedLayerServiceManager binds a generic wrapper to an already deployed contract.
func bindContractAlignedLayerServiceManager(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := ContractAlignedLayerServiceManagerMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractAlignedLayerServiceManager.Contract.ContractAlignedLayerServiceManagerCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.ContractAlignedLayerServiceManagerTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.ContractAlignedLayerServiceManagerTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractAlignedLayerServiceManager.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.contract.Transact(opts, method, params...)
}

// AlignedAggregator is a free data retrieval call binding the contract method 0x4a5bf632.
//
// Solidity: function alignedAggregator() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) AlignedAggregator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "alignedAggregator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// AlignedAggregator is a free data retrieval call binding the contract method 0x4a5bf632.
//
// Solidity: function alignedAggregator() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) AlignedAggregator() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.AlignedAggregator(&_ContractAlignedLayerServiceManager.CallOpts)
}

// AlignedAggregator is a free data retrieval call binding the contract method 0x4a5bf632.
//
// Solidity: function alignedAggregator() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) AlignedAggregator() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.AlignedAggregator(&_ContractAlignedLayerServiceManager.CallOpts)
}

// AvsDirectory is a free data retrieval call binding the contract method 0x6b3aa72e.
//
// Solidity: function avsDirectory() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) AvsDirectory(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "avsDirectory")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// AvsDirectory is a free data retrieval call binding the contract method 0x6b3aa72e.
//
// Solidity: function avsDirectory() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) AvsDirectory() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.AvsDirectory(&_ContractAlignedLayerServiceManager.CallOpts)
}

// AvsDirectory is a free data retrieval call binding the contract method 0x6b3aa72e.
//
// Solidity: function avsDirectory() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) AvsDirectory() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.AvsDirectory(&_ContractAlignedLayerServiceManager.CallOpts)
}

// BalanceOf is a free data retrieval call binding the contract method 0x70a08231.
//
// Solidity: function balanceOf(address account) view returns(uint256)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) BalanceOf(opts *bind.CallOpts, account common.Address) (*big.Int, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "balanceOf", account)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// BalanceOf is a free data retrieval call binding the contract method 0x70a08231.
//
// Solidity: function balanceOf(address account) view returns(uint256)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) BalanceOf(account common.Address) (*big.Int, error) {
	return _ContractAlignedLayerServiceManager.Contract.BalanceOf(&_ContractAlignedLayerServiceManager.CallOpts, account)
}

// BalanceOf is a free data retrieval call binding the contract method 0x70a08231.
//
// Solidity: function balanceOf(address account) view returns(uint256)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) BalanceOf(account common.Address) (*big.Int, error) {
	return _ContractAlignedLayerServiceManager.Contract.BalanceOf(&_ContractAlignedLayerServiceManager.CallOpts, account)
}

// BatchesState is a free data retrieval call binding the contract method 0xb099627e.
//
// Solidity: function batchesState(bytes32 ) view returns(uint32 taskCreatedBlock, bool responded, uint256 respondToTaskFeeLimit)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) BatchesState(opts *bind.CallOpts, arg0 [32]byte) (struct {
	TaskCreatedBlock      uint32
	Responded             bool
	RespondToTaskFeeLimit *big.Int
}, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "batchesState", arg0)

	outstruct := new(struct {
		TaskCreatedBlock      uint32
		Responded             bool
		RespondToTaskFeeLimit *big.Int
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.TaskCreatedBlock = *abi.ConvertType(out[0], new(uint32)).(*uint32)
	outstruct.Responded = *abi.ConvertType(out[1], new(bool)).(*bool)
	outstruct.RespondToTaskFeeLimit = *abi.ConvertType(out[2], new(*big.Int)).(**big.Int)

	return *outstruct, err

}

// BatchesState is a free data retrieval call binding the contract method 0xb099627e.
//
// Solidity: function batchesState(bytes32 ) view returns(uint32 taskCreatedBlock, bool responded, uint256 respondToTaskFeeLimit)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) BatchesState(arg0 [32]byte) (struct {
	TaskCreatedBlock      uint32
	Responded             bool
	RespondToTaskFeeLimit *big.Int
}, error) {
	return _ContractAlignedLayerServiceManager.Contract.BatchesState(&_ContractAlignedLayerServiceManager.CallOpts, arg0)
}

// BatchesState is a free data retrieval call binding the contract method 0xb099627e.
//
// Solidity: function batchesState(bytes32 ) view returns(uint32 taskCreatedBlock, bool responded, uint256 respondToTaskFeeLimit)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) BatchesState(arg0 [32]byte) (struct {
	TaskCreatedBlock      uint32
	Responded             bool
	RespondToTaskFeeLimit *big.Int
}, error) {
	return _ContractAlignedLayerServiceManager.Contract.BatchesState(&_ContractAlignedLayerServiceManager.CallOpts, arg0)
}

// BlsApkRegistry is a free data retrieval call binding the contract method 0x5df45946.
//
// Solidity: function blsApkRegistry() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) BlsApkRegistry(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "blsApkRegistry")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// BlsApkRegistry is a free data retrieval call binding the contract method 0x5df45946.
//
// Solidity: function blsApkRegistry() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) BlsApkRegistry() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.BlsApkRegistry(&_ContractAlignedLayerServiceManager.CallOpts)
}

// BlsApkRegistry is a free data retrieval call binding the contract method 0x5df45946.
//
// Solidity: function blsApkRegistry() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) BlsApkRegistry() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.BlsApkRegistry(&_ContractAlignedLayerServiceManager.CallOpts)
}

// CheckPublicInput is a free data retrieval call binding the contract method 0x95c6d604.
//
// Solidity: function checkPublicInput(bytes publicInput, bytes32 hash) pure returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) CheckPublicInput(opts *bind.CallOpts, publicInput []byte, hash [32]byte) (bool, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "checkPublicInput", publicInput, hash)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// CheckPublicInput is a free data retrieval call binding the contract method 0x95c6d604.
//
// Solidity: function checkPublicInput(bytes publicInput, bytes32 hash) pure returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) CheckPublicInput(publicInput []byte, hash [32]byte) (bool, error) {
	return _ContractAlignedLayerServiceManager.Contract.CheckPublicInput(&_ContractAlignedLayerServiceManager.CallOpts, publicInput, hash)
}

// CheckPublicInput is a free data retrieval call binding the contract method 0x95c6d604.
//
// Solidity: function checkPublicInput(bytes publicInput, bytes32 hash) pure returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) CheckPublicInput(publicInput []byte, hash [32]byte) (bool, error) {
	return _ContractAlignedLayerServiceManager.Contract.CheckPublicInput(&_ContractAlignedLayerServiceManager.CallOpts, publicInput, hash)
}

// CheckSignatures is a free data retrieval call binding the contract method 0x4ae07c37.
//
// Solidity: function checkSignatures(bytes32 msgHash, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) params) view returns((uint96[],uint96[]), bytes32)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) CheckSignatures(opts *bind.CallOpts, msgHash [32]byte, referenceBlockNumber uint32, params IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, [32]byte, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "checkSignatures", msgHash, referenceBlockNumber, params)

	if err != nil {
		return *new(IBLSSignatureCheckerQuorumStakeTotals), *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new(IBLSSignatureCheckerQuorumStakeTotals)).(*IBLSSignatureCheckerQuorumStakeTotals)
	out1 := *abi.ConvertType(out[1], new([32]byte)).(*[32]byte)

	return out0, out1, err

}

// CheckSignatures is a free data retrieval call binding the contract method 0x4ae07c37.
//
// Solidity: function checkSignatures(bytes32 msgHash, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) params) view returns((uint96[],uint96[]), bytes32)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) CheckSignatures(msgHash [32]byte, referenceBlockNumber uint32, params IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, [32]byte, error) {
	return _ContractAlignedLayerServiceManager.Contract.CheckSignatures(&_ContractAlignedLayerServiceManager.CallOpts, msgHash, referenceBlockNumber, params)
}

// CheckSignatures is a free data retrieval call binding the contract method 0x4ae07c37.
//
// Solidity: function checkSignatures(bytes32 msgHash, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) params) view returns((uint96[],uint96[]), bytes32)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) CheckSignatures(msgHash [32]byte, referenceBlockNumber uint32, params IBLSSignatureCheckerNonSignerStakesAndSignature) (IBLSSignatureCheckerQuorumStakeTotals, [32]byte, error) {
	return _ContractAlignedLayerServiceManager.Contract.CheckSignatures(&_ContractAlignedLayerServiceManager.CallOpts, msgHash, referenceBlockNumber, params)
}

// Delegation is a free data retrieval call binding the contract method 0xdf5cf723.
//
// Solidity: function delegation() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) Delegation(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "delegation")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Delegation is a free data retrieval call binding the contract method 0xdf5cf723.
//
// Solidity: function delegation() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) Delegation() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.Delegation(&_ContractAlignedLayerServiceManager.CallOpts)
}

// Delegation is a free data retrieval call binding the contract method 0xdf5cf723.
//
// Solidity: function delegation() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) Delegation() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.Delegation(&_ContractAlignedLayerServiceManager.CallOpts)
}

// GetOperatorRestakedStrategies is a free data retrieval call binding the contract method 0x33cfb7b7.
//
// Solidity: function getOperatorRestakedStrategies(address operator) view returns(address[])
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) GetOperatorRestakedStrategies(opts *bind.CallOpts, operator common.Address) ([]common.Address, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "getOperatorRestakedStrategies", operator)

	if err != nil {
		return *new([]common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new([]common.Address)).(*[]common.Address)

	return out0, err

}

// GetOperatorRestakedStrategies is a free data retrieval call binding the contract method 0x33cfb7b7.
//
// Solidity: function getOperatorRestakedStrategies(address operator) view returns(address[])
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) GetOperatorRestakedStrategies(operator common.Address) ([]common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.GetOperatorRestakedStrategies(&_ContractAlignedLayerServiceManager.CallOpts, operator)
}

// GetOperatorRestakedStrategies is a free data retrieval call binding the contract method 0x33cfb7b7.
//
// Solidity: function getOperatorRestakedStrategies(address operator) view returns(address[])
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) GetOperatorRestakedStrategies(operator common.Address) ([]common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.GetOperatorRestakedStrategies(&_ContractAlignedLayerServiceManager.CallOpts, operator)
}

// GetRestakeableStrategies is a free data retrieval call binding the contract method 0xe481af9d.
//
// Solidity: function getRestakeableStrategies() view returns(address[])
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) GetRestakeableStrategies(opts *bind.CallOpts) ([]common.Address, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "getRestakeableStrategies")

	if err != nil {
		return *new([]common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new([]common.Address)).(*[]common.Address)

	return out0, err

}

// GetRestakeableStrategies is a free data retrieval call binding the contract method 0xe481af9d.
//
// Solidity: function getRestakeableStrategies() view returns(address[])
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) GetRestakeableStrategies() ([]common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.GetRestakeableStrategies(&_ContractAlignedLayerServiceManager.CallOpts)
}

// GetRestakeableStrategies is a free data retrieval call binding the contract method 0xe481af9d.
//
// Solidity: function getRestakeableStrategies() view returns(address[])
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) GetRestakeableStrategies() ([]common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.GetRestakeableStrategies(&_ContractAlignedLayerServiceManager.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) Owner() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.Owner(&_ContractAlignedLayerServiceManager.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) Owner() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.Owner(&_ContractAlignedLayerServiceManager.CallOpts)
}

// RegistryCoordinator is a free data retrieval call binding the contract method 0x6d14a987.
//
// Solidity: function registryCoordinator() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) RegistryCoordinator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "registryCoordinator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// RegistryCoordinator is a free data retrieval call binding the contract method 0x6d14a987.
//
// Solidity: function registryCoordinator() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) RegistryCoordinator() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.RegistryCoordinator(&_ContractAlignedLayerServiceManager.CallOpts)
}

// RegistryCoordinator is a free data retrieval call binding the contract method 0x6d14a987.
//
// Solidity: function registryCoordinator() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) RegistryCoordinator() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.RegistryCoordinator(&_ContractAlignedLayerServiceManager.CallOpts)
}

// RewardsInitiator is a free data retrieval call binding the contract method 0xfc299dee.
//
// Solidity: function rewardsInitiator() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) RewardsInitiator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "rewardsInitiator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// RewardsInitiator is a free data retrieval call binding the contract method 0xfc299dee.
//
// Solidity: function rewardsInitiator() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) RewardsInitiator() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.RewardsInitiator(&_ContractAlignedLayerServiceManager.CallOpts)
}

// RewardsInitiator is a free data retrieval call binding the contract method 0xfc299dee.
//
// Solidity: function rewardsInitiator() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) RewardsInitiator() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.RewardsInitiator(&_ContractAlignedLayerServiceManager.CallOpts)
}

// StakeRegistry is a free data retrieval call binding the contract method 0x68304835.
//
// Solidity: function stakeRegistry() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) StakeRegistry(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "stakeRegistry")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// StakeRegistry is a free data retrieval call binding the contract method 0x68304835.
//
// Solidity: function stakeRegistry() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) StakeRegistry() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.StakeRegistry(&_ContractAlignedLayerServiceManager.CallOpts)
}

// StakeRegistry is a free data retrieval call binding the contract method 0x68304835.
//
// Solidity: function stakeRegistry() view returns(address)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) StakeRegistry() (common.Address, error) {
	return _ContractAlignedLayerServiceManager.Contract.StakeRegistry(&_ContractAlignedLayerServiceManager.CallOpts)
}

// StaleStakesForbidden is a free data retrieval call binding the contract method 0xb98d0908.
//
// Solidity: function staleStakesForbidden() view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) StaleStakesForbidden(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "staleStakesForbidden")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// StaleStakesForbidden is a free data retrieval call binding the contract method 0xb98d0908.
//
// Solidity: function staleStakesForbidden() view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) StaleStakesForbidden() (bool, error) {
	return _ContractAlignedLayerServiceManager.Contract.StaleStakesForbidden(&_ContractAlignedLayerServiceManager.CallOpts)
}

// StaleStakesForbidden is a free data retrieval call binding the contract method 0xb98d0908.
//
// Solidity: function staleStakesForbidden() view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) StaleStakesForbidden() (bool, error) {
	return _ContractAlignedLayerServiceManager.Contract.StaleStakesForbidden(&_ContractAlignedLayerServiceManager.CallOpts)
}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) TrySignatureAndApkVerification(opts *bind.CallOpts, msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "trySignatureAndApkVerification", msgHash, apk, apkG2, sigma)

	outstruct := new(struct {
		PairingSuccessful bool
		SiganatureIsValid bool
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.PairingSuccessful = *abi.ConvertType(out[0], new(bool)).(*bool)
	outstruct.SiganatureIsValid = *abi.ConvertType(out[1], new(bool)).(*bool)

	return *outstruct, err

}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) TrySignatureAndApkVerification(msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	return _ContractAlignedLayerServiceManager.Contract.TrySignatureAndApkVerification(&_ContractAlignedLayerServiceManager.CallOpts, msgHash, apk, apkG2, sigma)
}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) TrySignatureAndApkVerification(msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	return _ContractAlignedLayerServiceManager.Contract.TrySignatureAndApkVerification(&_ContractAlignedLayerServiceManager.CallOpts, msgHash, apk, apkG2, sigma)
}

// VerifyBatchInclusion is a free data retrieval call binding the contract method 0x06045a91.
//
// Solidity: function verifyBatchInclusion(bytes32 proofCommitment, bytes32 pubInputCommitment, bytes32 provingSystemAuxDataCommitment, bytes20 proofGeneratorAddr, bytes32 batchMerkleRoot, bytes merkleProof, uint256 verificationDataBatchIndex, address senderAddress) view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) VerifyBatchInclusion(opts *bind.CallOpts, proofCommitment [32]byte, pubInputCommitment [32]byte, provingSystemAuxDataCommitment [32]byte, proofGeneratorAddr [20]byte, batchMerkleRoot [32]byte, merkleProof []byte, verificationDataBatchIndex *big.Int, senderAddress common.Address) (bool, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "verifyBatchInclusion", proofCommitment, pubInputCommitment, provingSystemAuxDataCommitment, proofGeneratorAddr, batchMerkleRoot, merkleProof, verificationDataBatchIndex, senderAddress)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// VerifyBatchInclusion is a free data retrieval call binding the contract method 0x06045a91.
//
// Solidity: function verifyBatchInclusion(bytes32 proofCommitment, bytes32 pubInputCommitment, bytes32 provingSystemAuxDataCommitment, bytes20 proofGeneratorAddr, bytes32 batchMerkleRoot, bytes merkleProof, uint256 verificationDataBatchIndex, address senderAddress) view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) VerifyBatchInclusion(proofCommitment [32]byte, pubInputCommitment [32]byte, provingSystemAuxDataCommitment [32]byte, proofGeneratorAddr [20]byte, batchMerkleRoot [32]byte, merkleProof []byte, verificationDataBatchIndex *big.Int, senderAddress common.Address) (bool, error) {
	return _ContractAlignedLayerServiceManager.Contract.VerifyBatchInclusion(&_ContractAlignedLayerServiceManager.CallOpts, proofCommitment, pubInputCommitment, provingSystemAuxDataCommitment, proofGeneratorAddr, batchMerkleRoot, merkleProof, verificationDataBatchIndex, senderAddress)
}

// VerifyBatchInclusion is a free data retrieval call binding the contract method 0x06045a91.
//
// Solidity: function verifyBatchInclusion(bytes32 proofCommitment, bytes32 pubInputCommitment, bytes32 provingSystemAuxDataCommitment, bytes20 proofGeneratorAddr, bytes32 batchMerkleRoot, bytes merkleProof, uint256 verificationDataBatchIndex, address senderAddress) view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) VerifyBatchInclusion(proofCommitment [32]byte, pubInputCommitment [32]byte, provingSystemAuxDataCommitment [32]byte, proofGeneratorAddr [20]byte, batchMerkleRoot [32]byte, merkleProof []byte, verificationDataBatchIndex *big.Int, senderAddress common.Address) (bool, error) {
	return _ContractAlignedLayerServiceManager.Contract.VerifyBatchInclusion(&_ContractAlignedLayerServiceManager.CallOpts, proofCommitment, pubInputCommitment, provingSystemAuxDataCommitment, proofGeneratorAddr, batchMerkleRoot, merkleProof, verificationDataBatchIndex, senderAddress)
}

// VerifyBatchInclusion0 is a free data retrieval call binding the contract method 0xfa534dc0.
//
// Solidity: function verifyBatchInclusion(bytes32 proofCommitment, bytes32 pubInputCommitment, bytes32 provingSystemAuxDataCommitment, bytes20 proofGeneratorAddr, bytes32 batchMerkleRoot, bytes merkleProof, uint256 verificationDataBatchIndex) view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) VerifyBatchInclusion0(opts *bind.CallOpts, proofCommitment [32]byte, pubInputCommitment [32]byte, provingSystemAuxDataCommitment [32]byte, proofGeneratorAddr [20]byte, batchMerkleRoot [32]byte, merkleProof []byte, verificationDataBatchIndex *big.Int) (bool, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "verifyBatchInclusion0", proofCommitment, pubInputCommitment, provingSystemAuxDataCommitment, proofGeneratorAddr, batchMerkleRoot, merkleProof, verificationDataBatchIndex)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// VerifyBatchInclusion0 is a free data retrieval call binding the contract method 0xfa534dc0.
//
// Solidity: function verifyBatchInclusion(bytes32 proofCommitment, bytes32 pubInputCommitment, bytes32 provingSystemAuxDataCommitment, bytes20 proofGeneratorAddr, bytes32 batchMerkleRoot, bytes merkleProof, uint256 verificationDataBatchIndex) view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) VerifyBatchInclusion0(proofCommitment [32]byte, pubInputCommitment [32]byte, provingSystemAuxDataCommitment [32]byte, proofGeneratorAddr [20]byte, batchMerkleRoot [32]byte, merkleProof []byte, verificationDataBatchIndex *big.Int) (bool, error) {
	return _ContractAlignedLayerServiceManager.Contract.VerifyBatchInclusion0(&_ContractAlignedLayerServiceManager.CallOpts, proofCommitment, pubInputCommitment, provingSystemAuxDataCommitment, proofGeneratorAddr, batchMerkleRoot, merkleProof, verificationDataBatchIndex)
}

// VerifyBatchInclusion0 is a free data retrieval call binding the contract method 0xfa534dc0.
//
// Solidity: function verifyBatchInclusion(bytes32 proofCommitment, bytes32 pubInputCommitment, bytes32 provingSystemAuxDataCommitment, bytes20 proofGeneratorAddr, bytes32 batchMerkleRoot, bytes merkleProof, uint256 verificationDataBatchIndex) view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) VerifyBatchInclusion0(proofCommitment [32]byte, pubInputCommitment [32]byte, provingSystemAuxDataCommitment [32]byte, proofGeneratorAddr [20]byte, batchMerkleRoot [32]byte, merkleProof []byte, verificationDataBatchIndex *big.Int) (bool, error) {
	return _ContractAlignedLayerServiceManager.Contract.VerifyBatchInclusion0(&_ContractAlignedLayerServiceManager.CallOpts, proofCommitment, pubInputCommitment, provingSystemAuxDataCommitment, proofGeneratorAddr, batchMerkleRoot, merkleProof, verificationDataBatchIndex)
}

// CreateAVSRewardsSubmission is a paid mutator transaction binding the contract method 0xfce36c7d.
//
// Solidity: function createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[] rewardsSubmissions) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) CreateAVSRewardsSubmission(opts *bind.TransactOpts, rewardsSubmissions []IRewardsCoordinatorRewardsSubmission) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "createAVSRewardsSubmission", rewardsSubmissions)
}

// CreateAVSRewardsSubmission is a paid mutator transaction binding the contract method 0xfce36c7d.
//
// Solidity: function createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[] rewardsSubmissions) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) CreateAVSRewardsSubmission(rewardsSubmissions []IRewardsCoordinatorRewardsSubmission) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.CreateAVSRewardsSubmission(&_ContractAlignedLayerServiceManager.TransactOpts, rewardsSubmissions)
}

// CreateAVSRewardsSubmission is a paid mutator transaction binding the contract method 0xfce36c7d.
//
// Solidity: function createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[] rewardsSubmissions) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) CreateAVSRewardsSubmission(rewardsSubmissions []IRewardsCoordinatorRewardsSubmission) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.CreateAVSRewardsSubmission(&_ContractAlignedLayerServiceManager.TransactOpts, rewardsSubmissions)
}

// CreateNewTask is a paid mutator transaction binding the contract method 0xd66eaabd.
//
// Solidity: function createNewTask(bytes32 batchMerkleRoot, string batchDataPointer, uint256 respondToTaskFeeLimit) payable returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) CreateNewTask(opts *bind.TransactOpts, batchMerkleRoot [32]byte, batchDataPointer string, respondToTaskFeeLimit *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "createNewTask", batchMerkleRoot, batchDataPointer, respondToTaskFeeLimit)
}

// CreateNewTask is a paid mutator transaction binding the contract method 0xd66eaabd.
//
// Solidity: function createNewTask(bytes32 batchMerkleRoot, string batchDataPointer, uint256 respondToTaskFeeLimit) payable returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) CreateNewTask(batchMerkleRoot [32]byte, batchDataPointer string, respondToTaskFeeLimit *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.CreateNewTask(&_ContractAlignedLayerServiceManager.TransactOpts, batchMerkleRoot, batchDataPointer, respondToTaskFeeLimit)
}

// CreateNewTask is a paid mutator transaction binding the contract method 0xd66eaabd.
//
// Solidity: function createNewTask(bytes32 batchMerkleRoot, string batchDataPointer, uint256 respondToTaskFeeLimit) payable returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) CreateNewTask(batchMerkleRoot [32]byte, batchDataPointer string, respondToTaskFeeLimit *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.CreateNewTask(&_ContractAlignedLayerServiceManager.TransactOpts, batchMerkleRoot, batchDataPointer, respondToTaskFeeLimit)
}

// DepositToBatcher is a paid mutator transaction binding the contract method 0x4223d551.
//
// Solidity: function depositToBatcher(address account) payable returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) DepositToBatcher(opts *bind.TransactOpts, account common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "depositToBatcher", account)
}

// DepositToBatcher is a paid mutator transaction binding the contract method 0x4223d551.
//
// Solidity: function depositToBatcher(address account) payable returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) DepositToBatcher(account common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.DepositToBatcher(&_ContractAlignedLayerServiceManager.TransactOpts, account)
}

// DepositToBatcher is a paid mutator transaction binding the contract method 0x4223d551.
//
// Solidity: function depositToBatcher(address account) payable returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) DepositToBatcher(account common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.DepositToBatcher(&_ContractAlignedLayerServiceManager.TransactOpts, account)
}

// DeregisterOperatorFromAVS is a paid mutator transaction binding the contract method 0xa364f4da.
//
// Solidity: function deregisterOperatorFromAVS(address operator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) DeregisterOperatorFromAVS(opts *bind.TransactOpts, operator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "deregisterOperatorFromAVS", operator)
}

// DeregisterOperatorFromAVS is a paid mutator transaction binding the contract method 0xa364f4da.
//
// Solidity: function deregisterOperatorFromAVS(address operator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) DeregisterOperatorFromAVS(operator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.DeregisterOperatorFromAVS(&_ContractAlignedLayerServiceManager.TransactOpts, operator)
}

// DeregisterOperatorFromAVS is a paid mutator transaction binding the contract method 0xa364f4da.
//
// Solidity: function deregisterOperatorFromAVS(address operator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) DeregisterOperatorFromAVS(operator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.DeregisterOperatorFromAVS(&_ContractAlignedLayerServiceManager.TransactOpts, operator)
}

// Initialize is a paid mutator transaction binding the contract method 0xc0c53b8b.
//
// Solidity: function initialize(address _initialOwner, address _rewardsInitiator, address _alignedAggregator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) Initialize(opts *bind.TransactOpts, _initialOwner common.Address, _rewardsInitiator common.Address, _alignedAggregator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "initialize", _initialOwner, _rewardsInitiator, _alignedAggregator)
}

// Initialize is a paid mutator transaction binding the contract method 0xc0c53b8b.
//
// Solidity: function initialize(address _initialOwner, address _rewardsInitiator, address _alignedAggregator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) Initialize(_initialOwner common.Address, _rewardsInitiator common.Address, _alignedAggregator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.Initialize(&_ContractAlignedLayerServiceManager.TransactOpts, _initialOwner, _rewardsInitiator, _alignedAggregator)
}

// Initialize is a paid mutator transaction binding the contract method 0xc0c53b8b.
//
// Solidity: function initialize(address _initialOwner, address _rewardsInitiator, address _alignedAggregator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) Initialize(_initialOwner common.Address, _rewardsInitiator common.Address, _alignedAggregator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.Initialize(&_ContractAlignedLayerServiceManager.TransactOpts, _initialOwner, _rewardsInitiator, _alignedAggregator)
}

// InitializeAggregator is a paid mutator transaction binding the contract method 0x800fb61f.
//
// Solidity: function initializeAggregator(address _alignedAggregator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) InitializeAggregator(opts *bind.TransactOpts, _alignedAggregator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "initializeAggregator", _alignedAggregator)
}

// InitializeAggregator is a paid mutator transaction binding the contract method 0x800fb61f.
//
// Solidity: function initializeAggregator(address _alignedAggregator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) InitializeAggregator(_alignedAggregator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.InitializeAggregator(&_ContractAlignedLayerServiceManager.TransactOpts, _alignedAggregator)
}

// InitializeAggregator is a paid mutator transaction binding the contract method 0x800fb61f.
//
// Solidity: function initializeAggregator(address _alignedAggregator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) InitializeAggregator(_alignedAggregator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.InitializeAggregator(&_ContractAlignedLayerServiceManager.TransactOpts, _alignedAggregator)
}

// RegisterOperatorToAVS is a paid mutator transaction binding the contract method 0x9926ee7d.
//
// Solidity: function registerOperatorToAVS(address operator, (bytes,bytes32,uint256) operatorSignature) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) RegisterOperatorToAVS(opts *bind.TransactOpts, operator common.Address, operatorSignature ISignatureUtilsSignatureWithSaltAndExpiry) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "registerOperatorToAVS", operator, operatorSignature)
}

// RegisterOperatorToAVS is a paid mutator transaction binding the contract method 0x9926ee7d.
//
// Solidity: function registerOperatorToAVS(address operator, (bytes,bytes32,uint256) operatorSignature) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) RegisterOperatorToAVS(operator common.Address, operatorSignature ISignatureUtilsSignatureWithSaltAndExpiry) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.RegisterOperatorToAVS(&_ContractAlignedLayerServiceManager.TransactOpts, operator, operatorSignature)
}

// RegisterOperatorToAVS is a paid mutator transaction binding the contract method 0x9926ee7d.
//
// Solidity: function registerOperatorToAVS(address operator, (bytes,bytes32,uint256) operatorSignature) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) RegisterOperatorToAVS(operator common.Address, operatorSignature ISignatureUtilsSignatureWithSaltAndExpiry) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.RegisterOperatorToAVS(&_ContractAlignedLayerServiceManager.TransactOpts, operator, operatorSignature)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) RenounceOwnership(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "renounceOwnership")
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.RenounceOwnership(&_ContractAlignedLayerServiceManager.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.RenounceOwnership(&_ContractAlignedLayerServiceManager.TransactOpts)
}

// RespondToTaskV2 is a paid mutator transaction binding the contract method 0xab21739a.
//
// Solidity: function respondToTaskV2(bytes32 batchMerkleRoot, address senderAddress, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) RespondToTaskV2(opts *bind.TransactOpts, batchMerkleRoot [32]byte, senderAddress common.Address, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "respondToTaskV2", batchMerkleRoot, senderAddress, nonSignerStakesAndSignature)
}

// RespondToTaskV2 is a paid mutator transaction binding the contract method 0xab21739a.
//
// Solidity: function respondToTaskV2(bytes32 batchMerkleRoot, address senderAddress, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) RespondToTaskV2(batchMerkleRoot [32]byte, senderAddress common.Address, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.RespondToTaskV2(&_ContractAlignedLayerServiceManager.TransactOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature)
}

// RespondToTaskV2 is a paid mutator transaction binding the contract method 0xab21739a.
//
// Solidity: function respondToTaskV2(bytes32 batchMerkleRoot, address senderAddress, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) RespondToTaskV2(batchMerkleRoot [32]byte, senderAddress common.Address, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.RespondToTaskV2(&_ContractAlignedLayerServiceManager.TransactOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature)
}

// SetAggregator is a paid mutator transaction binding the contract method 0xf9120af6.
//
// Solidity: function setAggregator(address _alignedAggregator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) SetAggregator(opts *bind.TransactOpts, _alignedAggregator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "setAggregator", _alignedAggregator)
}

// SetAggregator is a paid mutator transaction binding the contract method 0xf9120af6.
//
// Solidity: function setAggregator(address _alignedAggregator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) SetAggregator(_alignedAggregator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.SetAggregator(&_ContractAlignedLayerServiceManager.TransactOpts, _alignedAggregator)
}

// SetAggregator is a paid mutator transaction binding the contract method 0xf9120af6.
//
// Solidity: function setAggregator(address _alignedAggregator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) SetAggregator(_alignedAggregator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.SetAggregator(&_ContractAlignedLayerServiceManager.TransactOpts, _alignedAggregator)
}

// SetRewardsInitiator is a paid mutator transaction binding the contract method 0x3bc28c8c.
//
// Solidity: function setRewardsInitiator(address newRewardsInitiator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) SetRewardsInitiator(opts *bind.TransactOpts, newRewardsInitiator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "setRewardsInitiator", newRewardsInitiator)
}

// SetRewardsInitiator is a paid mutator transaction binding the contract method 0x3bc28c8c.
//
// Solidity: function setRewardsInitiator(address newRewardsInitiator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) SetRewardsInitiator(newRewardsInitiator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.SetRewardsInitiator(&_ContractAlignedLayerServiceManager.TransactOpts, newRewardsInitiator)
}

// SetRewardsInitiator is a paid mutator transaction binding the contract method 0x3bc28c8c.
//
// Solidity: function setRewardsInitiator(address newRewardsInitiator) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) SetRewardsInitiator(newRewardsInitiator common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.SetRewardsInitiator(&_ContractAlignedLayerServiceManager.TransactOpts, newRewardsInitiator)
}

// SetStaleStakesForbidden is a paid mutator transaction binding the contract method 0x416c7e5e.
//
// Solidity: function setStaleStakesForbidden(bool value) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) SetStaleStakesForbidden(opts *bind.TransactOpts, value bool) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "setStaleStakesForbidden", value)
}

// SetStaleStakesForbidden is a paid mutator transaction binding the contract method 0x416c7e5e.
//
// Solidity: function setStaleStakesForbidden(bool value) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) SetStaleStakesForbidden(value bool) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.SetStaleStakesForbidden(&_ContractAlignedLayerServiceManager.TransactOpts, value)
}

// SetStaleStakesForbidden is a paid mutator transaction binding the contract method 0x416c7e5e.
//
// Solidity: function setStaleStakesForbidden(bool value) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) SetStaleStakesForbidden(value bool) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.SetStaleStakesForbidden(&_ContractAlignedLayerServiceManager.TransactOpts, value)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) TransferOwnership(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "transferOwnership", newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.TransferOwnership(&_ContractAlignedLayerServiceManager.TransactOpts, newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.TransferOwnership(&_ContractAlignedLayerServiceManager.TransactOpts, newOwner)
}

// UpdateAVSMetadataURI is a paid mutator transaction binding the contract method 0xa98fb355.
//
// Solidity: function updateAVSMetadataURI(string _metadataURI) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) UpdateAVSMetadataURI(opts *bind.TransactOpts, _metadataURI string) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "updateAVSMetadataURI", _metadataURI)
}

// UpdateAVSMetadataURI is a paid mutator transaction binding the contract method 0xa98fb355.
//
// Solidity: function updateAVSMetadataURI(string _metadataURI) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) UpdateAVSMetadataURI(_metadataURI string) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.UpdateAVSMetadataURI(&_ContractAlignedLayerServiceManager.TransactOpts, _metadataURI)
}

// UpdateAVSMetadataURI is a paid mutator transaction binding the contract method 0xa98fb355.
//
// Solidity: function updateAVSMetadataURI(string _metadataURI) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) UpdateAVSMetadataURI(_metadataURI string) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.UpdateAVSMetadataURI(&_ContractAlignedLayerServiceManager.TransactOpts, _metadataURI)
}

// Withdraw is a paid mutator transaction binding the contract method 0x2e1a7d4d.
//
// Solidity: function withdraw(uint256 amount) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) Withdraw(opts *bind.TransactOpts, amount *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "withdraw", amount)
}

// Withdraw is a paid mutator transaction binding the contract method 0x2e1a7d4d.
//
// Solidity: function withdraw(uint256 amount) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) Withdraw(amount *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.Withdraw(&_ContractAlignedLayerServiceManager.TransactOpts, amount)
}

// Withdraw is a paid mutator transaction binding the contract method 0x2e1a7d4d.
//
// Solidity: function withdraw(uint256 amount) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) Withdraw(amount *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.Withdraw(&_ContractAlignedLayerServiceManager.TransactOpts, amount)
}

// Receive is a paid mutator transaction binding the contract receive function.
//
// Solidity: receive() payable returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) Receive(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.RawTransact(opts, nil) // calldata is disallowed for receive function
}

// Receive is a paid mutator transaction binding the contract receive function.
//
// Solidity: receive() payable returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) Receive() (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.Receive(&_ContractAlignedLayerServiceManager.TransactOpts)
}

// Receive is a paid mutator transaction binding the contract receive function.
//
// Solidity: receive() payable returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) Receive() (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.Receive(&_ContractAlignedLayerServiceManager.TransactOpts)
}

// ContractAlignedLayerServiceManagerBatchVerifiedIterator is returned from FilterBatchVerified and is used to iterate over the raw logs and unpacked data for BatchVerified events raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerBatchVerifiedIterator struct {
	Event *ContractAlignedLayerServiceManagerBatchVerified // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractAlignedLayerServiceManagerBatchVerifiedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractAlignedLayerServiceManagerBatchVerified)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractAlignedLayerServiceManagerBatchVerified)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractAlignedLayerServiceManagerBatchVerifiedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractAlignedLayerServiceManagerBatchVerifiedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractAlignedLayerServiceManagerBatchVerified represents a BatchVerified event raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerBatchVerified struct {
	BatchMerkleRoot [32]byte
	SenderAddress   common.Address
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterBatchVerified is a free log retrieval operation binding the contract event 0x8511746b73275e06971968773119b9601fc501d7bdf3824d8754042d148940e2.
//
// Solidity: event BatchVerified(bytes32 indexed batchMerkleRoot, address senderAddress)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) FilterBatchVerified(opts *bind.FilterOpts, batchMerkleRoot [][32]byte) (*ContractAlignedLayerServiceManagerBatchVerifiedIterator, error) {

	var batchMerkleRootRule []interface{}
	for _, batchMerkleRootItem := range batchMerkleRoot {
		batchMerkleRootRule = append(batchMerkleRootRule, batchMerkleRootItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.FilterLogs(opts, "BatchVerified", batchMerkleRootRule)
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerBatchVerifiedIterator{contract: _ContractAlignedLayerServiceManager.contract, event: "BatchVerified", logs: logs, sub: sub}, nil
}

// WatchBatchVerified is a free log subscription operation binding the contract event 0x8511746b73275e06971968773119b9601fc501d7bdf3824d8754042d148940e2.
//
// Solidity: event BatchVerified(bytes32 indexed batchMerkleRoot, address senderAddress)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) WatchBatchVerified(opts *bind.WatchOpts, sink chan<- *ContractAlignedLayerServiceManagerBatchVerified, batchMerkleRoot [][32]byte) (event.Subscription, error) {

	var batchMerkleRootRule []interface{}
	for _, batchMerkleRootItem := range batchMerkleRoot {
		batchMerkleRootRule = append(batchMerkleRootRule, batchMerkleRootItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.WatchLogs(opts, "BatchVerified", batchMerkleRootRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractAlignedLayerServiceManagerBatchVerified)
				if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "BatchVerified", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseBatchVerified is a log parse operation binding the contract event 0x8511746b73275e06971968773119b9601fc501d7bdf3824d8754042d148940e2.
//
// Solidity: event BatchVerified(bytes32 indexed batchMerkleRoot, address senderAddress)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) ParseBatchVerified(log types.Log) (*ContractAlignedLayerServiceManagerBatchVerified, error) {
	event := new(ContractAlignedLayerServiceManagerBatchVerified)
	if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "BatchVerified", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractAlignedLayerServiceManagerBatcherBalanceUpdatedIterator is returned from FilterBatcherBalanceUpdated and is used to iterate over the raw logs and unpacked data for BatcherBalanceUpdated events raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerBatcherBalanceUpdatedIterator struct {
	Event *ContractAlignedLayerServiceManagerBatcherBalanceUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractAlignedLayerServiceManagerBatcherBalanceUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractAlignedLayerServiceManagerBatcherBalanceUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractAlignedLayerServiceManagerBatcherBalanceUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractAlignedLayerServiceManagerBatcherBalanceUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractAlignedLayerServiceManagerBatcherBalanceUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractAlignedLayerServiceManagerBatcherBalanceUpdated represents a BatcherBalanceUpdated event raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerBatcherBalanceUpdated struct {
	Batcher    common.Address
	NewBalance *big.Int
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterBatcherBalanceUpdated is a free log retrieval operation binding the contract event 0x0ea46f246ccfc58f7a93aa09bc6245a6818e97b1a160d186afe78993a3b194a0.
//
// Solidity: event BatcherBalanceUpdated(address indexed batcher, uint256 newBalance)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) FilterBatcherBalanceUpdated(opts *bind.FilterOpts, batcher []common.Address) (*ContractAlignedLayerServiceManagerBatcherBalanceUpdatedIterator, error) {

	var batcherRule []interface{}
	for _, batcherItem := range batcher {
		batcherRule = append(batcherRule, batcherItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.FilterLogs(opts, "BatcherBalanceUpdated", batcherRule)
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerBatcherBalanceUpdatedIterator{contract: _ContractAlignedLayerServiceManager.contract, event: "BatcherBalanceUpdated", logs: logs, sub: sub}, nil
}

// WatchBatcherBalanceUpdated is a free log subscription operation binding the contract event 0x0ea46f246ccfc58f7a93aa09bc6245a6818e97b1a160d186afe78993a3b194a0.
//
// Solidity: event BatcherBalanceUpdated(address indexed batcher, uint256 newBalance)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) WatchBatcherBalanceUpdated(opts *bind.WatchOpts, sink chan<- *ContractAlignedLayerServiceManagerBatcherBalanceUpdated, batcher []common.Address) (event.Subscription, error) {

	var batcherRule []interface{}
	for _, batcherItem := range batcher {
		batcherRule = append(batcherRule, batcherItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.WatchLogs(opts, "BatcherBalanceUpdated", batcherRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractAlignedLayerServiceManagerBatcherBalanceUpdated)
				if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "BatcherBalanceUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseBatcherBalanceUpdated is a log parse operation binding the contract event 0x0ea46f246ccfc58f7a93aa09bc6245a6818e97b1a160d186afe78993a3b194a0.
//
// Solidity: event BatcherBalanceUpdated(address indexed batcher, uint256 newBalance)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) ParseBatcherBalanceUpdated(log types.Log) (*ContractAlignedLayerServiceManagerBatcherBalanceUpdated, error) {
	event := new(ContractAlignedLayerServiceManagerBatcherBalanceUpdated)
	if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "BatcherBalanceUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractAlignedLayerServiceManagerInitializedIterator is returned from FilterInitialized and is used to iterate over the raw logs and unpacked data for Initialized events raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerInitializedIterator struct {
	Event *ContractAlignedLayerServiceManagerInitialized // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractAlignedLayerServiceManagerInitializedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractAlignedLayerServiceManagerInitialized)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractAlignedLayerServiceManagerInitialized)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractAlignedLayerServiceManagerInitializedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractAlignedLayerServiceManagerInitializedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractAlignedLayerServiceManagerInitialized represents a Initialized event raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerInitialized struct {
	Version uint8
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterInitialized is a free log retrieval operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) FilterInitialized(opts *bind.FilterOpts) (*ContractAlignedLayerServiceManagerInitializedIterator, error) {

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.FilterLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerInitializedIterator{contract: _ContractAlignedLayerServiceManager.contract, event: "Initialized", logs: logs, sub: sub}, nil
}

// WatchInitialized is a free log subscription operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) WatchInitialized(opts *bind.WatchOpts, sink chan<- *ContractAlignedLayerServiceManagerInitialized) (event.Subscription, error) {

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.WatchLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractAlignedLayerServiceManagerInitialized)
				if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "Initialized", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseInitialized is a log parse operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) ParseInitialized(log types.Log) (*ContractAlignedLayerServiceManagerInitialized, error) {
	event := new(ContractAlignedLayerServiceManagerInitialized)
	if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "Initialized", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractAlignedLayerServiceManagerNewBatchV2Iterator is returned from FilterNewBatchV2 and is used to iterate over the raw logs and unpacked data for NewBatchV2 events raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerNewBatchV2Iterator struct {
	Event *ContractAlignedLayerServiceManagerNewBatchV2 // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractAlignedLayerServiceManagerNewBatchV2Iterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractAlignedLayerServiceManagerNewBatchV2)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractAlignedLayerServiceManagerNewBatchV2)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractAlignedLayerServiceManagerNewBatchV2Iterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractAlignedLayerServiceManagerNewBatchV2Iterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractAlignedLayerServiceManagerNewBatchV2 represents a NewBatchV2 event raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerNewBatchV2 struct {
	BatchMerkleRoot  [32]byte
	SenderAddress    common.Address
	TaskCreatedBlock uint32
	BatchDataPointer string
	Raw              types.Log // Blockchain specific contextual infos
}

// FilterNewBatchV2 is a free log retrieval operation binding the contract event 0x130d3e81af62e03ed6fff5e3bb343695ec513892cfad24d286486745dcc61437.
//
// Solidity: event NewBatchV2(bytes32 indexed batchMerkleRoot, address senderAddress, uint32 taskCreatedBlock, string batchDataPointer)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) FilterNewBatchV2(opts *bind.FilterOpts, batchMerkleRoot [][32]byte) (*ContractAlignedLayerServiceManagerNewBatchV2Iterator, error) {

	var batchMerkleRootRule []interface{}
	for _, batchMerkleRootItem := range batchMerkleRoot {
		batchMerkleRootRule = append(batchMerkleRootRule, batchMerkleRootItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.FilterLogs(opts, "NewBatchV2", batchMerkleRootRule)
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerNewBatchV2Iterator{contract: _ContractAlignedLayerServiceManager.contract, event: "NewBatchV2", logs: logs, sub: sub}, nil
}

// WatchNewBatchV2 is a free log subscription operation binding the contract event 0x130d3e81af62e03ed6fff5e3bb343695ec513892cfad24d286486745dcc61437.
//
// Solidity: event NewBatchV2(bytes32 indexed batchMerkleRoot, address senderAddress, uint32 taskCreatedBlock, string batchDataPointer)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) WatchNewBatchV2(opts *bind.WatchOpts, sink chan<- *ContractAlignedLayerServiceManagerNewBatchV2, batchMerkleRoot [][32]byte) (event.Subscription, error) {

	var batchMerkleRootRule []interface{}
	for _, batchMerkleRootItem := range batchMerkleRoot {
		batchMerkleRootRule = append(batchMerkleRootRule, batchMerkleRootItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.WatchLogs(opts, "NewBatchV2", batchMerkleRootRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractAlignedLayerServiceManagerNewBatchV2)
				if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "NewBatchV2", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseNewBatchV2 is a log parse operation binding the contract event 0x130d3e81af62e03ed6fff5e3bb343695ec513892cfad24d286486745dcc61437.
//
// Solidity: event NewBatchV2(bytes32 indexed batchMerkleRoot, address senderAddress, uint32 taskCreatedBlock, string batchDataPointer)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) ParseNewBatchV2(log types.Log) (*ContractAlignedLayerServiceManagerNewBatchV2, error) {
	event := new(ContractAlignedLayerServiceManagerNewBatchV2)
	if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "NewBatchV2", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractAlignedLayerServiceManagerNewBatchV3Iterator is returned from FilterNewBatchV3 and is used to iterate over the raw logs and unpacked data for NewBatchV3 events raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerNewBatchV3Iterator struct {
	Event *ContractAlignedLayerServiceManagerNewBatchV3 // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractAlignedLayerServiceManagerNewBatchV3Iterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractAlignedLayerServiceManagerNewBatchV3)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractAlignedLayerServiceManagerNewBatchV3)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractAlignedLayerServiceManagerNewBatchV3Iterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractAlignedLayerServiceManagerNewBatchV3Iterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractAlignedLayerServiceManagerNewBatchV3 represents a NewBatchV3 event raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerNewBatchV3 struct {
	BatchMerkleRoot       [32]byte
	SenderAddress         common.Address
	TaskCreatedBlock      uint32
	BatchDataPointer      string
	RespondToTaskFeeLimit *big.Int
	Raw                   types.Log // Blockchain specific contextual infos
}

// FilterNewBatchV3 is a free log retrieval operation binding the contract event 0x8801fc966deb2c8f563a103c35c9e80740585c292cd97518587e6e7927e6af55.
//
// Solidity: event NewBatchV3(bytes32 indexed batchMerkleRoot, address senderAddress, uint32 taskCreatedBlock, string batchDataPointer, uint256 respondToTaskFeeLimit)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) FilterNewBatchV3(opts *bind.FilterOpts, batchMerkleRoot [][32]byte) (*ContractAlignedLayerServiceManagerNewBatchV3Iterator, error) {

	var batchMerkleRootRule []interface{}
	for _, batchMerkleRootItem := range batchMerkleRoot {
		batchMerkleRootRule = append(batchMerkleRootRule, batchMerkleRootItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.FilterLogs(opts, "NewBatchV3", batchMerkleRootRule)
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerNewBatchV3Iterator{contract: _ContractAlignedLayerServiceManager.contract, event: "NewBatchV3", logs: logs, sub: sub}, nil
}

// WatchNewBatchV3 is a free log subscription operation binding the contract event 0x8801fc966deb2c8f563a103c35c9e80740585c292cd97518587e6e7927e6af55.
//
// Solidity: event NewBatchV3(bytes32 indexed batchMerkleRoot, address senderAddress, uint32 taskCreatedBlock, string batchDataPointer, uint256 respondToTaskFeeLimit)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) WatchNewBatchV3(opts *bind.WatchOpts, sink chan<- *ContractAlignedLayerServiceManagerNewBatchV3, batchMerkleRoot [][32]byte) (event.Subscription, error) {

	var batchMerkleRootRule []interface{}
	for _, batchMerkleRootItem := range batchMerkleRoot {
		batchMerkleRootRule = append(batchMerkleRootRule, batchMerkleRootItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.WatchLogs(opts, "NewBatchV3", batchMerkleRootRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractAlignedLayerServiceManagerNewBatchV3)
				if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "NewBatchV3", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseNewBatchV3 is a log parse operation binding the contract event 0x8801fc966deb2c8f563a103c35c9e80740585c292cd97518587e6e7927e6af55.
//
// Solidity: event NewBatchV3(bytes32 indexed batchMerkleRoot, address senderAddress, uint32 taskCreatedBlock, string batchDataPointer, uint256 respondToTaskFeeLimit)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) ParseNewBatchV3(log types.Log) (*ContractAlignedLayerServiceManagerNewBatchV3, error) {
	event := new(ContractAlignedLayerServiceManagerNewBatchV3)
	if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "NewBatchV3", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractAlignedLayerServiceManagerOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerOwnershipTransferredIterator struct {
	Event *ContractAlignedLayerServiceManagerOwnershipTransferred // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractAlignedLayerServiceManagerOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractAlignedLayerServiceManagerOwnershipTransferred)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractAlignedLayerServiceManagerOwnershipTransferred)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractAlignedLayerServiceManagerOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractAlignedLayerServiceManagerOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractAlignedLayerServiceManagerOwnershipTransferred represents a OwnershipTransferred event raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerOwnershipTransferred struct {
	PreviousOwner common.Address
	NewOwner      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, previousOwner []common.Address, newOwner []common.Address) (*ContractAlignedLayerServiceManagerOwnershipTransferredIterator, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.FilterLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerOwnershipTransferredIterator{contract: _ContractAlignedLayerServiceManager.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *ContractAlignedLayerServiceManagerOwnershipTransferred, previousOwner []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.WatchLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractAlignedLayerServiceManagerOwnershipTransferred)
				if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOwnershipTransferred is a log parse operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) ParseOwnershipTransferred(log types.Log) (*ContractAlignedLayerServiceManagerOwnershipTransferred, error) {
	event := new(ContractAlignedLayerServiceManagerOwnershipTransferred)
	if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractAlignedLayerServiceManagerRewardsInitiatorUpdatedIterator is returned from FilterRewardsInitiatorUpdated and is used to iterate over the raw logs and unpacked data for RewardsInitiatorUpdated events raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerRewardsInitiatorUpdatedIterator struct {
	Event *ContractAlignedLayerServiceManagerRewardsInitiatorUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractAlignedLayerServiceManagerRewardsInitiatorUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractAlignedLayerServiceManagerRewardsInitiatorUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractAlignedLayerServiceManagerRewardsInitiatorUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractAlignedLayerServiceManagerRewardsInitiatorUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractAlignedLayerServiceManagerRewardsInitiatorUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractAlignedLayerServiceManagerRewardsInitiatorUpdated represents a RewardsInitiatorUpdated event raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerRewardsInitiatorUpdated struct {
	PrevRewardsInitiator common.Address
	NewRewardsInitiator  common.Address
	Raw                  types.Log // Blockchain specific contextual infos
}

// FilterRewardsInitiatorUpdated is a free log retrieval operation binding the contract event 0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3.
//
// Solidity: event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) FilterRewardsInitiatorUpdated(opts *bind.FilterOpts) (*ContractAlignedLayerServiceManagerRewardsInitiatorUpdatedIterator, error) {

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.FilterLogs(opts, "RewardsInitiatorUpdated")
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerRewardsInitiatorUpdatedIterator{contract: _ContractAlignedLayerServiceManager.contract, event: "RewardsInitiatorUpdated", logs: logs, sub: sub}, nil
}

// WatchRewardsInitiatorUpdated is a free log subscription operation binding the contract event 0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3.
//
// Solidity: event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) WatchRewardsInitiatorUpdated(opts *bind.WatchOpts, sink chan<- *ContractAlignedLayerServiceManagerRewardsInitiatorUpdated) (event.Subscription, error) {

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.WatchLogs(opts, "RewardsInitiatorUpdated")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractAlignedLayerServiceManagerRewardsInitiatorUpdated)
				if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "RewardsInitiatorUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseRewardsInitiatorUpdated is a log parse operation binding the contract event 0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3.
//
// Solidity: event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) ParseRewardsInitiatorUpdated(log types.Log) (*ContractAlignedLayerServiceManagerRewardsInitiatorUpdated, error) {
	event := new(ContractAlignedLayerServiceManagerRewardsInitiatorUpdated)
	if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "RewardsInitiatorUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdateIterator is returned from FilterStaleStakesForbiddenUpdate and is used to iterate over the raw logs and unpacked data for StaleStakesForbiddenUpdate events raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdateIterator struct {
	Event *ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdate // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdateIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdate)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdate)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdateIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdateIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdate represents a StaleStakesForbiddenUpdate event raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdate struct {
	Value bool
	Raw   types.Log // Blockchain specific contextual infos
}

// FilterStaleStakesForbiddenUpdate is a free log retrieval operation binding the contract event 0x40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc.
//
// Solidity: event StaleStakesForbiddenUpdate(bool value)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) FilterStaleStakesForbiddenUpdate(opts *bind.FilterOpts) (*ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdateIterator, error) {

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.FilterLogs(opts, "StaleStakesForbiddenUpdate")
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdateIterator{contract: _ContractAlignedLayerServiceManager.contract, event: "StaleStakesForbiddenUpdate", logs: logs, sub: sub}, nil
}

// WatchStaleStakesForbiddenUpdate is a free log subscription operation binding the contract event 0x40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc.
//
// Solidity: event StaleStakesForbiddenUpdate(bool value)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) WatchStaleStakesForbiddenUpdate(opts *bind.WatchOpts, sink chan<- *ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdate) (event.Subscription, error) {

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.WatchLogs(opts, "StaleStakesForbiddenUpdate")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdate)
				if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "StaleStakesForbiddenUpdate", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseStaleStakesForbiddenUpdate is a log parse operation binding the contract event 0x40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc.
//
// Solidity: event StaleStakesForbiddenUpdate(bool value)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) ParseStaleStakesForbiddenUpdate(log types.Log) (*ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdate, error) {
	event := new(ContractAlignedLayerServiceManagerStaleStakesForbiddenUpdate)
	if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "StaleStakesForbiddenUpdate", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
