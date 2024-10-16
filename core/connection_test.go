package connection_test

import (
	"fmt"
	"testing"

	"github.com/yetanotherco/aligned_layer/core"
)

func DummyFunction(x uint64) (uint64, error) {
	fmt.Println("Doing some work...")
	if x == 42 {
		return 0, connection.PermanentError{Inner: fmt.Errorf("Permanent error!")}
	} else if x < 42 {
		return 0, fmt.Errorf("Transient error!")
	}
	return x, nil
}

func TestRetry(t *testing.T) {
	function := func() (interface{}, error) { return DummyFunction(42) }
	data, err := connection.Retry(function, 1000, 2, 3)
	if err != nil {
		t.Errorf("Retry error!: %s", err)
	} else {
		fmt.Printf("DATA: %d\n", data)
	}
}
