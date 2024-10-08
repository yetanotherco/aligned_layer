package operator_test

import (
	"context"
	"crypto/ecdsa"
	"crypto/rand"
	"fmt"
	"math/big"
	"net/http"
	"sync"
	"testing"
	"time"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	gethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/params"
	contractAlignedLayerServiceManager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"golang.org/x/net/http2"
)

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

func NewAnvilClient(t *testing.T) *ethclient.Client {
	client, err := ethclient.Dial("http://localhost:8545")
	if err != nil {
		t.Fatalf("could not connect to anvil: %s", err)
	}
	return client
}

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

func CreateNewTask(t *testing.T, user *AlignedUser, contractAddress common.Address, merkleRoot [32]byte, dataPointer string) {
	serviceManager, err := contractAlignedLayerServiceManager.NewContractAlignedLayerServiceManager(
		contractAddress,
		user.Client,
	)
	if err != nil {
		t.Fatalf("could not create service manager: %s", err)
	}
	t.Logf("ServiceManager created")

	DepositToUser(t, user, serviceManager)

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
		big.NewInt(1510000000000000),
	)
	if err != nil {
		t.Fatalf("could not create task: %s", err)
	}

	t.Logf("New task created")

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
		t.Logf("Receipt Status: %v", r.Status)
		if r.Status != 0 {
			break
		}
		time.Sleep(1 * time.Second)
	}

}

func DepositToUser(t *testing.T, user *AlignedUser, serviceManager *contractAlignedLayerServiceManager.ContractAlignedLayerServiceManager) {
	depositTx, err := serviceManager.DepositToBatcher(
		&bind.TransactOpts{
			From:  user.UserAddress,
			Nonce: user.getNonce(),
			Signer: func(addr common.Address, tx *gethtypes.Transaction) (*gethtypes.Transaction, error) {
				return gethtypes.SignTx(tx, user.Signer, user.PrivateKey)
			},
			Value:    new(big.Int).SetUint64(2000000000000000),
			GasLimit: params.GenesisGasLimit / 2,
		},
		user.UserAddress,
	)
	if err != nil {
		t.Fatalf("could not create task: %s", err)
	}

	t.Logf("New task created")

	i := 0
	r := new(gethtypes.Receipt)
	for {
		r, err = user.Client.TransactionReceipt(context.TODO(), depositTx.Hash())
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
		t.Logf("Receipt Status: %v", r.Status)
		if r.Status != 0 {
			break
		}
		time.Sleep(1 * time.Second)
	}

}

func processGzipBomb(t *testing.T) {
	client := NewAnvilClient(t)
	alice := NewAlignedUser(t, alicePrivateKey, client, aliceAddress, "alice")
	t.Logf("New user %v message", alice)

	var randHash [32]byte
	if _, err := rand.Read(randHash[:]); err != nil {
		t.Fatalf("could not generate random hash: %s", err)
	}

	CreateNewTask(t, alice, alignedLayerServiceManagerAddress, randHash, fmt.Sprintf("http://localhost:1515/%x", randHash))
}

func startTestServerOOM(t *testing.T, wg *sync.WaitGroup) {
	wg.Add(1)
	defer wg.Done()

	server := &http.Server{
		Addr: ":1515",
		Handler: http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			handleRequestOOM(t, w, r)
		}),
	}

	if err := http2.ConfigureServer(server, &http2.Server{}); err != nil {
		t.Fatalf("could not configure server for http2: %s", err)
	}
	t.Logf("Starting the server")

	t.Fatal(server.ListenAndServe())
}

func handleRequestOOM(t *testing.T, w http.ResponseWriter, r *http.Request) {
	t.Logf("Received request: %s %s", r.Method, r.URL)
	switch r.Method {
	case http.MethodHead:
	case http.MethodGet:
		w.Header().Set("Content-Encoding", "gzip")
		for {
			if _, err := w.Write([]byte("infinite content")); err != nil {
				t.Logf("Finishing test")
				return
			}
			w.(http.Flusher).Flush()
		}
	default:
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
	}
}

func TestGetBatchExplorerOOM(t *testing.T) {
	wg := sync.WaitGroup{}
	go startTestServerOOM(t, &wg)
	processGzipBomb(t)
	wg.Wait()
}
