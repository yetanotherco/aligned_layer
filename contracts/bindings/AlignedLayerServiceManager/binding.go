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
	Bin: "0x6101806040523480156200001257600080fd5b50604051620056fe380380620056fe8339810160408190526200003591620002e5565b6001600160a01b0380851660805280841660a05280831660c052811660e0528184848284620000636200020a565b50505050806001600160a01b0316610100816001600160a01b031681525050806001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015620000c1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000e791906200034d565b6001600160a01b0316610120816001600160a01b031681525050806001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000140573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200016691906200034d565b6001600160a01b0316610140816001600160a01b031681525050610120516001600160a01b031663df5cf7236040518163ffffffff1660e01b8152600401602060405180830381865afa158015620001c2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001e891906200034d565b6001600160a01b03166101605250620002006200020a565b5050505062000374565b600054610100900460ff1615620002775760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff9081161015620002ca576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b0381168114620002e257600080fd5b50565b60008060008060808587031215620002fc57600080fd5b84516200030981620002cc565b60208601519094506200031c81620002cc565b60408601519093506200032f81620002cc565b60608601519092506200034281620002cc565b939692955090935050565b6000602082840312156200036057600080fd5b81516200036d81620002cc565b9392505050565b60805160a05160c05160e0516101005161012051610140516101605161527c620004826000396000818161064901526117ca01526000818161038c01526119dd0152600081816103c001528181611bca0152611dba01526000818161042701528181610ff2015281816114860152818161162d015261187e015260008181610d0701528181610e6201528181610ef901528181612aa801528181612c2b0152612cca015260008181610b2e01528181610bbd01528181610c3d0152818161216801528181612234015281816129e60152612b8601526000818161309701528181613153015261323f0152600081816103f1015281816121bc01528181612290015261230f015261527c6000f3fe6080604052600436106101f25760003560e01c80638da5cb5b1161010d578063c0c53b8b116100a0578063f2fde38b1161006f578063f2fde38b14610680578063f9120af6146106a0578063fa534dc0146106c0578063fc299dee146106e0578063fce36c7d1461070057600080fd5b8063c0c53b8b14610604578063d66eaabd14610624578063df5cf72314610637578063e481af9d1461066b57600080fd5b8063a98fb355116100dc578063a98fb35514610540578063ab21739a14610560578063b099627e14610580578063b98d0908146105ea57600080fd5b80638da5cb5b146104c257806395c6d604146104e05780639926ee7d14610500578063a364f4da1461052057600080fd5b80634ae07c37116101855780636d14a987116101545780636d14a9871461041557806370a0823114610449578063715018a61461048d578063800fb61f146104a257600080fd5b80634ae07c371461034c5780635df459461461037a57806368304835146103ae5780636b3aa72e146103e257600080fd5b80633bc28c8c116101c15780633bc28c8c146102c1578063416c7e5e146102e15780634223d551146103015780634a5bf6321461031457600080fd5b806306045a9114610208578063171f1d5b1461023d5780632e1a7d4d1461027457806333cfb7b71461029457600080fd5b36610203576102013334610720565b005b600080fd5b34801561021457600080fd5b506102286102233660046141cc565b6107b2565b60405190151581526020015b60405180910390f35b34801561024957600080fd5b5061025d61025836600461431e565b6108a7565b604080519215158352901515602083015201610234565b34801561028057600080fd5b5061020161028f36600461436f565b610a31565b3480156102a057600080fd5b506102b46102af366004614388565b610b09565b60405161023491906143a5565b3480156102cd57600080fd5b506102016102dc366004614388565b610fdc565b3480156102ed57600080fd5b506102016102fc366004614400565b610ff0565b61020161030f366004614388565b611127565b34801561032057600080fd5b5060cb54610334906001600160a01b031681565b6040516001600160a01b039091168152602001610234565b34801561035857600080fd5b5061036c6103673660046146d3565b611131565b60405161023492919061476d565b34801561038657600080fd5b506103347f000000000000000000000000000000000000000000000000000000000000000081565b3480156103ba57600080fd5b506103347f000000000000000000000000000000000000000000000000000000000000000081565b3480156103ee57600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610334565b34801561042157600080fd5b506103347f000000000000000000000000000000000000000000000000000000000000000081565b34801561045557600080fd5b5061047f610464366004614388565b6001600160a01b0316600090815260ca602052604090205490565b604051908152602001610234565b34801561049957600080fd5b50610201612084565b3480156104ae57600080fd5b506102016104bd366004614388565b612098565b3480156104ce57600080fd5b506033546001600160a01b0316610334565b3480156104ec57600080fd5b506102286104fb3660046147fe565b612138565b34801561050c57600080fd5b5061020161051b366004614849565b61215d565b34801561052c57600080fd5b5061020161053b366004614388565b612229565b34801561054c57600080fd5b5061020161055b3660046148f4565b6122f0565b34801561056c57600080fd5b5061020161057b366004614944565b612344565b34801561058c57600080fd5b506105c861059b36600461436f565b60c9602052600090815260409020805460019091015463ffffffff821691640100000000900460ff169083565b6040805163ffffffff9094168452911515602084015290820152606001610234565b3480156105f657600080fd5b506097546102289060ff1681565b34801561061057600080fd5b5061020161061f36600461496b565b6126fe565b6102016106323660046149b6565b6127e8565b34801561064357600080fd5b506103347f000000000000000000000000000000000000000000000000000000000000000081565b34801561067757600080fd5b506102b46129e0565b34801561068c57600080fd5b5061020161069b366004614388565b612da9565b3480156106ac57600080fd5b506102016106bb366004614388565b612e1f565b3480156106cc57600080fd5b506102286106db366004614a08565b612e49565b3480156106ec57600080fd5b50606554610334906001600160a01b031681565b34801561070c57600080fd5b5061020161071b366004614a85565b612ebe565b8061074657604051632097692160e11b8152600481018290526024015b60405180910390fd5b6001600160a01b038216600090815260ca60205260408120805483929061076e908490614b0f565b90915550506001600160a01b038216600081815260ca6020908152604091829020549151918252600080516020615207833981519152910160405180910390a25050565b6000806001600160a01b0383166107ca5750846107f6565b85836040516020016107dd929190614b27565b6040516020818303038152906040528051906020012090505b600081815260c9602052604090205463ffffffff1661081957600091505061089b565b600081815260c96020526040902054640100000000900460ff1661084157600091505061089b565b60408051602081018c90529081018a9052606081018990526001600160601b03198816608082015260009060940160408051601f198184030181529190528051602082012090915061089587898389613276565b93505050505b98975050505050505050565b60008060007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001878760000151886020015188600001516000600281106108ef576108ef614b42565b60200201518951600160200201518a6020015160006002811061091457610914614b42565b60200201518b6020015160016002811061093057610930614b42565b602090810291909101518c518d83015160405161098d9a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b6040516020818303038152906040528051906020012060001c6109b09190614b58565b9050610a236109c96109c2888461328e565b8690613325565b6109d16133b9565b610a19610a0a85610a04604080518082018252600080825260209182015281518083019092526001825260029082015290565b9061328e565b610a138c613479565b90613325565b886201d4c0613509565b909890975095505050505050565b33600090815260ca6020526040902054811115610a825733600081815260ca602052604090819020549051632e2a182f60e11b8152600481019290925260248201839052604482015260640161073d565b33600090815260ca602052604081208054839290610aa1908490614b7a565b909155505033600081815260ca6020908152604091829020549151918252600080516020615207833981519152910160405180910390a2604051339082156108fc029083906000818181858888f19350505050158015610b05573d6000803e3d6000fd5b5050565b6040516309aa152760e11b81526001600160a01b0382811660048301526060916000917f000000000000000000000000000000000000000000000000000000000000000016906313542a4e90602401602060405180830381865afa158015610b75573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b999190614b91565b60405163871ef04960e01b8152600481018290529091506000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063871ef04990602401602060405180830381865afa158015610c04573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c289190614baa565b90506001600160c01b0381161580610cc257507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c99573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610cbd9190614bd3565b60ff16155b15610ce25760408051600080825260208201909252905b50949350505050565b6000610cf6826001600160c01b031661372d565b90506000805b8251811015610dcc577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316633ca5a5f5848381518110610d4657610d46614b42565b01602001516040516001600160e01b031960e084901b16815260f89190911c6004820152602401602060405180830381865afa158015610d8a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610dae9190614b91565b610db89083614b0f565b915080610dc481614bf6565b915050610cfc565b506000816001600160401b03811115610de757610de76140a8565b604051908082528060200260200182016040528015610e10578160200160208202803683370190505b5090506000805b8451811015610fcf576000858281518110610e3457610e34614b42565b0160200151604051633ca5a5f560e01b815260f89190911c6004820181905291506000906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633ca5a5f590602401602060405180830381865afa158015610ea9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ecd9190614b91565b905060005b81811015610fb9576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa158015610f47573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f6b9190614c26565b60000151868681518110610f8157610f81614b42565b6001600160a01b039092166020928302919091019091015284610fa381614bf6565b9550508080610fb190614bf6565b915050610ed2565b5050508080610fc790614bf6565b915050610e17565b5090979650505050505050565b610fe46137ef565b610fed81613849565b50565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561104e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110729190614c67565b6001600160a01b0316336001600160a01b03161461111e5760405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a40161073d565b610fed816138b2565b610fed8134610720565b6040805180820190915260608082526020820152600082604001515160405180604001604052806001815260200160008152505114801561118d57508260a0015151604051806040016040528060018152602001600081525051145b80156111b457508260c0015151604051806040016040528060018152602001600081525051145b80156111db57508260e0015151604051806040016040528060018152602001600081525051145b6112455760405162461bcd60e51b8152602060048201526041602482015260008051602061522783398151915260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a40161073d565b825151602084015151146112bd5760405162461bcd60e51b815260206004820152604460248201819052600080516020615227833981519152908201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b608482015260a40161073d565b4363ffffffff168463ffffffff161061132c5760405162461bcd60e51b815260206004820152603c602482015260008051602061522783398151915260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b00000000606482015260840161073d565b60408051808201825260008082526020808301829052835180850185526060808252818301528451808601865260018082529083019390935284518381528086019095529293919082810190803683370190505060208281019190915260408051808201825260018082526000919093015280518281528082019091529081602001602082028036833701905050815260408051808201909152606080825260208201528560200151516001600160401b038111156113ed576113ed6140a8565b604051908082528060200260200182016040528015611416578160200160208202803683370190505b5081526020860151516001600160401b03811115611436576114366140a8565b60405190808252806020026020018201604052801561145f578160200160208202803683370190505b508160200181905250600061150b60405180604001604052806001815260200160008152507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156114e2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115069190614bd3565b6138f9565b905060005b8760200151518110156117a6576115558860200151828151811061153657611536614b42565b6020026020010151805160009081526020918201519091526040902090565b8360200151828151811061156b5761156b614b42565b6020908102919091010152801561162b57602083015161158c600183614b7a565b8151811061159c5761159c614b42565b602002602001015160001c836020015182815181106115bd576115bd614b42565b602002602001015160001c1161162b576040805162461bcd60e51b815260206004820152602481019190915260008051602061522783398151915260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f72746564606482015260840161073d565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166304ec63518460200151838151811061167057611670614b42565b60200260200101518b8b60000151858151811061168f5761168f614b42565b60200260200101516040518463ffffffff1660e01b81526004016116cc9392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa1580156116e9573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061170d9190614baa565b6001600160c01b03168360000151828151811061172c5761172c614b42565b6020026020010181815250506117926109c2611766848660000151858151811061175857611758614b42565b60200260200101511661398c565b8a60200151848151811061177c5761177c614b42565b60200260200101516139b790919063ffffffff16565b94508061179e81614bf6565b915050611510565b50506117b183613a9b565b60975490935060ff166000816117c857600061184a565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c448feb86040518163ffffffff1660e01b8152600401602060405180830381865afa158015611826573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061184a9190614b91565b905060005b604051806040016040528060018152602001600081525051811015611f555782156119db578963ffffffff16827f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663249a0c42604051806040016040528060018152602001600081525085815181106118d3576118d3614b42565b01602001516040516001600160e01b031960e084901b16815260f89190911c6004820152602401602060405180830381865afa158015611917573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061193b9190614b91565b6119459190614b0f565b116119db5760405162461bcd60e51b8152602060048201526066602482015260008051602061522783398151915260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c40161073d565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166368bccaac60405180604001604052806001815260200160008152508381518110611a3257611a32614b42565b602001015160f81c60f81b60f81c8c8c60a001518581518110611a5757611a57614b42565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015611ab3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ad79190614c84565b6001600160401b031916611afa8a60400151838151811061153657611536614b42565b67ffffffffffffffff191614611b965760405162461bcd60e51b8152602060048201526061602482015260008051602061522783398151915260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c40161073d565b611bc689604001518281518110611baf57611baf614b42565b60200260200101518761332590919063ffffffff16565b95507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c8294c5660405180604001604052806001815260200160008152508381518110611c1f57611c1f614b42565b602001015160f81c60f81b60f81c8c8c60c001518581518110611c4457611c44614b42565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015611ca0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611cc49190614caf565b85602001518281518110611cda57611cda614b42565b6001600160601b03909216602092830291909101820152850151805182908110611d0657611d06614b42565b602002602001015185600001518281518110611d2457611d24614b42565b60200260200101906001600160601b031690816001600160601b0316815250506000805b8a6020015151811015611f4057611db386600001518281518110611d6e57611d6e614b42565b602002602001015160405180604001604052806001815260200160008152508581518110611d9e57611d9e614b42565b016020015160f81c60ff161c60019081161490565b15611f2e577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f2be94ae60405180604001604052806001815260200160008152508581518110611e0f57611e0f614b42565b602001015160f81c60f81b60f81c8e89602001518581518110611e3457611e34614b42565b60200260200101518f60e001518881518110611e5257611e52614b42565b60200260200101518781518110611e6b57611e6b614b42565b60209081029190910101516040516001600160e01b031960e087901b16815260ff909416600485015263ffffffff92831660248501526044840191909152166064820152608401602060405180830381865afa158015611ecf573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ef39190614caf565b8751805185908110611f0757611f07614b42565b60200260200101818151611f1b9190614ccc565b6001600160601b03169052506001909101905b80611f3881614bf6565b915050611d48565b50508080611f4d90614bf6565b91505061184f565b505050600080611f6f8a868a606001518b608001516108a7565b9150915081611fe05760405162461bcd60e51b8152602060048201526043602482015260008051602061522783398151915260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a40161073d565b806120415760405162461bcd60e51b8152602060048201526039602482015260008051602061522783398151915260448201527f7265733a207369676e617475726520697320696e76616c696400000000000000606482015260840161073d565b5050600087826020015160405160200161205c929190614cf4565b60408051808303601f1901815291905280516020909101209299929850919650505050505050565b61208c6137ef565b6120966000613b36565b565b600054600290610100900460ff161580156120ba575060005460ff8083169116105b6120d65760405162461bcd60e51b815260040161073d90614d3c565b6000805461ffff191660ff8316176101001790556120f382612e1f565b6000805461ff001916905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b600081848460405161214b929190614d8a565b60405180910390201490509392505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146121a55760405162461bcd60e51b815260040161073d90614d9a565b604051639926ee7d60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639926ee7d906121f39085908590600401614e5f565b600060405180830381600087803b15801561220d57600080fd5b505af1158015612221573d6000803e3d6000fd5b505050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146122715760405162461bcd60e51b815260040161073d90614d9a565b6040516351b27a6d60e11b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a364f4da906024015b600060405180830381600087803b1580156122d557600080fd5b505af11580156122e9573d6000803e3d6000fd5b5050505050565b6122f86137ef565b60405163a98fb35560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a98fb355906122bb908490600401614eaa565b60cb546001600160a01b031633146123845760cb54604051632cbe419560e01b81523360048201526001600160a01b03909116602482015260440161073d565b60005a90506000848460405160200161239e929190614b27565b60408051601f198184030181529181528151602092830120600081815260c9909352912080549192509063ffffffff166123ee576040516311cb69a760e11b81526004810183905260240161073d565b8054640100000000900460ff161561241c57604051634e78d7f960e11b81526004810183905260240161073d565b805464ff00000000191664010000000017815560018101546001600160a01b038616600090815260ca6020526040902054101561249f5760018101546001600160a01b038616600081815260ca602052604090819020549051632e2a182f60e11b815260048101929092526024820192909252604481019190915260640161073d565b80546000906124b690849063ffffffff1687611131565b509050604360ff1681602001516000815181106124d5576124d5614b42565b60200260200101516124e79190614ebd565b6001600160601b03166064826000015160008151811061250957612509614b42565b60200260200101516001600160601b03166125249190614eec565b10156125b7576064816000015160008151811061254357612543614b42565b60200260200101516001600160601b031661255e9190614eec565b604360ff16826020015160008151811061257a5761257a614b42565b602002602001015161258c9190614ebd565b60405163530f5c4560e11b815260048101929092526001600160601b0316602482015260440161073d565b6040516001600160a01b038716815287907f8511746b73275e06971968773119b9601fc501d7bdf3824d8754042d148940e29060200160405180910390a260003a5a6126039087614b7a565b6126109062011170614b0f565b61261a9190614eec565b9050826001015481111561265157600183015460405163437e283f60e11b815260048101919091526024810182905260440161073d565b6001600160a01b038716600090815260ca602052604081208054839290612679908490614b7a565b90915550506001600160a01b038716600081815260ca6020908152604091829020549151918252600080516020615207833981519152910160405180910390a260cb546040516001600160a01b039091169082156108fc029083906000818181858888f193505050501580156126f3573d6000803e3d6000fd5b505050505050505050565b600054610100900460ff161580801561271e5750600054600160ff909116105b806127385750303b158015612738575060005460ff166001145b6127545760405162461bcd60e51b815260040161073d90614d3c565b6000805460ff191660011790558015612777576000805461ff0019166101001790555b6127818484613b88565b60cb80546001600160a01b0319166001600160a01b03841617905580156127e2576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b600084336040516020016127fd929190614b27565b60408051601f198184030181529181528151602092830120600081815260c990935291205490915063ffffffff161561284c57604051630c40bc4360e21b81526004810182905260240161073d565b34156128a95733600090815260ca602052604081208054349290612871908490614b0f565b909155505033600081815260ca6020908152604091829020549151918252600080516020615207833981519152910160405180910390a25b33600090815260ca60205260409020548211156128fa5733600081815260ca602052604090819020549051632e2a182f60e11b8152600481019290925260248201849052604482015260640161073d565b604080516060810182526000602080830182815263ffffffff43818116865285870189815288865260c99094529386902085518154935115156401000000000264ffffffffff1990941692169190911791909117815590516001909101559151909187917f130d3e81af62e03ed6fff5e3bb343695ec513892cfad24d286486745dcc6143791612990913391908a908a90614f34565b60405180910390a2857f8801fc966deb2c8f563a103c35c9e80740585c292cd97518587e6e7927e6af5533438888886040516129d0959493929190614f6f565b60405180910390a2505050505050565b606060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612a42573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612a669190614bd3565b60ff16905080612a8457505060408051600081526020810190915290565b6000805b82811015612b3957604051633ca5a5f560e01b815260ff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa158015612af7573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612b1b9190614b91565b612b259083614b0f565b915080612b3181614bf6565b915050612a88565b506000816001600160401b03811115612b5457612b546140a8565b604051908082528060200260200182016040528015612b7d578160200160208202803683370190505b5090506000805b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612be2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612c069190614bd3565b60ff16811015612d9f57604051633ca5a5f560e01b815260ff821660048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa158015612c7a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612c9e9190614b91565b905060005b81811015612d8a576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa158015612d18573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612d3c9190614c26565b60000151858581518110612d5257612d52614b42565b6001600160a01b039092166020928302919091019091015283612d7481614bf6565b9450508080612d8290614bf6565b915050612ca3565b50508080612d9790614bf6565b915050612b84565b5090949350505050565b612db16137ef565b6001600160a01b038116612e165760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161073d565b610fed81613b36565b612e276137ef565b60cb80546001600160a01b0319166001600160a01b0392909216919091179055565b6040516306045a9160e01b815260009030906306045a9190612e7d908b908b908b908b908b908b908b908b90600401614fb2565b602060405180830381865afa158015612e9a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089b9190615014565b6065546001600160a01b03163314612f535760405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a40161073d565b60005b8181101561322757828282818110612f7057612f70614b42565b9050602002810190612f829190615031565b612f93906040810190602001614388565b6001600160a01b03166323b872dd3330868686818110612fb557612fb5614b42565b9050602002810190612fc79190615031565b604080516001600160e01b031960e087901b1681526001600160a01b039485166004820152939092166024840152013560448201526064016020604051808303816000875af115801561301e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906130429190615014565b50600083838381811061305757613057614b42565b90506020028101906130699190615031565b61307a906040810190602001614388565b604051636eb1769f60e11b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081166024830152919091169063dd62ed3e90604401602060405180830381865afa1580156130e8573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061310c9190614b91565b905083838381811061312057613120614b42565b90506020028101906131329190615031565b613143906040810190602001614388565b6001600160a01b031663095ea7b37f00000000000000000000000000000000000000000000000000000000000000008387878781811061318557613185614b42565b90506020028101906131979190615031565b604001356131a59190614b0f565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044016020604051808303816000875af11580156131f0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906132149190615014565b50508061322090614bf6565b9050612f56565b5060405163fce36c7d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063fce36c7d906121f390859085906004016150b7565b600083613284868585613c05565b1495945050505050565b60408051808201909152600080825260208201526132aa613fb6565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa90508080156132dd576132df565bfe5b508061331d5760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b604482015260640161073d565b505092915050565b6040805180820190915260008082526020820152613341613fd4565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa90508080156132dd57508061331d5760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b604482015260640161073d565b6133c1613ff2565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b6040805180820190915260008082526020820152600080806134a96000805160206151e783398151915286614b58565b90505b6134b581613cff565b90935091506000805160206151e78339815191528283098314156134ef576040805180820190915290815260208101919091529392505050565b6000805160206151e78339815191526001820890506134ac565b60408051808201825286815260208082018690528251808401909352868352820184905260009182919061353b614017565b60005b6002811015613700576000613554826006614eec565b905084826002811061356857613568614b42565b6020020151518361357a836000614b0f565b600c811061358a5761358a614b42565b60200201528482600281106135a1576135a1614b42565b602002015160200151838260016135b89190614b0f565b600c81106135c8576135c8614b42565b60200201528382600281106135df576135df614b42565b60200201515151836135f2836002614b0f565b600c811061360257613602614b42565b602002015283826002811061361957613619614b42565b6020020151516001602002015183613632836003614b0f565b600c811061364257613642614b42565b602002015283826002811061365957613659614b42565b60200201516020015160006002811061367457613674614b42565b602002015183613685836004614b0f565b600c811061369557613695614b42565b60200201528382600281106136ac576136ac614b42565b6020020151602001516001600281106136c7576136c7614b42565b6020020151836136d8836005614b0f565b600c81106136e8576136e8614b42565b602002015250806136f881614bf6565b91505061353e565b50613709614036565b60006020826101808560088cfa9151919c9115159b50909950505050505050505050565b606060008061373b8461398c565b61ffff166001600160401b03811115613756576137566140a8565b6040519080825280601f01601f191660200182016040528015613780576020820181803683370190505b5090506000805b825182108015613798575061010081105b15612d9f576001811b9350858416156137df578060f81b8383815181106137c1576137c1614b42565b60200101906001600160f81b031916908160001a9053508160010191505b6137e881614bf6565b9050613787565b6033546001600160a01b031633146120965760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161073d565b606554604080516001600160a01b03928316815291831660208301527fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6097805460ff19168215159081179091556040519081527f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc9060200160405180910390a150565b60008061390584613d81565b9050808360ff166001901b116139835760405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c756500606482015260840161073d565b90505b92915050565b6000805b8215613986576139a1600184614b7a565b90921691806139af816151c4565b915050613990565b60408051808201909152600080825260208201526102008261ffff1610613a135760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b604482015260640161073d565b8161ffff1660011415613a27575081613986565b6040805180820190915260008082526020820181905284906001905b8161ffff168661ffff1610613a9057600161ffff871660ff83161c81161415613a7357613a708484613325565b93505b613a7d8384613325565b92506201fffe600192831b169101613a43565b509195945050505050565b60408051808201909152600080825260208201528151158015613ac057506020820151155b15613ade575050604080518082019091526000808252602082015290565b6040518060400160405280836000015181526020016000805160206151e78339815191528460200151613b119190614b58565b613b29906000805160206151e7833981519152614b7a565b905292915050565b919050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b600054610100900460ff16613bf35760405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b606482015260840161073d565b613bfc82613b36565b610b0581613849565b600060208451613c159190614b58565b15613c9c5760405162461bcd60e51b815260206004820152604b60248201527f4d65726b6c652e70726f63657373496e636c7573696f6e50726f6f664b65636360448201527f616b3a2070726f6f66206c656e6774682073686f756c642062652061206d756c60648201526a3a34b836329037b310199960a91b608482015260a40161073d565b8260205b85518111610cd957613cb3600285614b58565b613cd457816000528086015160205260406000209150600284049350613ced565b8086015160005281602052604060002091506002840493505b613cf8602082614b0f565b9050613ca0565b600080806000805160206151e783398151915260036000805160206151e7833981519152866000805160206151e7833981519152888909090890506000613d75827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f526000805160206151e7833981519152613f0e565b91959194509092505050565b600061010082511115613e0a5760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a40161073d565b8151613e1857506000919050565b60008083600081518110613e2e57613e2e614b42565b0160200151600160f89190911c81901b92505b8451811015613f0557848181518110613e5c57613e5c614b42565b0160200151600160f89190911c1b9150828211613ef15760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a40161073d565b91811791613efe81614bf6565b9050613e41565b50909392505050565b600080613f19614036565b613f21614054565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280156132dd575082613fab5760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c757265000000000000604482015260640161073d565b505195945050505050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060400160405280614005614072565b8152602001614012614072565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b80356001600160601b031981168114613b3157600080fd5b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156140e0576140e06140a8565b60405290565b60405161010081016001600160401b03811182821017156140e0576140e06140a8565b604051601f8201601f191681016001600160401b0381118282101715614131576141316140a8565b604052919050565b60006001600160401b03831115614152576141526140a8565b614165601f8401601f1916602001614109565b905082815283838301111561417957600080fd5b828260208301376000602084830101529392505050565b600082601f8301126141a157600080fd5b6141b083833560208501614139565b9392505050565b6001600160a01b0381168114610fed57600080fd5b600080600080600080600080610100898b0312156141e957600080fd5b88359750602089013596506040890135955061420760608a01614090565b94506080890135935060a08901356001600160401b0381111561422957600080fd5b6142358b828c01614190565b93505060c0890135915060e089013561424d816141b7565b809150509295985092959890939650565b60006040828403121561427057600080fd5b6142786140be565b9050813581526020820135602082015292915050565b600082601f83011261429f57600080fd5b6142a76140be565b8060408401858111156142b957600080fd5b845b818110156142d35780358452602093840193016142bb565b509095945050505050565b6000608082840312156142f057600080fd5b6142f86140be565b9050614304838361428e565b8152614313836040840161428e565b602082015292915050565b600080600080610120858703121561433557600080fd5b84359350614346866020870161425e565b925061435586606087016142de565b91506143648660e0870161425e565b905092959194509250565b60006020828403121561438157600080fd5b5035919050565b60006020828403121561439a57600080fd5b8135613983816141b7565b6020808252825182820181905260009190848201906040850190845b818110156143e65783516001600160a01b0316835292840192918401916001016143c1565b50909695505050505050565b8015158114610fed57600080fd5b60006020828403121561441257600080fd5b8135613983816143f2565b803563ffffffff81168114613b3157600080fd5b60006001600160401b0382111561444a5761444a6140a8565b5060051b60200190565b600082601f83011261446557600080fd5b8135602061447a61447583614431565b614109565b82815260059290921b8401810191818101908684111561449957600080fd5b8286015b848110156144bb576144ae8161441d565b835291830191830161449d565b509695505050505050565b600082601f8301126144d757600080fd5b813560206144e761447583614431565b82815260069290921b8401810191818101908684111561450657600080fd5b8286015b848110156144bb5761451c888261425e565b83529183019160400161450a565b600082601f83011261453b57600080fd5b8135602061454b61447583614431565b82815260059290921b8401810191818101908684111561456a57600080fd5b8286015b848110156144bb5780356001600160401b0381111561458d5760008081fd5b61459b8986838b0101614454565b84525091830191830161456e565b600061018082840312156145bc57600080fd5b6145c46140e6565b905081356001600160401b03808211156145dd57600080fd5b6145e985838601614454565b835260208401359150808211156145ff57600080fd5b61460b858386016144c6565b6020840152604084013591508082111561462457600080fd5b614630858386016144c6565b604084015261464285606086016142de565b60608401526146548560e0860161425e565b608084015261012084013591508082111561466e57600080fd5b61467a85838601614454565b60a084015261014084013591508082111561469457600080fd5b6146a085838601614454565b60c08401526101608401359150808211156146ba57600080fd5b506146c78482850161452a565b60e08301525092915050565b6000806000606084860312156146e857600080fd5b833592506146f86020850161441d565b915060408401356001600160401b0381111561471357600080fd5b61471f868287016145a9565b9150509250925092565b600081518084526020808501945080840160005b838110156147625781516001600160601b03168752958201959082019060010161473d565b509495945050505050565b60408152600083516040808401526147886080840182614729565b90506020850151603f198483030160608501526147a58282614729565b925050508260208301529392505050565b60008083601f8401126147c857600080fd5b5081356001600160401b038111156147df57600080fd5b6020830191508360208285010111156147f757600080fd5b9250929050565b60008060006040848603121561481357600080fd5b83356001600160401b0381111561482957600080fd5b614835868287016147b6565b909790965060209590950135949350505050565b6000806040838503121561485c57600080fd5b8235614867816141b7565b915060208301356001600160401b038082111561488357600080fd5b908401906060828703121561489757600080fd5b6040516060810181811083821117156148b2576148b26140a8565b6040528235828111156148c457600080fd5b6148d088828601614190565b82525060208301356020820152604083013560408201528093505050509250929050565b60006020828403121561490657600080fd5b81356001600160401b0381111561491c57600080fd5b8201601f8101841361492d57600080fd5b61493c84823560208401614139565b949350505050565b60008060006060848603121561495957600080fd5b8335925060208401356146f8816141b7565b60008060006060848603121561498057600080fd5b833561498b816141b7565b9250602084013561499b816141b7565b915060408401356149ab816141b7565b809150509250925092565b600080600080606085870312156149cc57600080fd5b8435935060208501356001600160401b038111156149e957600080fd5b6149f5878288016147b6565b9598909750949560400135949350505050565b600080600080600080600060e0888a031215614a2357600080fd5b873596506020880135955060408801359450614a4160608901614090565b93506080880135925060a08801356001600160401b03811115614a6357600080fd5b614a6f8a828b01614190565b92505060c0880135905092959891949750929550565b60008060208385031215614a9857600080fd5b82356001600160401b0380821115614aaf57600080fd5b818501915085601f830112614ac357600080fd5b813581811115614ad257600080fd5b8660208260051b8501011115614ae757600080fd5b60209290920196919550909350505050565b634e487b7160e01b600052601160045260246000fd5b60008219821115614b2257614b22614af9565b500190565b91825260601b6001600160601b031916602082015260340190565b634e487b7160e01b600052603260045260246000fd5b600082614b7557634e487b7160e01b600052601260045260246000fd5b500690565b600082821015614b8c57614b8c614af9565b500390565b600060208284031215614ba357600080fd5b5051919050565b600060208284031215614bbc57600080fd5b81516001600160c01b038116811461398357600080fd5b600060208284031215614be557600080fd5b815160ff8116811461398357600080fd5b6000600019821415614c0a57614c0a614af9565b5060010190565b6001600160601b0381168114610fed57600080fd5b600060408284031215614c3857600080fd5b614c406140be565b8251614c4b816141b7565b81526020830151614c5b81614c11565b60208201529392505050565b600060208284031215614c7957600080fd5b8151613983816141b7565b600060208284031215614c9657600080fd5b815167ffffffffffffffff198116811461398357600080fd5b600060208284031215614cc157600080fd5b815161398381614c11565b60006001600160601b0383811690831681811015614cec57614cec614af9565b039392505050565b63ffffffff60e01b8360e01b1681526000600482018351602080860160005b83811015614d2f57815185529382019390820190600101614d13565b5092979650505050505050565b6020808252602e908201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160408201526d191e481a5b9a5d1a585b1a5e995960921b606082015260800190565b8183823760009101908152919050565b60208082526052908201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360408201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560608201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608082015260a00190565b6000815180845260005b81811015614e3857602081850181015186830182015201614e1c565b81811115614e4a576000602083870101525b50601f01601f19169290920160200192915050565b60018060a01b0383168152604060208201526000825160606040840152614e8960a0840182614e12565b90506020840151606084015260408401516080840152809150509392505050565b6020815260006141b06020830184614e12565b60006001600160601b0380831681851681830481118215151615614ee357614ee3614af9565b02949350505050565b6000816000190483118215151615614f0657614f06614af9565b500290565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6001600160a01b038516815263ffffffff84166020820152606060408201819052600090614f659083018486614f0b565b9695505050505050565b6001600160a01b038616815263ffffffff85166020820152608060408201819052600090614fa09083018587614f0b565b90508260608301529695505050505050565b60006101008a83528960208401528860408401526001600160601b0319881660608401528660808401528060a0840152614fee81840187614e12565b60c084019590955250506001600160a01b039190911660e0909101529695505050505050565b60006020828403121561502657600080fd5b8151613983816143f2565b60008235609e1983360301811261504757600080fd5b9190910192915050565b8035613b31816141b7565b8183526000602080850194508260005b8581101561476257813561507f816141b7565b6001600160a01b031687528183013561509781614c11565b6001600160601b031687840152604096870196919091019060010161506c565b60208082528181018390526000906040808401600586901b8501820187855b888110156151b657878303603f190184528135368b9003609e190181126150fc57600080fd5b8a0160a0813536839003601e1901811261511557600080fd5b820180356001600160401b0381111561512d57600080fd5b8060061b360384131561513f57600080fd5b828752615151838801828c850161505c565b92505050615160888301615051565b6001600160a01b0316888601528187013587860152606061518281840161441d565b63ffffffff1690860152608061519983820161441d565b63ffffffff169501949094525092850192908501906001016150d6565b509098975050505050505050565b600061ffff808316818114156151dc576151dc614af9565b600101939250505056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd470ea46f246ccfc58f7a93aa09bc6245a6818e97b1a160d186afe78993a3b194a0424c535369676e6174757265436865636b65722e636865636b5369676e617475a264697066735822122017dfd150dae9460b2ec60dad206a1badc342ad50fc4b8453a30d1d04bafbb76364736f6c634300080c0033",
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
