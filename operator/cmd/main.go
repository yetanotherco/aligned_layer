package main

import (
	"fmt"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/yetanotherco/aligned_layer/core/chainio"
)

func main() {
	fmt.Println("Booting operator ...")

	// NOTE(marian): This could be used as a test -->
	avsReader, err := chainio.NewAvsReaderFromConfig()
	if err != nil {
		panic(err)
	}

	avsWriter, err := chainio.NewAvsWriterFromConfig()
	if err != nil {
		panic(err)
	}

	AvsSubscriber, err := chainio.NewAvsSubscriberFromConfig()
	if err != nil {
		panic(err)
	}

	readerMeaning, _ := avsReader.AvsServiceBindings.ServiceManager.GetMeaning(&bind.CallOpts{})
	writerMeaning, _ := avsWriter.AvsContractBindings.ServiceManager.GetMeaning(&bind.CallOpts{})
	subscriberMeaning, _ := AvsSubscriber.AvsContractBindings.ServiceManager.GetMeaning(&bind.CallOpts{})

	if err != nil {
		fmt.Println(err)
		panic("Could not create clients: ")
	}

	fmt.Println("THE MEANING (READER) IS: ", readerMeaning)
	fmt.Println("THE MEANING (WRITER) IS: ", writerMeaning)
	fmt.Println("THE MEANING (SUBSCRIBER) IS: ", subscriberMeaning)
}
