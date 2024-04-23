package chainio_test

import (
	"math/big"
	"reflect"
	"testing"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/yetanotherco/aligned_layer/core/chainio"
)

// NOTE(marian): Just some dummy tests, should be reworked later
// Make sure to have the anvil blockchain running in the background with the
// loaded state for running the tests.

func TestAvsReader(t *testing.T) {
	avsReader, err := chainio.NewAvsReaderFromConfig()
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
	avsWriter, err := chainio.NewAvsWriterFromConfig()
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
	avsSubscriber, err := chainio.NewAvsSubscriberFromConfig()
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
