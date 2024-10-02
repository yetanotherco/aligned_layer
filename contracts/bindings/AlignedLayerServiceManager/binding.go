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
	ABI: "[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"__avsDirectory\",\"type\":\"address\",\"internalType\":\"contractIAVSDirectory\"},{\"name\":\"__rewardsCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRewardsCoordinator\"},{\"name\":\"__registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"__stakeRegistry\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"receive\",\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"alignedAggregator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"avsDirectory\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"balanceOf\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"batchersBalances\",\"inputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"batchesState\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"responded\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"respondToTaskFeeLimit\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blacklistVerifier\",\"inputs\":[{\"name\":\"verifierIdx\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"blacklistedVerifiers\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsApkRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBLSApkRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkPublicInput\",\"inputs\":[{\"name\":\"publicInput\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"hash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"pure\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"params\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createAVSRewardsSubmission\",\"inputs\":[{\"name\":\"rewardsSubmissions\",\"type\":\"tuple[]\",\"internalType\":\"structIRewardsCoordinator.RewardsSubmission[]\",\"components\":[{\"name\":\"strategiesAndMultipliers\",\"type\":\"tuple[]\",\"internalType\":\"structIRewardsCoordinator.StrategyAndMultiplier[]\",\"components\":[{\"name\":\"strategy\",\"type\":\"address\",\"internalType\":\"contractIStrategy\"},{\"name\":\"multiplier\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"token\",\"type\":\"address\",\"internalType\":\"contractIERC20\"},{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"startTimestamp\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"duration\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"createNewTask\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"batchDataPointer\",\"type\":\"string\",\"internalType\":\"string\"},{\"name\":\"respondToTaskFeeLimit\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"delegation\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIDelegationManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"depositToBatcher\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"deregisterOperatorFromAVS\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"getOperatorRestakedStrategies\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getRestakeableStrategies\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_rewardsInitiator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_alignedAggregator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"initializeAggregator\",\"inputs\":[{\"name\":\"_alignedAggregator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"isVerifierBlacklisted\",\"inputs\":[{\"name\":\"verifierIdx\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registerOperatorToAVS\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorSignature\",\"type\":\"tuple\",\"internalType\":\"structISignatureUtils.SignatureWithSaltAndExpiry\",\"components\":[{\"name\":\"signature\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"salt\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"expiry\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"registryCoordinator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToTaskV2\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"rewardsInitiator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setAggregator\",\"inputs\":[{\"name\":\"_alignedAggregator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setRewardsInitiator\",\"inputs\":[{\"name\":\"newRewardsInitiator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setStaleStakesForbidden\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setVerifiersBlacklist\",\"inputs\":[{\"name\":\"bitmap\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stakeRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"staleStakesForbidden\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"trySignatureAndApkVerification\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"apk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[{\"name\":\"pairingSuccessful\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"siganatureIsValid\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"updateAVSMetadataURI\",\"inputs\":[{\"name\":\"_metadataURI\",\"type\":\"string\",\"internalType\":\"string\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"verifyBatchInclusion\",\"inputs\":[{\"name\":\"proofCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pubInputCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"provingSystemAuxDataCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"proofGeneratorAddr\",\"type\":\"bytes20\",\"internalType\":\"bytes20\"},{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"merkleProof\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"verificationDataBatchIndex\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"verifyBatchInclusion\",\"inputs\":[{\"name\":\"proofCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pubInputCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"provingSystemAuxDataCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"proofGeneratorAddr\",\"type\":\"bytes20\",\"internalType\":\"bytes20\"},{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"merkleProof\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"verificationDataBatchIndex\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"whitelistVerifier\",\"inputs\":[{\"name\":\"verifierIdx\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"withdraw\",\"inputs\":[{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"BatchVerified\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"BatcherBalanceUpdated\",\"inputs\":[{\"name\":\"batcher\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newBalance\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewBatchV2\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"batchDataPointer\",\"type\":\"string\",\"indexed\":false,\"internalType\":\"string\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewBatchV3\",\"inputs\":[{\"name\":\"batchMerkleRoot\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"senderAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"batchDataPointer\",\"type\":\"string\",\"indexed\":false,\"internalType\":\"string\"},{\"name\":\"respondToTaskFeeLimit\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RewardsInitiatorUpdated\",\"inputs\":[{\"name\":\"prevRewardsInitiator\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"newRewardsInitiator\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"StaleStakesForbiddenUpdate\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"VerifierBlacklisted\",\"inputs\":[{\"name\":\"verifierIdx\",\"type\":\"uint8\",\"indexed\":true,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"VerifierWhitelisted\",\"inputs\":[{\"name\":\"verifierIdx\",\"type\":\"uint8\",\"indexed\":true,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"error\",\"name\":\"BatchAlreadyResponded\",\"inputs\":[{\"name\":\"batchIdentifierHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"BatchAlreadySubmitted\",\"inputs\":[{\"name\":\"batchIdentifierHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"BatchDoesNotExist\",\"inputs\":[{\"name\":\"batchIdentifierHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"ExceededMaxRespondFee\",\"inputs\":[{\"name\":\"respondToTaskFeeLimit\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"txCost\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"InsufficientFunds\",\"inputs\":[{\"name\":\"batcher\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"required\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"available\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"InvalidAddress\",\"inputs\":[{\"name\":\"param\",\"type\":\"string\",\"internalType\":\"string\"}]},{\"type\":\"error\",\"name\":\"InvalidDepositAmount\",\"inputs\":[{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"InvalidQuorumThreshold\",\"inputs\":[{\"name\":\"signedStake\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"requiredStake\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"SenderIsNotAggregator\",\"inputs\":[{\"name\":\"sender\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"alignedAggregator\",\"type\":\"address\",\"internalType\":\"address\"}]}]",
	Bin: "0x61018060405234801561001157600080fd5b506040516159d23803806159d2833981016040819052610030916103fb565b6001600160a01b0380851660805280841660a05280831660c052811660e052818484828461005c610327565b50505050806001600160a01b0316610100816001600160a01b031681525050806001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100b9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100dd919061045a565b6001600160a01b0316610120816001600160a01b031681525050806001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa158015610135573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610159919061045a565b6001600160a01b0316610140816001600160a01b031681525050610120516001600160a01b031663df5cf7236040518163ffffffff1660e01b8152600401602060405180830381865afa1580156101b4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101d8919061045a565b6001600160a01b03908116610160528516905061022c57604051630b0f5aa160e11b815260206004820152600c60248201526b6176734469726563746f727960a01b60448201526064015b60405180910390fd5b6001600160a01b03831661027857604051630b0f5aa160e11b81526020600482015260126024820152713932bbb0b93239a1b7b7b93234b730ba37b960711b6044820152606401610223565b6001600160a01b0382166102cf57604051630b0f5aa160e11b815260206004820152601360248201527f7265676973747279436f6f7264696e61746f72000000000000000000000000006044820152606401610223565b6001600160a01b03811661031657604051630b0f5aa160e11b815260206004820152600d60248201526c7374616b65526567697374727960981b6044820152606401610223565b61031e610327565b5050505061047e565b600054610100900460ff161561038f5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608401610223565b60005460ff90811610156103e1576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146103f857600080fd5b50565b6000806000806080858703121561041157600080fd5b845161041c816103e3565b602086015190945061042d816103e3565b604086015190935061043e816103e3565b606086015190925061044f816103e3565b939692955090935050565b60006020828403121561046c57600080fd5b8151610477816103e3565b9392505050565b60805160a05160c05160e0516101005161012051610140516101605161544761058b6000396000818161073001526118b90152600081816103fd0152611acc01526000818161043101528181611cb90152611ea9015260008181610498015281816110eb0152818161157f01528181611726015261196d015260008181610e2001528181610f710152818161100801528181612cc301528181612e3c0152612edb015260008181610c4701528181610cd601528181610d56015281816122420152818161235601528181612bfe0152612d970152600081816132920152818161334e015261343101526000818161046201528181612296015281816123b2015261243101526154476000f3fe6080604052600436106102345760003560e01c80639926ee7d1161012e578063d66eaabd116100ab578063f474b5201161006f578063f474b52014610787578063f9120af6146107b4578063fa534dc0146107d4578063fc299dee146107f4578063fce36c7d1461081457600080fd5b8063d66eaabd146106eb578063db45b1bf146106fe578063df5cf7231461071e578063e481af9d14610752578063f2fde38b1461076757600080fd5b8063b099627e116100f2578063b099627e14610611578063b98d09081461067b578063bd11c55a14610695578063c0c53b8b146106ab578063c97418c0146106cb57600080fd5b80639926ee7d14610571578063a24a669014610591578063a364f4da146105b1578063a98fb355146105d1578063ab21739a146105f157600080fd5b80635c9244ca116101bc57806370a082311161018057806370a08231146104ba578063715018a6146104fe578063800fb61f146105135780638da5cb5b1461053357806395c6d6041461055157600080fd5b80635c9244ca146103bc5780635df45946146103eb578063683048351461041f5780636b3aa72e146104535780636d14a9871461048657600080fd5b80633bc28c8c116102035780633bc28c8c14610303578063416c7e5e146103235780634223d551146103435780634a5bf632146103565780634ae07c371461038e57600080fd5b806306045a911461024a578063171f1d5b1461027f5780632e1a7d4d146102b657806333cfb7b7146102d657600080fd5b36610245576102433334610834565b005b600080fd5b34801561025657600080fd5b5061026a6102653660046143b0565b6108c9565b60405190151581526020015b60405180910390f35b34801561028b57600080fd5b5061029f61029a366004614503565b6109c0565b604080519215158352901515602083015201610276565b3480156102c257600080fd5b506102436102d1366004614554565b610b4a565b3480156102e257600080fd5b506102f66102f136600461456d565b610c22565b604051610276919061458a565b34801561030f57600080fd5b5061024361031e36600461456d565b6110d5565b34801561032f57600080fd5b5061024361033e3660046145d9565b6110e9565b61024361035136600461456d565b611220565b34801561036257600080fd5b5060cc54610376906001600160a01b031681565b6040516001600160a01b039091168152602001610276565b34801561039a57600080fd5b506103ae6103a93660046148d1565b61122a565b60405161027692919061496c565b3480156103c857600080fd5b5061026a6103d73660046149c4565b60ca54600160ff9092169190911b16151590565b3480156103f757600080fd5b506103767f000000000000000000000000000000000000000000000000000000000000000081565b34801561042b57600080fd5b506103767f000000000000000000000000000000000000000000000000000000000000000081565b34801561045f57600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610376565b34801561049257600080fd5b506103767f000000000000000000000000000000000000000000000000000000000000000081565b3480156104c657600080fd5b506104f06104d536600461456d565b6001600160a01b0316600090815260cb602052604090205490565b604051908152602001610276565b34801561050a57600080fd5b5061024361215e565b34801561051f57600080fd5b5061024361052e36600461456d565b612172565b34801561053f57600080fd5b506033546001600160a01b0316610376565b34801561055d57600080fd5b5061026a61056c366004614a29565b612212565b34801561057d57600080fd5b5061024361058c366004614a74565b612237565b34801561059d57600080fd5b506102436105ac3660046149c4565b612303565b3480156105bd57600080fd5b506102436105cc36600461456d565b61234b565b3480156105dd57600080fd5b506102436105ec366004614b2b565b612412565b3480156105fd57600080fd5b5061024361060c366004614b7b565b612466565b34801561061d57600080fd5b5061065961062c366004614554565b60c9602052600090815260409020805460019091015463ffffffff821691640100000000900460ff169083565b6040805163ffffffff9094168452911515602084015290820152606001610276565b34801561068757600080fd5b5060975461026a9060ff1681565b3480156106a157600080fd5b506104f060ca5481565b3480156106b757600080fd5b506102436106c6366004614ba2565b612825565b3480156106d757600080fd5b506102436106e63660046149c4565b6129ea565b6102436106f9366004614bed565b612a31565b34801561070a57600080fd5b50610243610719366004614554565b612beb565b34801561072a57600080fd5b506103767f000000000000000000000000000000000000000000000000000000000000000081565b34801561075e57600080fd5b506102f6612bf8565b34801561077357600080fd5b5061024361078236600461456d565b612fa4565b34801561079357600080fd5b506104f06107a236600461456d565b60cb6020526000908152604090205481565b3480156107c057600080fd5b506102436107cf36600461456d565b61301a565b3480156107e057600080fd5b5061026a6107ef366004614c3f565b613044565b34801561080057600080fd5b50606554610376906001600160a01b031681565b34801561082057600080fd5b5061024361082f366004614cbf565b6130b9565b8060000361085d57604051632097692160e11b8152600481018290526024015b60405180910390fd5b6001600160a01b038216600090815260cb602052604081208054839290610885908490614d4a565b90915550506001600160a01b038216600081815260cb60209081526040918290205491519182526000805160206153d2833981519152910160405180910390a25050565b6000806001600160a01b0383166108e157508461090d565b85836040516020016108f4929190614d5d565b6040516020818303038152906040528051906020012090505b600081815260c9602052604081205463ffffffff1690036109325760009150506109b4565b600081815260c96020526040902054640100000000900460ff1661095a5760009150506109b4565b60408051602081018c90529081018a9052606081018990526001600160601b03198816608082015260009060940160408051601f19818403018152919052805160208201209091506109ae87898389613468565b93505050505b98975050505050505050565b60008060007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000187876000015188602001518860000151600060028110610a0857610a08614d78565b60200201518951600160200201518a60200151600060028110610a2d57610a2d614d78565b60200201518b60200151600160028110610a4957610a49614d78565b602090810291909101518c518d830151604051610aa69a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b6040516020818303038152906040528051906020012060001c610ac99190614d8e565b9050610b3c610ae2610adb8884613480565b8690613511565b610aea6135a6565b610b32610b2385610b1d604080518082018252600080825260209182015281518083019092526001825260029082015290565b90613480565b610b2c8c613666565b90613511565b886201d4c06136f5565b909890975095505050505050565b33600090815260cb6020526040902054811115610b9b5733600081815260cb602052604090819020549051632e2a182f60e11b81526004810192909252602482018390526044820152606401610854565b33600090815260cb602052604081208054839290610bba908490614db0565b909155505033600081815260cb60209081526040918290205491519182526000805160206153d2833981519152910160405180910390a2604051339082156108fc029083906000818181858888f19350505050158015610c1e573d6000803e3d6000fd5b5050565b6040516309aa152760e11b81526001600160a01b0382811660048301526060916000917f000000000000000000000000000000000000000000000000000000000000000016906313542a4e90602401602060405180830381865afa158015610c8e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610cb29190614dc3565b60405163871ef04960e01b8152600481018290529091506000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063871ef04990602401602060405180830381865afa158015610d1d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d419190614ddc565b90506001600160c01b0381161580610ddb57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610db2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610dd69190614e05565b60ff16155b15610dfb5760408051600080825260208201909252905b50949350505050565b6000610e0f826001600160c01b031661390f565b90506000805b8251811015610edb577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316633ca5a5f5848381518110610e5f57610e5f614d78565b01602001516040516001600160e01b031960e084901b16815260f89190911c6004820152602401602060405180830381865afa158015610ea3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ec79190614dc3565b610ed19083614d4a565b9150600101610e15565b506000816001600160401b03811115610ef657610ef6614288565b604051908082528060200260200182016040528015610f1f578160200160208202803683370190505b5090506000805b84518110156110c8576000858281518110610f4357610f43614d78565b0160200151604051633ca5a5f560e01b815260f89190911c6004820181905291506000906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633ca5a5f590602401602060405180830381865afa158015610fb8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fdc9190614dc3565b905060005b818110156110bd576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa158015611056573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061107a9190614e37565b6000015186868151811061109057611090614d78565b6001600160a01b0390921660209283029190910190910152846110b281614e7a565b955050600101610fe1565b505050600101610f26565b5090979650505050505050565b6110dd6139d1565b6110e681613a2b565b50565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611147573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061116b9190614e93565b6001600160a01b0316336001600160a01b0316146112175760405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a401610854565b6110e681613a94565b6110e68134610834565b6040805180820190915260608082526020820152600082604001515160405180604001604052806001815260200160008152505114801561128657508260a0015151604051806040016040528060018152602001600081525051145b80156112ad57508260c0015151604051806040016040528060018152602001600081525051145b80156112d457508260e0015151604051806040016040528060018152602001600081525051145b61133e5760405162461bcd60e51b815260206004820152604160248201526000805160206153f283398151915260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a401610854565b825151602084015151146113b65760405162461bcd60e51b8152602060048201526044602482018190526000805160206153f2833981519152908201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b608482015260a401610854565b4363ffffffff168463ffffffff16106114255760405162461bcd60e51b815260206004820152603c60248201526000805160206153f283398151915260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b000000006064820152608401610854565b60408051808201825260008082526020808301829052835180850185526060808252818301528451808601865260018082529083019390935284518381528086019095529293919082810190803683370190505060208281019190915260408051808201825260018082526000919093015280518281528082019091529081602001602082028036833701905050815260408051808201909152606080825260208201528560200151516001600160401b038111156114e6576114e6614288565b60405190808252806020026020018201604052801561150f578160200160208202803683370190505b5081526020860151516001600160401b0381111561152f5761152f614288565b604051908082528060200260200182016040528015611558578160200160208202803683370190505b508160200181905250600061160460405180604001604052806001815260200160008152507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156115db573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115ff9190614e05565b613adb565b905060005b8760200151518110156118955761164e8860200151828151811061162f5761162f614d78565b6020026020010151805160009081526020918201519091526040902090565b8360200151828151811061166457611664614d78565b60209081029190910101528015611724576020830151611685600183614db0565b8151811061169557611695614d78565b602002602001015160001c836020015182815181106116b6576116b6614d78565b602002602001015160001c11611724576040805162461bcd60e51b81526020600482015260248101919091526000805160206153f283398151915260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152608401610854565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166304ec63518460200151838151811061176957611769614d78565b60200260200101518b8b60000151858151811061178857611788614d78565b60200260200101516040518463ffffffff1660e01b81526004016117c59392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa1580156117e2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118069190614ddc565b6001600160c01b03168360000151828151811061182557611825614d78565b60200260200101818152505061188b610adb61185f848660000151858151811061185157611851614d78565b602002602001015116613b6e565b8a60200151848151811061187557611875614d78565b6020026020010151613b9990919063ffffffff16565b9450600101611609565b50506118a083613c7c565b60975490935060ff166000816118b7576000611939565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c448feb86040518163ffffffff1660e01b8152600401602060405180830381865afa158015611915573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906119399190614dc3565b905060005b60405180604001604052806001815260200160008152505181101561202f578215611aca578963ffffffff16827f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663249a0c42604051806040016040528060018152602001600081525085815181106119c2576119c2614d78565b01602001516040516001600160e01b031960e084901b16815260f89190911c6004820152602401602060405180830381865afa158015611a06573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a2a9190614dc3565b611a349190614d4a565b11611aca5760405162461bcd60e51b815260206004820152606660248201526000805160206153f283398151915260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c401610854565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166368bccaac60405180604001604052806001815260200160008152508381518110611b2157611b21614d78565b602001015160f81c60f81b60f81c8c8c60a001518581518110611b4657611b46614d78565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015611ba2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611bc69190614eb0565b6001600160401b031916611be98a60400151838151811061162f5761162f614d78565b67ffffffffffffffff191614611c855760405162461bcd60e51b815260206004820152606160248201526000805160206153f283398151915260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c401610854565b611cb589604001518281518110611c9e57611c9e614d78565b60200260200101518761351190919063ffffffff16565b95507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c8294c5660405180604001604052806001815260200160008152508381518110611d0e57611d0e614d78565b602001015160f81c60f81b60f81c8c8c60c001518581518110611d3357611d33614d78565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015611d8f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611db39190614edb565b85602001518281518110611dc957611dc9614d78565b6001600160601b03909216602092830291909101820152850151805182908110611df557611df5614d78565b602002602001015185600001518281518110611e1357611e13614d78565b60200260200101906001600160601b031690816001600160601b0316815250506000805b8a602001515181101561202557611ea286600001518281518110611e5d57611e5d614d78565b602002602001015160405180604001604052806001815260200160008152508581518110611e8d57611e8d614d78565b016020015160f81c60ff161c60019081161490565b1561201d577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f2be94ae60405180604001604052806001815260200160008152508581518110611efe57611efe614d78565b602001015160f81c60f81b60f81c8e89602001518581518110611f2357611f23614d78565b60200260200101518f60e001518881518110611f4157611f41614d78565b60200260200101518781518110611f5a57611f5a614d78565b60209081029190910101516040516001600160e01b031960e087901b16815260ff909416600485015263ffffffff92831660248501526044840191909152166064820152608401602060405180830381865afa158015611fbe573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611fe29190614edb565b8751805185908110611ff657611ff6614d78565b6020026020010181815161200a9190614ef8565b6001600160601b03169052506001909101905b600101611e37565b505060010161193e565b5050506000806120498a868a606001518b608001516109c0565b91509150816120ba5760405162461bcd60e51b815260206004820152604360248201526000805160206153f283398151915260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a401610854565b8061211b5760405162461bcd60e51b815260206004820152603960248201526000805160206153f283398151915260448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608401610854565b50506000878260200151604051602001612136929190614f17565b60408051808303601f1901815291905280516020909101209299929850919650505050505050565b6121666139d1565b6121706000613d17565b565b600054600290610100900460ff16158015612194575060005460ff8083169116105b6121b05760405162461bcd60e51b815260040161085490614f5f565b6000805461ffff191660ff8316176101001790556121cd8261301a565b6000805461ff001916905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b6000818484604051612225929190614fad565b60405180910390201490509392505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461227f5760405162461bcd60e51b815260040161085490614fbd565b604051639926ee7d60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639926ee7d906122cd908590859060040161507b565b600060405180830381600087803b1580156122e757600080fd5b505af11580156122fb573d6000803e3d6000fd5b505050505050565b61230b6139d1565b60ca8054600160ff841690811b199091169091556040517f65c7ff2890067393fb22d496e9fb2ecd8bdb4231818f2cdd28794ea5f12e6c3c90600090a250565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146123935760405162461bcd60e51b815260040161085490614fbd565b6040516351b27a6d60e11b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a364f4da906024015b600060405180830381600087803b1580156123f757600080fd5b505af115801561240b573d6000803e3d6000fd5b5050505050565b61241a6139d1565b60405163a98fb35560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a98fb355906123dd9084906004016150c6565b60cc546001600160a01b031633146124a65760cc54604051632cbe419560e01b81523360048201526001600160a01b039091166024820152604401610854565b60005a9050600084846040516020016124c0929190614d5d565b60408051601f198184030181529181528151602092830120600081815260c990935290822080549193509163ffffffff9091169003612515576040516311cb69a760e11b815260048101839052602401610854565b8054640100000000900460ff161561254357604051634e78d7f960e11b815260048101839052602401610854565b805464ff00000000191664010000000017815560018101546001600160a01b038616600090815260cb602052604090205410156125c65760018101546001600160a01b038616600081815260cb602052604090819020549051632e2a182f60e11b8152600481019290925260248201929092526044810191909152606401610854565b80546000906125dd90849063ffffffff168761122a565b509050604360ff1681602001516000815181106125fc576125fc614d78565b602002602001015161260e91906150d9565b6001600160601b03166064826000015160008151811061263057612630614d78565b60200260200101516001600160601b031661264b9190615102565b10156126de576064816000015160008151811061266a5761266a614d78565b60200260200101516001600160601b03166126859190615102565b604360ff1682602001516000815181106126a1576126a1614d78565b60200260200101516126b391906150d9565b60405163530f5c4560e11b815260048101929092526001600160601b03166024820152604401610854565b6040516001600160a01b038716815287907f8511746b73275e06971968773119b9601fc501d7bdf3824d8754042d148940e29060200160405180910390a260003a5a61272a9087614db0565b6127379062011170614d4a565b6127419190615102565b9050826001015481111561277857600183015460405163437e283f60e11b8152600481019190915260248101829052604401610854565b6001600160a01b038716600090815260cb6020526040812080548392906127a0908490614db0565b90915550506001600160a01b038716600081815260cb60209081526040918290205491519182526000805160206153d2833981519152910160405180910390a260cc546040516001600160a01b039091169082156108fc029083906000818181858888f1935050505015801561281a573d6000803e3d6000fd5b505050505050505050565b600054610100900460ff16158080156128455750600054600160ff909116105b8061285f5750303b15801561285f575060005460ff166001145b61287b5760405162461bcd60e51b815260040161085490614f5f565b6000805460ff19166001179055801561289e576000805461ff0019166101001790555b6001600160a01b0384166128e457604051630b0f5aa160e11b815260206004820152600c60248201526b34b734ba34b0b627bbb732b960a11b6044820152606401610854565b6001600160a01b03831661292e57604051630b0f5aa160e11b815260206004820152601060248201526f3932bbb0b93239a4b734ba34b0ba37b960811b6044820152606401610854565b6001600160a01b03821661297957604051630b0f5aa160e11b815260206004820152601160248201527030b634b3b732b220b3b3b932b3b0ba37b960791b6044820152606401610854565b6129838484613d69565b60cc80546001600160a01b0319166001600160a01b03841617905580156129e4576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b6129f26139d1565b60ca8054600160ff841690811b9091179091556040517ff7d44cdeceb21bc027e9a0aa16d424d59eaaa76c302f7df60e40d68e60a588e790600090a250565b60008433604051602001612a46929190614d5d565b60408051601f198184030181529181528151602092830120600081815260c990935291205490915063ffffffff1615612a9557604051630c40bc4360e21b815260048101829052602401610854565b3415612af25733600090815260cb602052604081208054349290612aba908490614d4a565b909155505033600081815260cb60209081526040918290205491519182526000805160206153d2833981519152910160405180910390a25b33600090815260cb6020526040902054821115612b435733600081815260cb602052604090819020549051632e2a182f60e11b81526004810192909252602482018490526044820152606401610854565b604080516060810182526000602080830182815263ffffffff43818116865285870189815288865260c99094529386902085518154935115156401000000000264ffffffffff1990941692169190911791909117815590516001909101559151909187917f8801fc966deb2c8f563a103c35c9e80740585c292cd97518587e6e7927e6af5591612bdb913391908a908a908a90615119565b60405180910390a2505050505050565b612bf36139d1565b60ca55565b606060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612c5a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612c7e9190614e05565b60ff16905080600003612c9f57505060408051600081526020810190915290565b6000805b82811015612d4a57604051633ca5a5f560e01b815260ff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa158015612d12573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612d369190614dc3565b612d409083614d4a565b9150600101612ca3565b506000816001600160401b03811115612d6557612d65614288565b604051908082528060200260200182016040528015612d8e578160200160208202803683370190505b5090506000805b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612df3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612e179190614e05565b60ff16811015612f9a57604051633ca5a5f560e01b815260ff821660048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa158015612e8b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612eaf9190614dc3565b905060005b81811015612f90576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa158015612f29573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612f4d9190614e37565b60000151858581518110612f6357612f63614d78565b6001600160a01b039092166020928302919091019091015283612f8581614e7a565b945050600101612eb4565b5050600101612d95565b5090949350505050565b612fac6139d1565b6001600160a01b0381166130115760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610854565b6110e681613d17565b6130226139d1565b60cc80546001600160a01b0319166001600160a01b0392909216919091179055565b6040516306045a9160e01b815260009030906306045a9190613078908b908b908b908b908b908b908b908b90600401615170565b602060405180830381865afa158015613095573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109b491906151d2565b6065546001600160a01b0316331461314e5760405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a401610854565b60005b818110156134195782828281811061316b5761316b614d78565b905060200281019061317d91906151ef565b61318e90604081019060200161456d565b6001600160a01b03166323b872dd33308686868181106131b0576131b0614d78565b90506020028101906131c291906151ef565b604080516001600160e01b031960e087901b1681526001600160a01b039485166004820152939092166024840152013560448201526064016020604051808303816000875af1158015613219573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061323d91906151d2565b50600083838381811061325257613252614d78565b905060200281019061326491906151ef565b61327590604081019060200161456d565b604051636eb1769f60e11b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081166024830152919091169063dd62ed3e90604401602060405180830381865afa1580156132e3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906133079190614dc3565b905083838381811061331b5761331b614d78565b905060200281019061332d91906151ef565b61333e90604081019060200161456d565b6001600160a01b031663095ea7b37f00000000000000000000000000000000000000000000000000000000000000008387878781811061338057613380614d78565b905060200281019061339291906151ef565b604001356133a09190614d4a565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044016020604051808303816000875af11580156133eb573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061340f91906151d2565b5050600101613151565b5060405163fce36c7d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063fce36c7d906122cd9085908590600401615276565b600083613476868585613de6565b1495945050505050565b604080518082019091526000808252602082015261349c614196565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa905080806134cb57fe5b50806135095760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401610854565b505092915050565b604080518082019091526000808252602082015261352d6141b4565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa9050808061356857fe5b50806135095760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606401610854565b6135ae6141d2565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b6040805180820190915260008082526020820152600080806136966000805160206153b283398151915286614d8e565b90505b6136a281613ee3565b90935091506000805160206153b283398151915282830983036136db576040805180820190915290815260208101919091529392505050565b6000805160206153b2833981519152600182089050613699565b6040805180820182528681526020808201869052825180840190935286835282018490526000918291906137276141f7565b60005b60028110156138e2576000613740826006615102565b905084826002811061375457613754614d78565b60200201515183613766836000614d4a565b600c811061377657613776614d78565b602002015284826002811061378d5761378d614d78565b602002015160200151838260016137a49190614d4a565b600c81106137b4576137b4614d78565b60200201528382600281106137cb576137cb614d78565b60200201515151836137de836002614d4a565b600c81106137ee576137ee614d78565b602002015283826002811061380557613805614d78565b602002015151600160200201518361381e836003614d4a565b600c811061382e5761382e614d78565b602002015283826002811061384557613845614d78565b60200201516020015160006002811061386057613860614d78565b602002015183613871836004614d4a565b600c811061388157613881614d78565b602002015283826002811061389857613898614d78565b6020020151602001516001600281106138b3576138b3614d78565b6020020151836138c4836005614d4a565b600c81106138d4576138d4614d78565b60200201525060010161372a565b506138eb614216565b60006020826101808560088cfa9151919c9115159b50909950505050505050505050565b606060008061391d84613b6e565b61ffff166001600160401b0381111561393857613938614288565b6040519080825280601f01601f191660200182016040528015613962576020820181803683370190505b5090506000805b82518210801561397a575061010081105b15612f9a576001811b9350858416156139c1578060f81b8383815181106139a3576139a3614d78565b60200101906001600160f81b031916908160001a9053508160010191505b6139ca81614e7a565b9050613969565b6033546001600160a01b031633146121705760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610854565b606554604080516001600160a01b03928316815291831660208301527fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6097805460ff19168215159081179091556040519081527f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc9060200160405180910390a150565b600080613ae784613f65565b9050808360ff166001901b11613b655760405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c7565006064820152608401610854565b90505b92915050565b6000805b8215613b6857613b83600184614db0565b9092169180613b9181615390565b915050613b72565b60408051808201909152600080825260208201526102008261ffff1610613bf55760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606401610854565b8161ffff16600103613c08575081613b68565b6040805180820190915260008082526020820181905284906001905b8161ffff168661ffff1610613c7157600161ffff871660ff83161c81169003613c5457613c518484613511565b93505b613c5e8384613511565b92506201fffe600192831b169101613c24565b509195945050505050565b60408051808201909152600080825260208201528151158015613ca157506020820151155b15613cbf575050604080518082019091526000808252602082015290565b6040518060400160405280836000015181526020016000805160206153b28339815191528460200151613cf29190614d8e565b613d0a906000805160206153b2833981519152614db0565b905292915050565b919050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b600054610100900460ff16613dd45760405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608401610854565b613ddd82613d17565b610c1e81613a2b565b600060208451613df69190614d8e565b15613e7d5760405162461bcd60e51b815260206004820152604b60248201527f4d65726b6c652e70726f63657373496e636c7573696f6e50726f6f664b65636360448201527f616b3a2070726f6f66206c656e6774682073686f756c642062652061206d756c60648201526a3a34b836329037b310199960a91b608482015260a401610854565b8260205b85518111610df257613e94600285614d8e565b600003613eb857816000528086015160205260406000209150600284049350613ed1565b8086015160005281602052604060002091506002840493505b613edc602082614d4a565b9050613e81565b600080806000805160206153b283398151915260036000805160206153b2833981519152866000805160206153b2833981519152888909090890506000613f59827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f526000805160206153b28339815191526140ed565b91959194509092505050565b600061010082511115613fee5760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a401610854565b8151600003613fff57506000919050565b6000808360008151811061401557614015614d78565b0160200151600160f89190911c81901b92505b84518110156140e45784818151811061404357614043614d78565b0160200151600160f89190911c1b91508282116140d85760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a401610854565b91811791600101614028565b50909392505050565b6000806140f8614216565b614100614234565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa9250828061413d57fe5b508261418b5760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606401610854565b505195945050505050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b60405180604001604052806141e5614252565b81526020016141f2614252565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b80356001600160601b031981168114613d1257600080fd5b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156142c0576142c0614288565b60405290565b60405161010081016001600160401b03811182821017156142c0576142c0614288565b604051601f8201601f191681016001600160401b038111828210171561431157614311614288565b604052919050565b6000806001600160401b0384111561433357614333614288565b50601f8301601f1916602001614348816142e9565b91505082815283838301111561435d57600080fd5b828260208301376000602084830101529392505050565b600082601f83011261438557600080fd5b61439483833560208501614319565b9392505050565b6001600160a01b03811681146110e657600080fd5b600080600080600080600080610100898b0312156143cd57600080fd5b8835975060208901359650604089013595506143eb60608a01614270565b94506080890135935060a08901356001600160401b0381111561440d57600080fd5b6144198b828c01614374565b93505060c0890135915060e08901356144318161439b565b809150509295985092959890939650565b60006040828403121561445457600080fd5b61445c61429e565b823581526020928301359281019290925250919050565b600082601f83011261448457600080fd5b61448c61429e565b80604084018581111561449e57600080fd5b845b818110156144b85780358452602093840193016144a0565b509095945050505050565b6000608082840312156144d557600080fd5b6144dd61429e565b90506144e98383614473565b81526144f88360408401614473565b602082015292915050565b600080600080610120858703121561451a57600080fd5b8435935061452b8660208701614442565b925061453a86606087016144c3565b91506145498660e08701614442565b905092959194509250565b60006020828403121561456657600080fd5b5035919050565b60006020828403121561457f57600080fd5b8135613b658161439b565b602080825282518282018190526000918401906040840190835b818110156144b85783516001600160a01b03168352602093840193909201916001016145a4565b80151581146110e657600080fd5b6000602082840312156145eb57600080fd5b8135613b65816145cb565b803563ffffffff81168114613d1257600080fd5b60006001600160401b0382111561462357614623614288565b5060051b60200190565b600082601f83011261463e57600080fd5b813561465161464c8261460a565b6142e9565b8082825260208201915060208360051b86010192508583111561467357600080fd5b602085015b8381101561469757614689816145f6565b835260209283019201614678565b5095945050505050565b600082601f8301126146b257600080fd5b81356146c061464c8261460a565b8082825260208201915060208360061b8601019250858311156146e257600080fd5b602085015b83811015614697576146f98782614442565b83526020909201916040016146e7565b600082601f83011261471a57600080fd5b813561472861464c8261460a565b8082825260208201915060208360051b86010192508583111561474a57600080fd5b602085015b838110156146975780356001600160401b0381111561476d57600080fd5b61477c886020838a010161462d565b8452506020928301920161474f565b6000610180828403121561479e57600080fd5b6147a66142c6565b905081356001600160401b038111156147be57600080fd5b6147ca8482850161462d565b82525060208201356001600160401b038111156147e657600080fd5b6147f2848285016146a1565b60208301525060408201356001600160401b0381111561481157600080fd5b61481d848285016146a1565b60408301525061483083606084016144c3565b60608201526148428360e08401614442565b60808201526101208201356001600160401b0381111561486157600080fd5b61486d8482850161462d565b60a0830152506101408201356001600160401b0381111561488d57600080fd5b6148998482850161462d565b60c0830152506101608201356001600160401b038111156148b957600080fd5b6148c584828501614709565b60e08301525092915050565b6000806000606084860312156148e657600080fd5b833592506148f6602085016145f6565b915060408401356001600160401b0381111561491157600080fd5b61491d8682870161478b565b9150509250925092565b600081518084526020840193506020830160005b828110156149625781516001600160601b031686526020958601959091019060010161493b565b5093949350505050565b60408152600083516040808401526149876080840182614927565b90506020850151603f198483030160608501526149a48282614927565b925050508260208301529392505050565b60ff811681146110e657600080fd5b6000602082840312156149d657600080fd5b8135613b65816149b5565b60008083601f8401126149f357600080fd5b5081356001600160401b03811115614a0a57600080fd5b602083019150836020828501011115614a2257600080fd5b9250929050565b600080600060408486031215614a3e57600080fd5b83356001600160401b03811115614a5457600080fd5b614a60868287016149e1565b909790965060209590950135949350505050565b60008060408385031215614a8757600080fd5b8235614a928161439b565b915060208301356001600160401b03811115614aad57600080fd5b830160608186031215614abf57600080fd5b604051606081016001600160401b0381118282101715614ae157614ae1614288565b60405281356001600160401b03811115614afa57600080fd5b614b0687828501614374565b8252506020828101359082015260409182013591810191909152919491935090915050565b600060208284031215614b3d57600080fd5b81356001600160401b03811115614b5357600080fd5b8201601f81018413614b6457600080fd5b614b7384823560208401614319565b949350505050565b600080600060608486031215614b9057600080fd5b8335925060208401356148f68161439b565b600080600060608486031215614bb757600080fd5b8335614bc28161439b565b92506020840135614bd28161439b565b91506040840135614be28161439b565b809150509250925092565b60008060008060608587031215614c0357600080fd5b8435935060208501356001600160401b03811115614c2057600080fd5b614c2c878288016149e1565b9598909750949560400135949350505050565b600080600080600080600060e0888a031215614c5a57600080fd5b873596506020880135955060408801359450614c7860608901614270565b93506080880135925060a08801356001600160401b03811115614c9a57600080fd5b614ca68a828b01614374565b979a969950949793969295929450505060c09091013590565b60008060208385031215614cd257600080fd5b82356001600160401b03811115614ce857600080fd5b8301601f81018513614cf957600080fd5b80356001600160401b03811115614d0f57600080fd5b8560208260051b8401011115614d2457600080fd5b6020919091019590945092505050565b634e487b7160e01b600052601160045260246000fd5b80820180821115613b6857613b68614d34565b91825260601b6001600160601b031916602082015260340190565b634e487b7160e01b600052603260045260246000fd5b600082614dab57634e487b7160e01b600052601260045260246000fd5b500690565b81810381811115613b6857613b68614d34565b600060208284031215614dd557600080fd5b5051919050565b600060208284031215614dee57600080fd5b81516001600160c01b0381168114613b6557600080fd5b600060208284031215614e1757600080fd5b8151613b65816149b5565b6001600160601b03811681146110e657600080fd5b60006040828403128015614e4a57600080fd5b50614e5361429e565b8251614e5e8161439b565b81526020830151614e6e81614e22565b60208201529392505050565b600060018201614e8c57614e8c614d34565b5060010190565b600060208284031215614ea557600080fd5b8151613b658161439b565b600060208284031215614ec257600080fd5b815167ffffffffffffffff1981168114613b6557600080fd5b600060208284031215614eed57600080fd5b8151613b6581614e22565b6001600160601b038281168282160390811115613b6857613b68614d34565b63ffffffff60e01b8360e01b16815260006004820183516020850160005b82811015614f53578151845260209384019390910190600101614f35565b50919695505050505050565b6020808252602e908201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160408201526d191e481a5b9a5d1a585b1a5e995960921b606082015260800190565b8183823760009101908152919050565b60208082526052908201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360408201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560608201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608082015260a00190565b6000815180845260005b8181101561505b5760208185018101518683018201520161503f565b506000602082860101526020601f19601f83011685010191505092915050565b60018060a01b03831681526040602082015260008251606060408401526150a560a0840182615035565b90506020840151606084015260408401516080840152809150509392505050565b6020815260006143946020830184615035565b6001600160601b0381811683821602908116908181146150fb576150fb614d34565b5092915050565b8082028115828204841417613b6857613b68614d34565b6001600160a01b038616815263ffffffff851660208201526080604082018190528101839052828460a0830137600060a08483010152600060a0601f19601f86011683010190508260608301529695505050505050565b8881528760208201528660408201526001600160601b03198616606082015284608082015261010060a082015260006151ad610100830186615035565b60c0830194909452506001600160a01b039190911660e0909101529695505050505050565b6000602082840312156151e457600080fd5b8151613b65816145cb565b60008235609e1983360301811261520557600080fd5b9190910192915050565b8035613d128161439b565b81835260208301925060008160005b8481101561496257813561523c8161439b565b6001600160a01b03168652602082013561525581614e22565b6001600160601b031660208701526040958601959190910190600101615229565b6020808252810182905260006040600584901b830181019083018583609e1936839003015b8782101561538357868503603f1901845282358181126152ba57600080fd5b8901803536829003601e190181126152d157600080fd5b81016020810190356001600160401b038111156152ed57600080fd5b8060061b36038213156152ff57600080fd5b60a0885261531160a08901828461521a565b9150506153206020830161520f565b6001600160a01b0316602088015260408281013590880152615344606083016145f6565b63ffffffff16606088015261535b608083016145f6565b63ffffffff81166080890152915095505060209384019392909201916001919091019061529b565b5092979650505050505050565b600061ffff821661ffff81036153a8576153a8614d34565b6001019291505056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd470ea46f246ccfc58f7a93aa09bc6245a6818e97b1a160d186afe78993a3b194a0424c535369676e6174757265436865636b65722e636865636b5369676e617475a26469706673582212201be710bbf5a9c362ed3cf16cb9edfa8a72b44939221300bc8753c510ac93084964736f6c634300081a0033",
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

// BlacklistedVerifiers is a free data retrieval call binding the contract method 0xbd11c55a.
//
// Solidity: function blacklistedVerifiers() view returns(uint256)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) BlacklistedVerifiers(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "blacklistedVerifiers")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// BlacklistedVerifiers is a free data retrieval call binding the contract method 0xbd11c55a.
//
// Solidity: function blacklistedVerifiers() view returns(uint256)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) BlacklistedVerifiers() (*big.Int, error) {
	return _ContractAlignedLayerServiceManager.Contract.BlacklistedVerifiers(&_ContractAlignedLayerServiceManager.CallOpts)
}

// BlacklistedVerifiers is a free data retrieval call binding the contract method 0xbd11c55a.
//
// Solidity: function blacklistedVerifiers() view returns(uint256)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) BlacklistedVerifiers() (*big.Int, error) {
	return _ContractAlignedLayerServiceManager.Contract.BlacklistedVerifiers(&_ContractAlignedLayerServiceManager.CallOpts)
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

// IsVerifierBlacklisted is a free data retrieval call binding the contract method 0x5c9244ca.
//
// Solidity: function isVerifierBlacklisted(uint8 verifierIdx) view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCaller) IsVerifierBlacklisted(opts *bind.CallOpts, verifierIdx uint8) (bool, error) {
	var out []interface{}
	err := _ContractAlignedLayerServiceManager.contract.Call(opts, &out, "isVerifierBlacklisted", verifierIdx)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// IsVerifierBlacklisted is a free data retrieval call binding the contract method 0x5c9244ca.
//
// Solidity: function isVerifierBlacklisted(uint8 verifierIdx) view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) IsVerifierBlacklisted(verifierIdx uint8) (bool, error) {
	return _ContractAlignedLayerServiceManager.Contract.IsVerifierBlacklisted(&_ContractAlignedLayerServiceManager.CallOpts, verifierIdx)
}

// IsVerifierBlacklisted is a free data retrieval call binding the contract method 0x5c9244ca.
//
// Solidity: function isVerifierBlacklisted(uint8 verifierIdx) view returns(bool)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerCallerSession) IsVerifierBlacklisted(verifierIdx uint8) (bool, error) {
	return _ContractAlignedLayerServiceManager.Contract.IsVerifierBlacklisted(&_ContractAlignedLayerServiceManager.CallOpts, verifierIdx)
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

// BlacklistVerifier is a paid mutator transaction binding the contract method 0xc97418c0.
//
// Solidity: function blacklistVerifier(uint8 verifierIdx) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) BlacklistVerifier(opts *bind.TransactOpts, verifierIdx uint8) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "blacklistVerifier", verifierIdx)
}

// BlacklistVerifier is a paid mutator transaction binding the contract method 0xc97418c0.
//
// Solidity: function blacklistVerifier(uint8 verifierIdx) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) BlacklistVerifier(verifierIdx uint8) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.BlacklistVerifier(&_ContractAlignedLayerServiceManager.TransactOpts, verifierIdx)
}

// BlacklistVerifier is a paid mutator transaction binding the contract method 0xc97418c0.
//
// Solidity: function blacklistVerifier(uint8 verifierIdx) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) BlacklistVerifier(verifierIdx uint8) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.BlacklistVerifier(&_ContractAlignedLayerServiceManager.TransactOpts, verifierIdx)
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

// SetVerifiersBlacklist is a paid mutator transaction binding the contract method 0xdb45b1bf.
//
// Solidity: function setVerifiersBlacklist(uint256 bitmap) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) SetVerifiersBlacklist(opts *bind.TransactOpts, bitmap *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "setVerifiersBlacklist", bitmap)
}

// SetVerifiersBlacklist is a paid mutator transaction binding the contract method 0xdb45b1bf.
//
// Solidity: function setVerifiersBlacklist(uint256 bitmap) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) SetVerifiersBlacklist(bitmap *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.SetVerifiersBlacklist(&_ContractAlignedLayerServiceManager.TransactOpts, bitmap)
}

// SetVerifiersBlacklist is a paid mutator transaction binding the contract method 0xdb45b1bf.
//
// Solidity: function setVerifiersBlacklist(uint256 bitmap) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) SetVerifiersBlacklist(bitmap *big.Int) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.SetVerifiersBlacklist(&_ContractAlignedLayerServiceManager.TransactOpts, bitmap)
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

// WhitelistVerifier is a paid mutator transaction binding the contract method 0xa24a6690.
//
// Solidity: function whitelistVerifier(uint8 verifierIdx) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactor) WhitelistVerifier(opts *bind.TransactOpts, verifierIdx uint8) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.contract.Transact(opts, "whitelistVerifier", verifierIdx)
}

// WhitelistVerifier is a paid mutator transaction binding the contract method 0xa24a6690.
//
// Solidity: function whitelistVerifier(uint8 verifierIdx) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerSession) WhitelistVerifier(verifierIdx uint8) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.WhitelistVerifier(&_ContractAlignedLayerServiceManager.TransactOpts, verifierIdx)
}

// WhitelistVerifier is a paid mutator transaction binding the contract method 0xa24a6690.
//
// Solidity: function whitelistVerifier(uint8 verifierIdx) returns()
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerTransactorSession) WhitelistVerifier(verifierIdx uint8) (*types.Transaction, error) {
	return _ContractAlignedLayerServiceManager.Contract.WhitelistVerifier(&_ContractAlignedLayerServiceManager.TransactOpts, verifierIdx)
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

// ContractAlignedLayerServiceManagerVerifierBlacklistedIterator is returned from FilterVerifierBlacklisted and is used to iterate over the raw logs and unpacked data for VerifierBlacklisted events raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerVerifierBlacklistedIterator struct {
	Event *ContractAlignedLayerServiceManagerVerifierBlacklisted // Event containing the contract specifics and raw log

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
func (it *ContractAlignedLayerServiceManagerVerifierBlacklistedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractAlignedLayerServiceManagerVerifierBlacklisted)
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
		it.Event = new(ContractAlignedLayerServiceManagerVerifierBlacklisted)
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
func (it *ContractAlignedLayerServiceManagerVerifierBlacklistedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractAlignedLayerServiceManagerVerifierBlacklistedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractAlignedLayerServiceManagerVerifierBlacklisted represents a VerifierBlacklisted event raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerVerifierBlacklisted struct {
	VerifierIdx uint8
	Raw         types.Log // Blockchain specific contextual infos
}

// FilterVerifierBlacklisted is a free log retrieval operation binding the contract event 0xf7d44cdeceb21bc027e9a0aa16d424d59eaaa76c302f7df60e40d68e60a588e7.
//
// Solidity: event VerifierBlacklisted(uint8 indexed verifierIdx)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) FilterVerifierBlacklisted(opts *bind.FilterOpts, verifierIdx []uint8) (*ContractAlignedLayerServiceManagerVerifierBlacklistedIterator, error) {

	var verifierIdxRule []interface{}
	for _, verifierIdxItem := range verifierIdx {
		verifierIdxRule = append(verifierIdxRule, verifierIdxItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.FilterLogs(opts, "VerifierBlacklisted", verifierIdxRule)
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerVerifierBlacklistedIterator{contract: _ContractAlignedLayerServiceManager.contract, event: "VerifierBlacklisted", logs: logs, sub: sub}, nil
}

// WatchVerifierBlacklisted is a free log subscription operation binding the contract event 0xf7d44cdeceb21bc027e9a0aa16d424d59eaaa76c302f7df60e40d68e60a588e7.
//
// Solidity: event VerifierBlacklisted(uint8 indexed verifierIdx)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) WatchVerifierBlacklisted(opts *bind.WatchOpts, sink chan<- *ContractAlignedLayerServiceManagerVerifierBlacklisted, verifierIdx []uint8) (event.Subscription, error) {

	var verifierIdxRule []interface{}
	for _, verifierIdxItem := range verifierIdx {
		verifierIdxRule = append(verifierIdxRule, verifierIdxItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.WatchLogs(opts, "VerifierBlacklisted", verifierIdxRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractAlignedLayerServiceManagerVerifierBlacklisted)
				if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "VerifierBlacklisted", log); err != nil {
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

// ParseVerifierBlacklisted is a log parse operation binding the contract event 0xf7d44cdeceb21bc027e9a0aa16d424d59eaaa76c302f7df60e40d68e60a588e7.
//
// Solidity: event VerifierBlacklisted(uint8 indexed verifierIdx)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) ParseVerifierBlacklisted(log types.Log) (*ContractAlignedLayerServiceManagerVerifierBlacklisted, error) {
	event := new(ContractAlignedLayerServiceManagerVerifierBlacklisted)
	if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "VerifierBlacklisted", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractAlignedLayerServiceManagerVerifierWhitelistedIterator is returned from FilterVerifierWhitelisted and is used to iterate over the raw logs and unpacked data for VerifierWhitelisted events raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerVerifierWhitelistedIterator struct {
	Event *ContractAlignedLayerServiceManagerVerifierWhitelisted // Event containing the contract specifics and raw log

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
func (it *ContractAlignedLayerServiceManagerVerifierWhitelistedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractAlignedLayerServiceManagerVerifierWhitelisted)
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
		it.Event = new(ContractAlignedLayerServiceManagerVerifierWhitelisted)
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
func (it *ContractAlignedLayerServiceManagerVerifierWhitelistedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractAlignedLayerServiceManagerVerifierWhitelistedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractAlignedLayerServiceManagerVerifierWhitelisted represents a VerifierWhitelisted event raised by the ContractAlignedLayerServiceManager contract.
type ContractAlignedLayerServiceManagerVerifierWhitelisted struct {
	VerifierIdx uint8
	Raw         types.Log // Blockchain specific contextual infos
}

// FilterVerifierWhitelisted is a free log retrieval operation binding the contract event 0x65c7ff2890067393fb22d496e9fb2ecd8bdb4231818f2cdd28794ea5f12e6c3c.
//
// Solidity: event VerifierWhitelisted(uint8 indexed verifierIdx)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) FilterVerifierWhitelisted(opts *bind.FilterOpts, verifierIdx []uint8) (*ContractAlignedLayerServiceManagerVerifierWhitelistedIterator, error) {

	var verifierIdxRule []interface{}
	for _, verifierIdxItem := range verifierIdx {
		verifierIdxRule = append(verifierIdxRule, verifierIdxItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.FilterLogs(opts, "VerifierWhitelisted", verifierIdxRule)
	if err != nil {
		return nil, err
	}
	return &ContractAlignedLayerServiceManagerVerifierWhitelistedIterator{contract: _ContractAlignedLayerServiceManager.contract, event: "VerifierWhitelisted", logs: logs, sub: sub}, nil
}

// WatchVerifierWhitelisted is a free log subscription operation binding the contract event 0x65c7ff2890067393fb22d496e9fb2ecd8bdb4231818f2cdd28794ea5f12e6c3c.
//
// Solidity: event VerifierWhitelisted(uint8 indexed verifierIdx)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) WatchVerifierWhitelisted(opts *bind.WatchOpts, sink chan<- *ContractAlignedLayerServiceManagerVerifierWhitelisted, verifierIdx []uint8) (event.Subscription, error) {

	var verifierIdxRule []interface{}
	for _, verifierIdxItem := range verifierIdx {
		verifierIdxRule = append(verifierIdxRule, verifierIdxItem)
	}

	logs, sub, err := _ContractAlignedLayerServiceManager.contract.WatchLogs(opts, "VerifierWhitelisted", verifierIdxRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractAlignedLayerServiceManagerVerifierWhitelisted)
				if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "VerifierWhitelisted", log); err != nil {
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

// ParseVerifierWhitelisted is a log parse operation binding the contract event 0x65c7ff2890067393fb22d496e9fb2ecd8bdb4231818f2cdd28794ea5f12e6c3c.
//
// Solidity: event VerifierWhitelisted(uint8 indexed verifierIdx)
func (_ContractAlignedLayerServiceManager *ContractAlignedLayerServiceManagerFilterer) ParseVerifierWhitelisted(log types.Log) (*ContractAlignedLayerServiceManagerVerifierWhitelisted, error) {
	event := new(ContractAlignedLayerServiceManagerVerifierWhitelisted)
	if err := _ContractAlignedLayerServiceManager.contract.UnpackLog(event, "VerifierWhitelisted", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
