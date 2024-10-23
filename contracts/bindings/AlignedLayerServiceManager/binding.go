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
	ABI: "[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"__avsDirectory\",\"type\":\"address\",\"internalType\":\"contractIAVSDirectory\"},{\"name\":\"__rewardsCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRewardsCoordinator\"},{\"name\":\"__registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"__stakeRegistry\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"receive\",\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"alignedAggregator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"avsDirectory\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"balanceOf\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"batchersBalances\",\"inputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"batchesState\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"responded\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"respondToTaskFeeLimit\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsApkRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBLSApkRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkPublicInput\",\"inputs\":[{\"name\":\"publicInput\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"hash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"pure\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"params\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createAVSRewardsSubmission\",\"inputs\":[{\"name\":\"rewardsSubmissions\",\"type\":\"tuple[]\",\"internalType\":\"structIRewardsCoordinator.RewardsSubmission[]\",\"components\":[{\"name\":\"strategiesAndMultipliers\",\"type\":\"tuple[]\",\"internalType\":\"structIRewardsCoordinator.StrategyAndMultiplier[]\",\"components\":[{\"name\":\"strategy\",\"type\":\"address\",\"internalType\":\"contractIStrategy\"},{\"name\":\"multiplier\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"token\",\"type\":\"address\",\"internalType\":\"contractIERC20\"},{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"startTimestamp\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"duration\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"createNewTask\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"batchDataPointer\",\"type\":\"string\",\"internalType\":\"string\"},{\"name\":\"respondToTaskFeeLimit\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"delegation\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIDelegationManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"depositToBatcher\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"deregisterOperatorFromAVS\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"disableVerifier\",\"inputs\":[{\"name\":\"verifierIdx\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"disabledVerifiers\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"enableVerifier\",\"inputs\":[{\"name\":\"verifierIdx\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"getOperatorRestakedStrategies\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getRestakeableStrategies\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_rewardsInitiator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_alignedAggregator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"initializeAggregator\",\"inputs\":[{\"name\":\"_alignedAggregator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"isVerifierDisabled\",\"inputs\":[{\"name\":\"verifierIdx\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registerOperatorToAVS\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorSignature\",\"type\":\"tuple\",\"internalType\":\"structISignatureUtils.SignatureWithSaltAndExpiry\",\"components\":[{\"name\":\"signature\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"salt\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"expiry\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"registryCoordinator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToTaskV2\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]},{\"name\":\"i\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"rewardsInitiator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setAggregator\",\"inputs\":[{\"name\":\"_alignedAggregator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setDisabledVerifiers\",\"inputs\":[{\"name\":\"bitmap\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setRewardsInitiator\",\"inputs\":[{\"name\":\"newRewardsInitiator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setStaleStakesForbidden\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stakeRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"staleStakesForbidden\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"trySignatureAndApkVerification\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"apk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[{\"name\":\"pairingSuccessful\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"siganatureIsValid\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"updateAVSMetadataURI\",\"inputs\":[{\"name\":\"_metadataURI\",\"type\":\"string\",\"internalType\":\"string\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"verifyBatchInclusion\",\"inputs\":[{\"name\":\"proofCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pubInputCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"provingSystemAuxDataCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"proofGeneratorAddr\",\"type\":\"bytes20\",\"internalType\":\"bytes20\"},{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"merkleProof\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"verificationDataBatchIndex\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"verifyBatchInclusion\",\"inputs\":[{\"name\":\"proofCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pubInputCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"provingSystemAuxDataCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"proofGeneratorAddr\",\"type\":\"bytes20\",\"internalType\":\"bytes20\"},{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"merkleProof\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"verificationDataBatchIndex\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"withdraw\",\"inputs\":[{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"BatchVerified\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"BatcherBalanceUpdated\",\"inputs\":[{\"name\":\"batcher\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newBalance\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewBatchV2\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"batchDataPointer\",\"type\":\"string\",\"indexed\":false,\"internalType\":\"string\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewBatchV3\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"batchDataPointer\",\"type\":\"string\",\"indexed\":false,\"internalType\":\"string\"},{\"name\":\"respondToTaskFeeLimit\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RewardsInitiatorUpdated\",\"inputs\":[{\"name\":\"prevRewardsInitiator\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"newRewardsInitiator\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"StaleStakesForbiddenUpdate\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"VerifierDisabled\",\"inputs\":[{\"name\":\"verifierIdx\",\"type\":\"uint8\",\"indexed\":true,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"VerifierEnabled\",\"inputs\":[{\"name\":\"verifierIdx\",\"type\":\"uint8\",\"indexed\":true,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"error\",\"name\":\"BatchAlreadyResponded\",\"inputs\":[{\"name\":\"batchIdentifierHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"BatchAlreadySubmitted\",\"inputs\":[{\"name\":\"batchIdentifierHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"BatchDoesNotExist\",\"inputs\":[{\"name\":\"batchIdentifierHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"ExceededMaxRespondFee\",\"inputs\":[{\"name\":\"respondToTaskFeeLimit\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"txCost\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"InsufficientFunds\",\"inputs\":[{\"name\":\"batcher\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"required\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"available\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"InvalidAddress\",\"inputs\":[{\"name\":\"param\",\"type\":\"string\",\"internalType\":\"string\"}]},{\"type\":\"error\",\"name\":\"InvalidDepositAmount\",\"inputs\":[{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"InvalidQuorumThreshold\",\"inputs\":[{\"name\":\"signedStake\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"requiredStake\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"SenderIsNotAggregator\",\"inputs\":[{\"name\":\"sender\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"alignedAggregator\",\"type\":\"address\",\"internalType\":\"address\"}]}]",
	Bin: "0x61018060405234801561001157600080fd5b50604051615a1b380380615a1b833981016040819052610030916103fb565b6001600160a01b0380851660805280841660a05280831660c052811660e052818484828461005c610327565b50505050806001600160a01b0316610100816001600160a01b031681525050806001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100b9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100dd919061045a565b6001600160a01b0316610120816001600160a01b031681525050806001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa158015610135573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610159919061045a565b6001600160a01b0316610140816001600160a01b031681525050610120516001600160a01b031663df5cf7236040518163ffffffff1660e01b8152600401602060405180830381865afa1580156101b4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101d8919061045a565b6001600160a01b03908116610160528516905061022c57604051630b0f5aa160e11b815260206004820152600c60248201526b6176734469726563746f727960a01b60448201526064015b60405180910390fd5b6001600160a01b03831661027857604051630b0f5aa160e11b81526020600482015260126024820152713932bbb0b93239a1b7b7b93234b730ba37b960711b6044820152606401610223565b6001600160a01b0382166102cf57604051630b0f5aa160e11b815260206004820152601360248201527f7265676973747279436f6f7264696e61746f72000000000000000000000000006044820152606401610223565b6001600160a01b03811661031657604051630b0f5aa160e11b815260206004820152600d60248201526c7374616b65526567697374727960981b6044820152606401610223565b61031e610327565b5050505061047e565b600054610100900460ff161561038f5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608401610223565b60005460ff90811610156103e1576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146103f857600080fd5b50565b6000806000806080858703121561041157600080fd5b845161041c816103e3565b602086015190945061042d816103e3565b604086015190935061043e816103e3565b606086015190925061044f816103e3565b939692955090935050565b60006020828403121561046c57600080fd5b8151610477816103e3565b9392505050565b60805160a05160c05160e0516101005161012051610140516101605161549061058b600039600081816106da015261190101526000818161041d0152611b1401526000818161045101528181611d010152611ef10152600081816104b801528181611133015281816115c70152818161176e01526119b5015260008181610e6801528181610fb901528181611050015281816128bd01528181612a360152612ad5015260008181610c8f01528181610d1e01528181610d9e0152818161228a01528181612356015281816127f80152612991015260008181612e8c01528181612f48015261302b015260008181610482015281816122de015281816123b2015261243101526154906000f3fe6080604052600436106102345760003560e01c806395c6d6041161012e578063e481af9d116100ab578063fa534dc01161006f578063fa534dc014610794578063fc299dee146107b4578063fce36c7d146107d4578063fd4c3b7c146107f4578063ff647ee81461081457600080fd5b8063e481af9d146106fc578063ea5ca34b14610711578063f2fde38b14610727578063f474b52014610747578063f9120af61461077457600080fd5b8063b753645e116100f2578063b753645e1461065b578063b98d09081461067b578063c0c53b8b14610695578063d66eaabd146106b5578063df5cf723146106c857600080fd5b806395c6d604146105715780639926ee7d14610591578063a364f4da146105b1578063a98fb355146105d1578063b099627e146105f157600080fd5b80634a5bf632116101bc5780636d14a987116101805780636d14a987146104a657806370a08231146104da578063715018a61461051e578063800fb61f146105335780638da5cb5b1461055357600080fd5b80634a5bf632146103a55780634ae07c37146103dd5780635df459461461040b578063683048351461043f5780636b3aa72e1461047357600080fd5b80632e1a7d4d116102035780632e1a7d4d1461030557806333cfb7b7146103255780633bc28c8c14610352578063416c7e5e146103725780634223d5511461039257600080fd5b806306045a911461024a578063137122b51461027f578063171f1d5b146102ae57806318daeeaf146102e557600080fd5b36610245576102433334610834565b005b600080fd5b34801561025657600080fd5b5061026a6102653660046143c0565b6108c9565b60405190151581526020015b60405180910390f35b34801561028b57600080fd5b5061026a61029a366004614461565b60cc54600160ff9092169190911b16151590565b3480156102ba57600080fd5b506102ce6102c936600461453f565b6109c0565b604080519215158352901515602083015201610276565b3480156102f157600080fd5b50610243610300366004614461565b610b4a565b34801561031157600080fd5b50610243610320366004614590565b610b92565b34801561033157600080fd5b506103456103403660046145a9565b610c6a565b60405161027691906145c6565b34801561035e57600080fd5b5061024361036d3660046145a9565b61111d565b34801561037e57600080fd5b5061024361038d366004614615565b611131565b6102436103a03660046145a9565b611268565b3480156103b157600080fd5b5060cb546103c5906001600160a01b031681565b6040516001600160a01b039091168152602001610276565b3480156103e957600080fd5b506103fd6103f836600461490d565b611272565b6040516102769291906149a8565b34801561041757600080fd5b506103c57f000000000000000000000000000000000000000000000000000000000000000081565b34801561044b57600080fd5b506103c57f000000000000000000000000000000000000000000000000000000000000000081565b34801561047f57600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006103c5565b3480156104b257600080fd5b506103c57f000000000000000000000000000000000000000000000000000000000000000081565b3480156104e657600080fd5b506105106104f53660046145a9565b6001600160a01b0316600090815260ca602052604090205490565b604051908152602001610276565b34801561052a57600080fd5b506102436121a6565b34801561053f57600080fd5b5061024361054e3660046145a9565b6121ba565b34801561055f57600080fd5b506033546001600160a01b03166103c5565b34801561057d57600080fd5b5061026a61058c366004614a39565b61225a565b34801561059d57600080fd5b506102436105ac366004614a84565b61227f565b3480156105bd57600080fd5b506102436105cc3660046145a9565b61234b565b3480156105dd57600080fd5b506102436105ec366004614b3b565b612412565b3480156105fd57600080fd5b5061063961060c366004614590565b60c9602052600090815260409020805460019091015463ffffffff821691640100000000900460ff169083565b6040805163ffffffff9094168452911515602084015290820152606001610276565b34801561066757600080fd5b50610243610676366004614590565b612466565b34801561068757600080fd5b5060975461026a9060ff1681565b3480156106a157600080fd5b506102436106b0366004614b8b565b612473565b6102436106c3366004614bd6565b612638565b3480156106d457600080fd5b506103c57f000000000000000000000000000000000000000000000000000000000000000081565b34801561070857600080fd5b506103456127f2565b34801561071d57600080fd5b5061051060cc5481565b34801561073357600080fd5b506102436107423660046145a9565b612b9e565b34801561075357600080fd5b506105106107623660046145a9565b60ca6020526000908152604090205481565b34801561078057600080fd5b5061024361078f3660046145a9565b612c14565b3480156107a057600080fd5b5061026a6107af366004614c28565b612c3e565b3480156107c057600080fd5b506065546103c5906001600160a01b031681565b3480156107e057600080fd5b506102436107ef366004614ca8565b612cb3565b34801561080057600080fd5b5061024361080f366004614461565b613062565b34801561082057600080fd5b5061024361082f366004614d1d565b6130a9565b8060000361085d57604051632097692160e11b8152600481018290526024015b60405180910390fd5b6001600160a01b038216600090815260ca602052604081208054839290610885908490614d93565b90915550506001600160a01b038216600081815260ca602090815260409182902054915191825260008051602061541b833981519152910160405180910390a25050565b6000806001600160a01b0383166108e157508461090d565b85836040516020016108f4929190614da6565b6040516020818303038152906040528051906020012090505b600081815260c9602052604081205463ffffffff1690036109325760009150506109b4565b600081815260c96020526040902054640100000000900460ff1661095a5760009150506109b4565b60408051602081018c90529081018a9052606081018990526001600160601b03198816608082015260009060940160408051601f19818403018152919052805160208201209091506109ae87898389613478565b93505050505b98975050505050505050565b60008060007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000187876000015188602001518860000151600060028110610a0857610a08614dc1565b60200201518951600160200201518a60200151600060028110610a2d57610a2d614dc1565b60200201518b60200151600160028110610a4957610a49614dc1565b602090810291909101518c518d830151604051610aa69a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b6040516020818303038152906040528051906020012060001c610ac99190614dd7565b9050610b3c610ae2610adb8884613490565b8690613521565b610aea6135b6565b610b32610b2385610b1d604080518082018252600080825260209182015281518083019092526001825260029082015290565b90613490565b610b2c8c613676565b90613521565b886201d4c0613705565b909890975095505050505050565b610b5261391f565b60cc8054600160ff841690811b199091169091556040517f5f52704e8e0190647930ccde0e43e14e89902d7d8c49c5f9e2544029f45ec12a90600090a250565b33600090815260ca6020526040902054811115610be35733600081815260ca602052604090819020549051632e2a182f60e11b81526004810192909252602482018390526044820152606401610854565b33600090815260ca602052604081208054839290610c02908490614df9565b909155505033600081815260ca602090815260409182902054915191825260008051602061541b833981519152910160405180910390a2604051339082156108fc029083906000818181858888f19350505050158015610c66573d6000803e3d6000fd5b5050565b6040516309aa152760e11b81526001600160a01b0382811660048301526060916000917f000000000000000000000000000000000000000000000000000000000000000016906313542a4e90602401602060405180830381865afa158015610cd6573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610cfa9190614e0c565b60405163871ef04960e01b8152600481018290529091506000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063871ef04990602401602060405180830381865afa158015610d65573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d899190614e25565b90506001600160c01b0381161580610e2357507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610dfa573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e1e9190614e4e565b60ff16155b15610e435760408051600080825260208201909252905b50949350505050565b6000610e57826001600160c01b0316613979565b90506000805b8251811015610f23577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316633ca5a5f5848381518110610ea757610ea7614dc1565b01602001516040516001600160e01b031960e084901b16815260f89190911c6004820152602401602060405180830381865afa158015610eeb573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f0f9190614e0c565b610f199083614d93565b9150600101610e5d565b506000816001600160401b03811115610f3e57610f3e614298565b604051908082528060200260200182016040528015610f67578160200160208202803683370190505b5090506000805b8451811015611110576000858281518110610f8b57610f8b614dc1565b0160200151604051633ca5a5f560e01b815260f89190911c6004820181905291506000906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633ca5a5f590602401602060405180830381865afa158015611000573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110249190614e0c565b905060005b81811015611105576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa15801561109e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110c29190614e80565b600001518686815181106110d8576110d8614dc1565b6001600160a01b0390921660209283029190910190910152846110fa81614ec3565b955050600101611029565b505050600101610f6e565b5090979650505050505050565b61112561391f565b61112e81613a3b565b50565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561118f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906111b39190614edc565b6001600160a01b0316336001600160a01b03161461125f5760405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a401610854565b61112e81613aa4565b61112e8134610834565b604080518082019091526060808252602082015260008260400151516040518060400160405280600181526020016000815250511480156112ce57508260a0015151604051806040016040528060018152602001600081525051145b80156112f557508260c0015151604051806040016040528060018152602001600081525051145b801561131c57508260e0015151604051806040016040528060018152602001600081525051145b6113865760405162461bcd60e51b8152602060048201526041602482015260008051602061543b83398151915260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a401610854565b825151602084015151146113fe5760405162461bcd60e51b81526020600482015260446024820181905260008051602061543b833981519152908201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b608482015260a401610854565b4363ffffffff168463ffffffff161061146d5760405162461bcd60e51b815260206004820152603c602482015260008051602061543b83398151915260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b000000006064820152608401610854565b60408051808201825260008082526020808301829052835180850185526060808252818301528451808601865260018082529083019390935284518381528086019095529293919082810190803683370190505060208281019190915260408051808201825260018082526000919093015280518281528082019091529081602001602082028036833701905050815260408051808201909152606080825260208201528560200151516001600160401b0381111561152e5761152e614298565b604051908082528060200260200182016040528015611557578160200160208202803683370190505b5081526020860151516001600160401b0381111561157757611577614298565b6040519080825280602002602001820160405280156115a0578160200160208202803683370190505b508160200181905250600061164c60405180604001604052806001815260200160008152507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611623573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116479190614e4e565b613aeb565b905060005b8760200151518110156118dd576116968860200151828151811061167757611677614dc1565b6020026020010151805160009081526020918201519091526040902090565b836020015182815181106116ac576116ac614dc1565b6020908102919091010152801561176c5760208301516116cd600183614df9565b815181106116dd576116dd614dc1565b602002602001015160001c836020015182815181106116fe576116fe614dc1565b602002602001015160001c1161176c576040805162461bcd60e51b815260206004820152602481019190915260008051602061543b83398151915260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152608401610854565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166304ec6351846020015183815181106117b1576117b1614dc1565b60200260200101518b8b6000015185815181106117d0576117d0614dc1565b60200260200101516040518463ffffffff1660e01b815260040161180d9392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa15801561182a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061184e9190614e25565b6001600160c01b03168360000151828151811061186d5761186d614dc1565b6020026020010181815250506118d3610adb6118a7848660000151858151811061189957611899614dc1565b602002602001015116613b7e565b8a6020015184815181106118bd576118bd614dc1565b6020026020010151613ba990919063ffffffff16565b9450600101611651565b50506118e883613c8c565b60975490935060ff166000816118ff576000611981565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c448feb86040518163ffffffff1660e01b8152600401602060405180830381865afa15801561195d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906119819190614e0c565b905060005b604051806040016040528060018152602001600081525051811015612077578215611b12578963ffffffff16827f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663249a0c4260405180604001604052806001815260200160008152508581518110611a0a57611a0a614dc1565b01602001516040516001600160e01b031960e084901b16815260f89190911c6004820152602401602060405180830381865afa158015611a4e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a729190614e0c565b611a7c9190614d93565b11611b125760405162461bcd60e51b8152602060048201526066602482015260008051602061543b83398151915260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c401610854565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166368bccaac60405180604001604052806001815260200160008152508381518110611b6957611b69614dc1565b602001015160f81c60f81b60f81c8c8c60a001518581518110611b8e57611b8e614dc1565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015611bea573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c0e9190614ef9565b6001600160401b031916611c318a60400151838151811061167757611677614dc1565b67ffffffffffffffff191614611ccd5760405162461bcd60e51b8152602060048201526061602482015260008051602061543b83398151915260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c401610854565b611cfd89604001518281518110611ce657611ce6614dc1565b60200260200101518761352190919063ffffffff16565b95507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c8294c5660405180604001604052806001815260200160008152508381518110611d5657611d56614dc1565b602001015160f81c60f81b60f81c8c8c60c001518581518110611d7b57611d7b614dc1565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015611dd7573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611dfb9190614f24565b85602001518281518110611e1157611e11614dc1565b6001600160601b03909216602092830291909101820152850151805182908110611e3d57611e3d614dc1565b602002602001015185600001518281518110611e5b57611e5b614dc1565b60200260200101906001600160601b031690816001600160601b0316815250506000805b8a602001515181101561206d57611eea86600001518281518110611ea557611ea5614dc1565b602002602001015160405180604001604052806001815260200160008152508581518110611ed557611ed5614dc1565b016020015160f81c60ff161c60019081161490565b15612065577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f2be94ae60405180604001604052806001815260200160008152508581518110611f4657611f46614dc1565b602001015160f81c60f81b60f81c8e89602001518581518110611f6b57611f6b614dc1565b60200260200101518f60e001518881518110611f8957611f89614dc1565b60200260200101518781518110611fa257611fa2614dc1565b60209081029190910101516040516001600160e01b031960e087901b16815260ff909416600485015263ffffffff92831660248501526044840191909152166064820152608401602060405180830381865afa158015612006573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061202a9190614f24565b875180518590811061203e5761203e614dc1565b602002602001018181516120529190614f41565b6001600160601b03169052506001909101905b600101611e7f565b5050600101611986565b5050506000806120918a868a606001518b608001516109c0565b91509150816121025760405162461bcd60e51b8152602060048201526043602482015260008051602061543b83398151915260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a401610854565b806121635760405162461bcd60e51b8152602060048201526039602482015260008051602061543b83398151915260448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608401610854565b5050600087826020015160405160200161217e929190614f60565b60408051808303601f1901815291905280516020909101209299929850919650505050505050565b6121ae61391f565b6121b86000613d27565b565b600054600290610100900460ff161580156121dc575060005460ff8083169116105b6121f85760405162461bcd60e51b815260040161085490614fa8565b6000805461ffff191660ff83161761010017905561221582612c14565b6000805461ff001916905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b600081848460405161226d929190614ff6565b60405180910390201490509392505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146122c75760405162461bcd60e51b815260040161085490615006565b604051639926ee7d60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639926ee7d9061231590859085906004016150c4565b600060405180830381600087803b15801561232f57600080fd5b505af1158015612343573d6000803e3d6000fd5b505050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146123935760405162461bcd60e51b815260040161085490615006565b6040516351b27a6d60e11b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a364f4da906024015b600060405180830381600087803b1580156123f757600080fd5b505af115801561240b573d6000803e3d6000fd5b5050505050565b61241a61391f565b60405163a98fb35560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a98fb355906123dd90849060040161510f565b61246e61391f565b60cc55565b600054610100900460ff16158080156124935750600054600160ff909116105b806124ad5750303b1580156124ad575060005460ff166001145b6124c95760405162461bcd60e51b815260040161085490614fa8565b6000805460ff1916600117905580156124ec576000805461ff0019166101001790555b6001600160a01b03841661253257604051630b0f5aa160e11b815260206004820152600c60248201526b34b734ba34b0b627bbb732b960a11b6044820152606401610854565b6001600160a01b03831661257c57604051630b0f5aa160e11b815260206004820152601060248201526f3932bbb0b93239a4b734ba34b0ba37b960811b6044820152606401610854565b6001600160a01b0382166125c757604051630b0f5aa160e11b815260206004820152601160248201527030b634b3b732b220b3b3b932b3b0ba37b960791b6044820152606401610854565b6125d18484613d79565b60cb80546001600160a01b0319166001600160a01b0384161790558015612632576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b6000843360405160200161264d929190614da6565b60408051601f198184030181529181528151602092830120600081815260c990935291205490915063ffffffff161561269c57604051630c40bc4360e21b815260048101829052602401610854565b34156126f95733600090815260ca6020526040812080543492906126c1908490614d93565b909155505033600081815260ca602090815260409182902054915191825260008051602061541b833981519152910160405180910390a25b33600090815260ca602052604090205482111561274a5733600081815260ca602052604090819020549051632e2a182f60e11b81526004810192909252602482018490526044820152606401610854565b604080516060810182526000602080830182815263ffffffff43818116865285870189815288865260c99094529386902085518154935115156401000000000264ffffffffff1990941692169190911791909117815590516001909101559151909187917f8801fc966deb2c8f563a103c35c9e80740585c292cd97518587e6e7927e6af55916127e2913391908a908a908a90615122565b60405180910390a2505050505050565b606060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612854573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906128789190614e4e565b60ff1690508060000361289957505060408051600081526020810190915290565b6000805b8281101561294457604051633ca5a5f560e01b815260ff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa15801561290c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906129309190614e0c565b61293a9083614d93565b915060010161289d565b506000816001600160401b0381111561295f5761295f614298565b604051908082528060200260200182016040528015612988578160200160208202803683370190505b5090506000805b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156129ed573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612a119190614e4e565b60ff16811015612b9457604051633ca5a5f560e01b815260ff821660048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa158015612a85573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612aa99190614e0c565b905060005b81811015612b8a576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa158015612b23573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612b479190614e80565b60000151858581518110612b5d57612b5d614dc1565b6001600160a01b039092166020928302919091019091015283612b7f81614ec3565b945050600101612aae565b505060010161298f565b5090949350505050565b612ba661391f565b6001600160a01b038116612c0b5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610854565b61112e81613d27565b612c1c61391f565b60cb80546001600160a01b0319166001600160a01b0392909216919091179055565b6040516306045a9160e01b815260009030906306045a9190612c72908b908b908b908b908b908b908b908b90600401615179565b602060405180830381865afa158015612c8f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109b491906151db565b6065546001600160a01b03163314612d485760405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a401610854565b60005b8181101561301357828282818110612d6557612d65614dc1565b9050602002810190612d7791906151f8565b612d889060408101906020016145a9565b6001600160a01b03166323b872dd3330868686818110612daa57612daa614dc1565b9050602002810190612dbc91906151f8565b604080516001600160e01b031960e087901b1681526001600160a01b039485166004820152939092166024840152013560448201526064016020604051808303816000875af1158015612e13573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612e3791906151db565b506000838383818110612e4c57612e4c614dc1565b9050602002810190612e5e91906151f8565b612e6f9060408101906020016145a9565b604051636eb1769f60e11b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081166024830152919091169063dd62ed3e90604401602060405180830381865afa158015612edd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612f019190614e0c565b9050838383818110612f1557612f15614dc1565b9050602002810190612f2791906151f8565b612f389060408101906020016145a9565b6001600160a01b031663095ea7b37f000000000000000000000000000000000000000000000000000000000000000083878787818110612f7a57612f7a614dc1565b9050602002810190612f8c91906151f8565b60400135612f9a9190614d93565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044016020604051808303816000875af1158015612fe5573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061300991906151db565b5050600101612d4b565b5060405163fce36c7d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063fce36c7d90612315908590859060040161527f565b61306a61391f565b60cc8054600160ff841690811b9091179091556040517fec54a85c01b5fc7fb41be0f33eabc56f2981110da8317b9817bc7c718f6d7bfe90600090a250565b60cb546001600160a01b031633146130e95760cb54604051632cbe419560e01b81523360048201526001600160a01b039091166024820152604401610854565b60005a905060048210156130fd5750612632565b60008585604051602001613112929190614da6565b60408051601f198184030181529181528151602092830120600081815260c990935290822080549193509163ffffffff9091169003613167576040516311cb69a760e11b815260048101839052602401610854565b8054640100000000900460ff161561319557604051634e78d7f960e11b815260048101839052602401610854565b805464ff00000000191664010000000017815560018101546001600160a01b038716600090815260ca602052604090205410156132185760018101546001600160a01b038716600081815260ca602052604090819020549051632e2a182f60e11b8152600481019290925260248201929092526044810191909152606401610854565b805460009061322f90849063ffffffff1688611272565b509050604360ff16816020015160008151811061324e5761324e614dc1565b60200260200101516132609190615399565b6001600160601b03166064826000015160008151811061328257613282614dc1565b60200260200101516001600160601b031661329d91906153c2565b101561333057606481600001516000815181106132bc576132bc614dc1565b60200260200101516001600160601b03166132d791906153c2565b604360ff1682602001516000815181106132f3576132f3614dc1565b60200260200101516133059190615399565b60405163530f5c4560e11b815260048101929092526001600160601b03166024820152604401610854565b6040516001600160a01b038816815288907f8511746b73275e06971968773119b9601fc501d7bdf3824d8754042d148940e29060200160405180910390a260003a5a61337c9087614df9565b6133899062011170614d93565b61339391906153c2565b905082600101548111156133ca57600183015460405163437e283f60e11b8152600481019190915260248101829052604401610854565b6001600160a01b038816600090815260ca6020526040812080548392906133f2908490614df9565b90915550506001600160a01b038816600081815260ca602090815260409182902054915191825260008051602061541b833981519152910160405180910390a260cb546040516001600160a01b039091169082156108fc029083906000818181858888f1935050505015801561346c573d6000803e3d6000fd5b50505050505050505050565b600083613486868585613df6565b1495945050505050565b60408051808201909152600080825260208201526134ac6141a6565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa905080806134db57fe5b50806135195760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401610854565b505092915050565b604080518082019091526000808252602082015261353d6141c4565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa9050808061357857fe5b50806135195760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606401610854565b6135be6141e2565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b6040805180820190915260008082526020820152600080806136a66000805160206153fb83398151915286614dd7565b90505b6136b281613ef3565b90935091506000805160206153fb83398151915282830983036136eb576040805180820190915290815260208101919091529392505050565b6000805160206153fb8339815191526001820890506136a9565b604080518082018252868152602080820186905282518084019093528683528201849052600091829190613737614207565b60005b60028110156138f25760006137508260066153c2565b905084826002811061376457613764614dc1565b60200201515183613776836000614d93565b600c811061378657613786614dc1565b602002015284826002811061379d5761379d614dc1565b602002015160200151838260016137b49190614d93565b600c81106137c4576137c4614dc1565b60200201528382600281106137db576137db614dc1565b60200201515151836137ee836002614d93565b600c81106137fe576137fe614dc1565b602002015283826002811061381557613815614dc1565b602002015151600160200201518361382e836003614d93565b600c811061383e5761383e614dc1565b602002015283826002811061385557613855614dc1565b60200201516020015160006002811061387057613870614dc1565b602002015183613881836004614d93565b600c811061389157613891614dc1565b60200201528382600281106138a8576138a8614dc1565b6020020151602001516001600281106138c3576138c3614dc1565b6020020151836138d4836005614d93565b600c81106138e4576138e4614dc1565b60200201525060010161373a565b506138fb614226565b60006020826101808560088cfa9151919c9115159b50909950505050505050505050565b6033546001600160a01b031633146121b85760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610854565b606060008061398784613b7e565b61ffff166001600160401b038111156139a2576139a2614298565b6040519080825280601f01601f1916602001820160405280156139cc576020820181803683370190505b5090506000805b8251821080156139e4575061010081105b15612b94576001811b935085841615613a2b578060f81b838381518110613a0d57613a0d614dc1565b60200101906001600160f81b031916908160001a9053508160010191505b613a3481614ec3565b90506139d3565b606554604080516001600160a01b03928316815291831660208301527fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6097805460ff19168215159081179091556040519081527f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc9060200160405180910390a150565b600080613af784613f75565b9050808360ff166001901b11613b755760405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c7565006064820152608401610854565b90505b92915050565b6000805b8215613b7857613b93600184614df9565b9092169180613ba1816153d9565b915050613b82565b60408051808201909152600080825260208201526102008261ffff1610613c055760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606401610854565b8161ffff16600103613c18575081613b78565b6040805180820190915260008082526020820181905284906001905b8161ffff168661ffff1610613c8157600161ffff871660ff83161c81169003613c6457613c618484613521565b93505b613c6e8384613521565b92506201fffe600192831b169101613c34565b509195945050505050565b60408051808201909152600080825260208201528151158015613cb157506020820151155b15613ccf575050604080518082019091526000808252602082015290565b6040518060400160405280836000015181526020016000805160206153fb8339815191528460200151613d029190614dd7565b613d1a906000805160206153fb833981519152614df9565b905292915050565b919050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b600054610100900460ff16613de45760405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608401610854565b613ded82613d27565b610c6681613a3b565b600060208451613e069190614dd7565b15613e8d5760405162461bcd60e51b815260206004820152604b60248201527f4d65726b6c652e70726f63657373496e636c7573696f6e50726f6f664b65636360448201527f616b3a2070726f6f66206c656e6774682073686f756c642062652061206d756c60648201526a3a34b836329037b310199960a91b608482015260a401610854565b8260205b85518111610e3a57613ea4600285614dd7565b600003613ec857816000528086015160205260406000209150600284049350613ee1565b8086015160005281602052604060002091506002840493505b613eec602082614d93565b9050613e91565b600080806000805160206153fb83398151915260036000805160206153fb833981519152866000805160206153fb833981519152888909090890506000613f69827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f526000805160206153fb8339815191526140fd565b91959194509092505050565b600061010082511115613ffe5760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a401610854565b815160000361400f57506000919050565b6000808360008151811061402557614025614dc1565b0160200151600160f89190911c81901b92505b84518110156140f45784818151811061405357614053614dc1565b0160200151600160f89190911c1b91508282116140e85760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a401610854565b91811791600101614038565b50909392505050565b600080614108614226565b614110614244565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa9250828061414d57fe5b508261419b5760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606401610854565b505195945050505050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b60405180604001604052806141f5614262565b8152602001614202614262565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b80356001600160601b031981168114613d2257600080fd5b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156142d0576142d0614298565b60405290565b60405161010081016001600160401b03811182821017156142d0576142d0614298565b604051601f8201601f191681016001600160401b038111828210171561432157614321614298565b604052919050565b6000806001600160401b0384111561434357614343614298565b50601f8301601f1916602001614358816142f9565b91505082815283838301111561436d57600080fd5b828260208301376000602084830101529392505050565b600082601f83011261439557600080fd5b6143a483833560208501614329565b9392505050565b6001600160a01b038116811461112e57600080fd5b600080600080600080600080610100898b0312156143dd57600080fd5b8835975060208901359650604089013595506143fb60608a01614280565b94506080890135935060a08901356001600160401b0381111561441d57600080fd5b6144298b828c01614384565b93505060c0890135915060e0890135614441816143ab565b809150509295985092959890939650565b60ff8116811461112e57600080fd5b60006020828403121561447357600080fd5b8135613b7581614452565b60006040828403121561449057600080fd5b6144986142ae565b823581526020928301359281019290925250919050565b600082601f8301126144c057600080fd5b6144c86142ae565b8060408401858111156144da57600080fd5b845b818110156144f45780358452602093840193016144dc565b509095945050505050565b60006080828403121561451157600080fd5b6145196142ae565b905061452583836144af565b815261453483604084016144af565b602082015292915050565b600080600080610120858703121561455657600080fd5b84359350614567866020870161447e565b925061457686606087016144ff565b91506145858660e0870161447e565b905092959194509250565b6000602082840312156145a257600080fd5b5035919050565b6000602082840312156145bb57600080fd5b8135613b75816143ab565b602080825282518282018190526000918401906040840190835b818110156144f45783516001600160a01b03168352602093840193909201916001016145e0565b801515811461112e57600080fd5b60006020828403121561462757600080fd5b8135613b7581614607565b803563ffffffff81168114613d2257600080fd5b60006001600160401b0382111561465f5761465f614298565b5060051b60200190565b600082601f83011261467a57600080fd5b813561468d61468882614646565b6142f9565b8082825260208201915060208360051b8601019250858311156146af57600080fd5b602085015b838110156146d3576146c581614632565b8352602092830192016146b4565b5095945050505050565b600082601f8301126146ee57600080fd5b81356146fc61468882614646565b8082825260208201915060208360061b86010192508583111561471e57600080fd5b602085015b838110156146d357614735878261447e565b8352602090920191604001614723565b600082601f83011261475657600080fd5b813561476461468882614646565b8082825260208201915060208360051b86010192508583111561478657600080fd5b602085015b838110156146d35780356001600160401b038111156147a957600080fd5b6147b8886020838a0101614669565b8452506020928301920161478b565b600061018082840312156147da57600080fd5b6147e26142d6565b905081356001600160401b038111156147fa57600080fd5b61480684828501614669565b82525060208201356001600160401b0381111561482257600080fd5b61482e848285016146dd565b60208301525060408201356001600160401b0381111561484d57600080fd5b614859848285016146dd565b60408301525061486c83606084016144ff565b606082015261487e8360e0840161447e565b60808201526101208201356001600160401b0381111561489d57600080fd5b6148a984828501614669565b60a0830152506101408201356001600160401b038111156148c957600080fd5b6148d584828501614669565b60c0830152506101608201356001600160401b038111156148f557600080fd5b61490184828501614745565b60e08301525092915050565b60008060006060848603121561492257600080fd5b8335925061493260208501614632565b915060408401356001600160401b0381111561494d57600080fd5b614959868287016147c7565b9150509250925092565b600081518084526020840193506020830160005b8281101561499e5781516001600160601b0316865260209586019590910190600101614977565b5093949350505050565b60408152600083516040808401526149c36080840182614963565b90506020850151603f198483030160608501526149e08282614963565b925050508260208301529392505050565b60008083601f840112614a0357600080fd5b5081356001600160401b03811115614a1a57600080fd5b602083019150836020828501011115614a3257600080fd5b9250929050565b600080600060408486031215614a4e57600080fd5b83356001600160401b03811115614a6457600080fd5b614a70868287016149f1565b909790965060209590950135949350505050565b60008060408385031215614a9757600080fd5b8235614aa2816143ab565b915060208301356001600160401b03811115614abd57600080fd5b830160608186031215614acf57600080fd5b604051606081016001600160401b0381118282101715614af157614af1614298565b60405281356001600160401b03811115614b0a57600080fd5b614b1687828501614384565b8252506020828101359082015260409182013591810191909152919491935090915050565b600060208284031215614b4d57600080fd5b81356001600160401b03811115614b6357600080fd5b8201601f81018413614b7457600080fd5b614b8384823560208401614329565b949350505050565b600080600060608486031215614ba057600080fd5b8335614bab816143ab565b92506020840135614bbb816143ab565b91506040840135614bcb816143ab565b809150509250925092565b60008060008060608587031215614bec57600080fd5b8435935060208501356001600160401b03811115614c0957600080fd5b614c15878288016149f1565b9598909750949560400135949350505050565b600080600080600080600060e0888a031215614c4357600080fd5b873596506020880135955060408801359450614c6160608901614280565b93506080880135925060a08801356001600160401b03811115614c8357600080fd5b614c8f8a828b01614384565b979a969950949793969295929450505060c09091013590565b60008060208385031215614cbb57600080fd5b82356001600160401b03811115614cd157600080fd5b8301601f81018513614ce257600080fd5b80356001600160401b03811115614cf857600080fd5b8560208260051b8401011115614d0d57600080fd5b6020919091019590945092505050565b60008060008060808587031215614d3357600080fd5b843593506020850135614d45816143ab565b925060408501356001600160401b03811115614d6057600080fd5b614d6c878288016147c7565b949793965093946060013593505050565b634e487b7160e01b600052601160045260246000fd5b80820180821115613b7857613b78614d7d565b91825260601b6001600160601b031916602082015260340190565b634e487b7160e01b600052603260045260246000fd5b600082614df457634e487b7160e01b600052601260045260246000fd5b500690565b81810381811115613b7857613b78614d7d565b600060208284031215614e1e57600080fd5b5051919050565b600060208284031215614e3757600080fd5b81516001600160c01b0381168114613b7557600080fd5b600060208284031215614e6057600080fd5b8151613b7581614452565b6001600160601b038116811461112e57600080fd5b60006040828403128015614e9357600080fd5b50614e9c6142ae565b8251614ea7816143ab565b81526020830151614eb781614e6b565b60208201529392505050565b600060018201614ed557614ed5614d7d565b5060010190565b600060208284031215614eee57600080fd5b8151613b75816143ab565b600060208284031215614f0b57600080fd5b815167ffffffffffffffff1981168114613b7557600080fd5b600060208284031215614f3657600080fd5b8151613b7581614e6b565b6001600160601b038281168282160390811115613b7857613b78614d7d565b63ffffffff60e01b8360e01b16815260006004820183516020850160005b82811015614f9c578151845260209384019390910190600101614f7e565b50919695505050505050565b6020808252602e908201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160408201526d191e481a5b9a5d1a585b1a5e995960921b606082015260800190565b8183823760009101908152919050565b60208082526052908201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360408201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560608201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608082015260a00190565b6000815180845260005b818110156150a457602081850181015186830182015201615088565b506000602082860101526020601f19601f83011685010191505092915050565b60018060a01b03831681526040602082015260008251606060408401526150ee60a084018261507e565b90506020840151606084015260408401516080840152809150509392505050565b6020815260006143a4602083018461507e565b6001600160a01b038616815263ffffffff851660208201526080604082018190528101839052828460a0830137600060a08483010152600060a0601f19601f86011683010190508260608301529695505050505050565b8881528760208201528660408201526001600160601b03198616606082015284608082015261010060a082015260006151b661010083018661507e565b60c0830194909452506001600160a01b039190911660e0909101529695505050505050565b6000602082840312156151ed57600080fd5b8151613b7581614607565b60008235609e1983360301811261520e57600080fd5b9190910192915050565b8035613d22816143ab565b81835260208301925060008160005b8481101561499e578135615245816143ab565b6001600160a01b03168652602082013561525e81614e6b565b6001600160601b031660208701526040958601959190910190600101615232565b6020808252810182905260006040600584901b830181019083018583609e1936839003015b8782101561538c57868503603f1901845282358181126152c357600080fd5b8901803536829003601e190181126152da57600080fd5b81016020810190356001600160401b038111156152f657600080fd5b8060061b360382131561530857600080fd5b60a0885261531a60a089018284615223565b91505061532960208301615218565b6001600160a01b031660208801526040828101359088015261534d60608301614632565b63ffffffff16606088015261536460808301614632565b63ffffffff8116608089015291509550506020938401939290920191600191909101906152a4565b5092979650505050505050565b6001600160601b0381811683821602908116908181146153bb576153bb614d7d565b5092915050565b8082028115828204841417613b7857613b78614d7d565b600061ffff821661ffff81036153f1576153f1614d7d565b6001019291505056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd470ea46f246ccfc58f7a93aa09bc6245a6818e97b1a160d186afe78993a3b194a0424c535369676e6174757265436865636b65722e636865636b5369676e617475a264697066735822122013365dc128488a5a29c029985adbb081cd485c46d476a793a0eab49e5c2f67fd64736f6c634300081b0033",
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

// BatchersBalances is a free data retrieval call binding the contract method 0xf474b520.
//
// Solidity: function batchersBalances(address ) view returns(uint256)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) BatchersBalances(opts *bind.CallOpts, arg0 common.Address) (*big.Int, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "batchersBalances", arg0)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// BatchersBalances is a free data retrieval call binding the contract method 0xf474b520.
//
// Solidity: function batchersBalances(address ) view returns(uint256)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) BatchersBalances(arg0 common.Address) (*big.Int, error) {
	return _ContractAlignedLayerServiceManager.Contract.BatchersBalances(&_ContractAlignedLayerServiceManager.CallOpts, arg0)
}

// BatchersBalances is a free data retrieval call binding the contract method 0xf474b520.
//
// Solidity: function batchersBalances(address ) view returns(uint256)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) BatchersBalances(arg0 common.Address) (*big.Int, error) {
	return _ContractAlignedLayerServiceManager.Contract.BatchersBalances(&_ContractAlignedLayerServiceManager.CallOpts, arg0)
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

// DisabledVerifiers is a free data retrieval call binding the contract method 0xea5ca34b.
//
// Solidity: function disabledVerifiers() view returns(uint256)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) DisabledVerifiers(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "disabledVerifiers")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// DisabledVerifiers is a free data retrieval call binding the contract method 0xea5ca34b.
//
// Solidity: function disabledVerifiers() view returns(uint256)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) DisabledVerifiers() (*big.Int, error) {
	return _ContractAlignedLayerServiceManager.Contract.DisabledVerifiers(&_ContractAlignedLayerServiceManager.CallOpts)
}

// DisabledVerifiers is a free data retrieval call binding the contract method 0xea5ca34b.
//
// Solidity: function disabledVerifiers() view returns(uint256)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) DisabledVerifiers() (*big.Int, error) {
	return _ContractAlignedLayerServiceManager.Contract.DisabledVerifiers(&_ContractAlignedLayerServiceManager.CallOpts)
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

// IsVerifierDisabled is a free data retrieval call binding the contract method 0x137122b5.
//
// Solidity: function isVerifierDisabled(uint8 verifierIdx) view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) IsVerifierDisabled(opts *bind.CallOpts, verifierIdx uint8) (bool, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "isVerifierDisabled", verifierIdx)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// IsVerifierDisabled is a free data retrieval call binding the contract method 0x137122b5.
//
// Solidity: function isVerifierDisabled(uint8 verifierIdx) view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) IsVerifierDisabled(verifierIdx uint8) (bool, error) {
	return _ContractAlignedLayerServiceManager.Contract.IsVerifierDisabled(&_ContractAlignedLayerServiceManager.CallOpts, verifierIdx)
}

// IsVerifierDisabled is a free data retrieval call binding the contract method 0x137122b5.
//
// Solidity: function isVerifierDisabled(uint8 verifierIdx) view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) IsVerifierDisabled(verifierIdx uint8) (bool, error) {
	return _ContractAlignedLayerServiceManager.Contract.IsVerifierDisabled(&_ContractAlignedLayerServiceManager.CallOpts, verifierIdx)
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

// DisableVerifier is a paid mutator transaction binding the contract method 0xfd4c3b7c.
//
// Solidity: function disableVerifier(uint8 verifierIdx) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) DisableVerifier(opts *bind.TransactOpts, verifierIdx uint8) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "disableVerifier", verifierIdx)
}

// DisableVerifier is a paid mutator transaction binding the contract method 0xfd4c3b7c.
//
// Solidity: function disableVerifier(uint8 verifierIdx) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) DisableVerifier(verifierIdx uint8) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.DisableVerifier(&_ContractAlignedLayerServiceManager.TransactOpts, verifierIdx)
}

// DisableVerifier is a paid mutator transaction binding the contract method 0xfd4c3b7c.
//
// Solidity: function disableVerifier(uint8 verifierIdx) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) DisableVerifier(verifierIdx uint8) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.DisableVerifier(&_ContractAlignedLayerServiceManager.TransactOpts, verifierIdx)
}

// EnableVerifier is a paid mutator transaction binding the contract method 0x18daeeaf.
//
// Solidity: function enableVerifier(uint8 verifierIdx) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) EnableVerifier(opts *bind.TransactOpts, verifierIdx uint8) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "enableVerifier", verifierIdx)
}

// EnableVerifier is a paid mutator transaction binding the contract method 0x18daeeaf.
//
// Solidity: function enableVerifier(uint8 verifierIdx) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) EnableVerifier(verifierIdx uint8) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.EnableVerifier(&_ContractAlignedLayerServiceManager.TransactOpts, verifierIdx)
}

// EnableVerifier is a paid mutator transaction binding the contract method 0x18daeeaf.
//
// Solidity: function enableVerifier(uint8 verifierIdx) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) EnableVerifier(verifierIdx uint8) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.EnableVerifier(&_ContractAlignedLayerServiceManager.TransactOpts, verifierIdx)
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

// RespondToTaskV2 is a paid mutator transaction binding the contract method 0xff647ee8.
//
// Solidity: function respondToTaskV2(bytes32 batchMerkleRoot, address senderAddress, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature, uint256 i) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) RespondToTaskV2(opts *bind.TransactOpts, batchMerkleRoot [32]byte, senderAddress common.Address, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature, i *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "respondToTaskV2", batchMerkleRoot, senderAddress, nonSignerStakesAndSignature, i)
}

// RespondToTaskV2 is a paid mutator transaction binding the contract method 0xff647ee8.
//
// Solidity: function respondToTaskV2(bytes32 batchMerkleRoot, address senderAddress, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature, uint256 i) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) RespondToTaskV2(batchMerkleRoot [32]byte, senderAddress common.Address, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature, i *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.RespondToTaskV2(&_ContractAlignedLayerServiceManager.TransactOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature, i)
}

// RespondToTaskV2 is a paid mutator transaction binding the contract method 0xff647ee8.
//
// Solidity: function respondToTaskV2(bytes32 batchMerkleRoot, address senderAddress, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature, uint256 i) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) RespondToTaskV2(batchMerkleRoot [32]byte, senderAddress common.Address, nonSignerStakesAndSignature IBLSSignatureCheckerNonSignerStakesAndSignature, i *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.RespondToTaskV2(&_ContractAlignedLayerServiceManager.TransactOpts, batchMerkleRoot, senderAddress, nonSignerStakesAndSignature, i)
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

// SetDisabledVerifiers is a paid mutator transaction binding the contract method 0xb753645e.
//
// Solidity: function setDisabledVerifiers(uint256 bitmap) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) SetDisabledVerifiers(opts *bind.TransactOpts, bitmap *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "setDisabledVerifiers", bitmap)
}

// SetDisabledVerifiers is a paid mutator transaction binding the contract method 0xb753645e.
//
// Solidity: function setDisabledVerifiers(uint256 bitmap) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) SetDisabledVerifiers(bitmap *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.SetDisabledVerifiers(&_ContractAlignedLayerServiceManager.TransactOpts, bitmap)
}

// SetDisabledVerifiers is a paid mutator transaction binding the contract method 0xb753645e.
//
// Solidity: function setDisabledVerifiers(uint256 bitmap) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) SetDisabledVerifiers(bitmap *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.SetDisabledVerifiers(&_ContractAlignedLayerServiceManager.TransactOpts, bitmap)
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

// ContractAlignedLayerServiceManagerVerifierDisabledIterator is returned from FilterVerifierDisabled and is used to iterate over the raw logs and unpacked data for VerifierDisabled events raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerVerifierDisabledIterator struct {
	Event *ContractAlignedLayerServiceManagerVerifierDisabled // Event containing the contract specifics and raw log

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
func (it *ContractAlignedLayerServiceManagerVerifierDisabledIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractAlignedLayerServiceManagerVerifierDisabled)
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
		it.Event = new(ContractAlignedLayerServiceManagerVerifierDisabled)
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
func (it *ContractAlignedLayerServiceManagerVerifierDisabledIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractAlignedLayerServiceManagerVerifierDisabledIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractAlignedLayerServiceManagerVerifierDisabled represents a VerifierDisabled event raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerVerifierDisabled struct {
	VerifierIdx uint8
	Raw         types.Log // Blockchain specific contextual infos
}

// FilterVerifierDisabled is a free log retrieval operation binding the contract event 0xec54a85c01b5fc7fb41be0f33eabc56f2981110da8317b9817bc7c718f6d7bfe.
//
// Solidity: event VerifierDisabled(uint8 indexed verifierIdx)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) FilterVerifierDisabled(opts *bind.FilterOpts, verifierIdx []uint8) (*ContractAlignedLayerServiceManagerVerifierDisabledIterator, error) {

	var verifierIdxRule []interface{}
	for _, verifierIdxItem := range verifierIdx {
		verifierIdxRule = append(verifierIdxRule, verifierIdxItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.FilterLogs(opts, "VerifierDisabled", verifierIdxRule)
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerVerifierDisabledIterator{contract: _ContractAlignedLayerServiceManager.contract, event: "VerifierDisabled", logs: logs, sub: sub}, nil
}

// WatchVerifierDisabled is a free log subscription operation binding the contract event 0xec54a85c01b5fc7fb41be0f33eabc56f2981110da8317b9817bc7c718f6d7bfe.
//
// Solidity: event VerifierDisabled(uint8 indexed verifierIdx)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) WatchVerifierDisabled(opts *bind.WatchOpts, sink chan<- *ContractAlignedLayerServiceManagerVerifierDisabled, verifierIdx []uint8) (event.Subscription, error) {

	var verifierIdxRule []interface{}
	for _, verifierIdxItem := range verifierIdx {
		verifierIdxRule = append(verifierIdxRule, verifierIdxItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.WatchLogs(opts, "VerifierDisabled", verifierIdxRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractAlignedLayerServiceManagerVerifierDisabled)
				if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "VerifierDisabled", log); err != nil {
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

// ParseVerifierDisabled is a log parse operation binding the contract event 0xec54a85c01b5fc7fb41be0f33eabc56f2981110da8317b9817bc7c718f6d7bfe.
//
// Solidity: event VerifierDisabled(uint8 indexed verifierIdx)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) ParseVerifierDisabled(log types.Log) (*ContractAlignedLayerServiceManagerVerifierDisabled, error) {
	event := new(ContractAlignedLayerServiceManagerVerifierDisabled)
	if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "VerifierDisabled", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractAlignedLayerServiceManagerVerifierEnabledIterator is returned from FilterVerifierEnabled and is used to iterate over the raw logs and unpacked data for VerifierEnabled events raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerVerifierEnabledIterator struct {
	Event *ContractAlignedLayerServiceManagerVerifierEnabled // Event containing the contract specifics and raw log

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
func (it *ContractAlignedLayerServiceManagerVerifierEnabledIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractAlignedLayerServiceManagerVerifierEnabled)
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
		it.Event = new(ContractAlignedLayerServiceManagerVerifierEnabled)
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
func (it *ContractAlignedLayerServiceManagerVerifierEnabledIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractAlignedLayerServiceManagerVerifierEnabledIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractAlignedLayerServiceManagerVerifierEnabled represents a VerifierEnabled event raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerVerifierEnabled struct {
	VerifierIdx uint8
	Raw         types.Log // Blockchain specific contextual infos
}

// FilterVerifierEnabled is a free log retrieval operation binding the contract event 0x5f52704e8e0190647930ccde0e43e14e89902d7d8c49c5f9e2544029f45ec12a.
//
// Solidity: event VerifierEnabled(uint8 indexed verifierIdx)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) FilterVerifierEnabled(opts *bind.FilterOpts, verifierIdx []uint8) (*ContractAlignedLayerServiceManagerVerifierEnabledIterator, error) {

	var verifierIdxRule []interface{}
	for _, verifierIdxItem := range verifierIdx {
		verifierIdxRule = append(verifierIdxRule, verifierIdxItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.FilterLogs(opts, "VerifierEnabled", verifierIdxRule)
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerVerifierEnabledIterator{contract: _ContractAlignedLayerServiceManager.contract, event: "VerifierEnabled", logs: logs, sub: sub}, nil
}

// WatchVerifierEnabled is a free log subscription operation binding the contract event 0x5f52704e8e0190647930ccde0e43e14e89902d7d8c49c5f9e2544029f45ec12a.
//
// Solidity: event VerifierEnabled(uint8 indexed verifierIdx)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) WatchVerifierEnabled(opts *bind.WatchOpts, sink chan<- *ContractAlignedLayerServiceManagerVerifierEnabled, verifierIdx []uint8) (event.Subscription, error) {

	var verifierIdxRule []interface{}
	for _, verifierIdxItem := range verifierIdx {
		verifierIdxRule = append(verifierIdxRule, verifierIdxItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.WatchLogs(opts, "VerifierEnabled", verifierIdxRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractAlignedLayerServiceManagerVerifierEnabled)
				if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "VerifierEnabled", log); err != nil {
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

// ParseVerifierEnabled is a log parse operation binding the contract event 0x5f52704e8e0190647930ccde0e43e14e89902d7d8c49c5f9e2544029f45ec12a.
//
// Solidity: event VerifierEnabled(uint8 indexed verifierIdx)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) ParseVerifierEnabled(log types.Log) (*ContractAlignedLayerServiceManagerVerifierEnabled, error) {
	event := new(ContractAlignedLayerServiceManagerVerifierEnabled)
	if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "VerifierEnabled", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
