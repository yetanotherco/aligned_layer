package chainio_test

import (
	"math/big"
	"reflect"
	"testing"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
)

// NOTE(marian): Just some dummy tests, should be reworked later
// Make sure to have the anvil blockchain running in the background with the
// loaded state for running the tests.

const ConfigPath = "../../config-files/config-test.yaml"

func TestAvsReader(t *testing.T) {
	baseConfig := config.NewBaseConfig(ConfigPath)
	ecdsaConfig := config.NewEcdsaConfig(ConfigPath, baseConfig.ChainId)

	avsReader, err := chainio.NewAvsReaderFromConfig(baseConfig, ecdsaConfig)
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
	baseConfig := config.NewBaseConfig(ConfigPath)
	ecdsaConfig := config.NewEcdsaConfig(ConfigPath, baseConfig.ChainId)

	avsWriter, err := chainio.NewAvsWriterFromConfig(baseConfig, ecdsaConfig)
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
	baseConfig := config.NewBaseConfig(ConfigPath)

	avsSubscriber, err := chainio.NewAvsSubscriberFromConfig(baseConfig)
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
