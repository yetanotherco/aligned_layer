package chainio_test

import (
	"math/big"
	"reflect"
	"testing"

	"github.com/yetanotherco/aligned_layer/core/tests/mocks"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/yetanotherco/aligned_layer/core/chainio"
)

// NOTE(marian): Just some dummy tests, should be reworked later
// Make sure to have the anvil blockchain running in the background with the
// loaded state for running the tests.

func TestAvsReader(t *testing.T) {
	mockConfig := mocks.NewMockConfig("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80", "0x9d4454b023096f34b160d6b654540c56a1f81688", "0xc5a5c42992decbae36851359345fe25997f5c42d")

	avsReader, err := chainio.NewAvsReaderFromConfig(mockConfig)
	if err != nil {
		t.Errorf("could not create AVS reader")
	}

	readerMeaning, err := avsReader.AvsContractBindings.ServiceManager.GetMeaning(&bind.CallOpts{})
	if err != nil {
		t.Errorf("could not call GetMeaning contract function")
	}

	if !reflect.DeepEqual(readerMeaning, big.NewInt(42)) {
		t.Errorf("expected 42, got %d", readerMeaning)
	}
}

func TestAvsWriter(t *testing.T) {
	mockConfig := mocks.NewMockConfig("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80", "0x9d4454b023096f34b160d6b654540c56a1f81688", "0xc5a5c42992decbae36851359345fe25997f5c42d")

	avsWriter, err := chainio.NewAvsWriterFromConfig(mockConfig)
	if err != nil {
		t.Errorf("could not create AVS reader")
	}

	writerMeaning, err := avsWriter.AvsContractBindings.ServiceManager.GetMeaning(&bind.CallOpts{})
	if err != nil {
		t.Errorf("could not call GetMeaning contract function")
	}

	if !reflect.DeepEqual(writerMeaning, big.NewInt(42)) {
		t.Errorf("expected 42, got %d", writerMeaning)
	}
}

func TestAvsSubscriber(t *testing.T) {
	mockConfig := mocks.NewMockConfig("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80", "0x9d4454b023096f34b160d6b654540c56a1f81688", "0xc5a5c42992decbae36851359345fe25997f5c42d")

	avsSubscriber, err := chainio.NewAvsSubscriberFromConfig(mockConfig)
	if err != nil {
		t.Errorf("could not create AVS reader")
	}

	subscriberMeaning, err := avsSubscriber.AvsContractBindings.ServiceManager.GetMeaning(&bind.CallOpts{})
	if err != nil {
		t.Errorf("could not call GetMeaning contract function")
	}

	if !reflect.DeepEqual(subscriberMeaning, big.NewInt(42)) {
		t.Errorf("expected 42, got %d", subscriberMeaning)
	}
}
