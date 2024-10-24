package connection_test

import (
	"fmt"
	"testing"

	connection "github.com/yetanotherco/aligned_layer/core"
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

// here we run the dummy function with the retry and check:
// - The number of retries checks based on the `n`
// - The returned valued matches based on the `n`
// - The returned err matches based on the `n`
func TestRetryWithData(t *testing.T) {
	retries := -1
	testFun := func(n uint64) func() (*uint64, error) {
		return func() (*uint64, error) {
			retries++
			x, err := DummyFunction(n)
			return &x, err
		}
	}
	data, err := connection.RetryWithData(testFun(uint64(41)), 1000, 2, 3)
	if !(retries == 3 && *data == 0 && err != nil) {
		t.Error("Incorrect execution when n == 41")
	}
	//restart
	retries = -1
	data, err = connection.RetryWithData(testFun(42), 1000, 2, 3)
	if !(retries == 0 && data == nil) {
		if _, ok := err.(*connection.PermanentError); ok {
			t.Error("Incorrect execution when n == 42")
		}
	}
	//restart
	retries = -1
	data, err = connection.RetryWithData(testFun(43), 1000, 2, 3)
	if !(retries == 0 && *data == 43 && err == nil) {
		t.Error("Incorrect execution when n == 43")
	}
}

// same as above but without checking returned data
func TestRetry(t *testing.T) {
	retries := -1
	testFun := func(n uint64) func() error {
		return func() error {
			retries++
			_, err := DummyFunction(n)
			return err
		}
	}
	err := connection.Retry(testFun(uint64(41)), 1000, 2, 3)
	if !(retries == 3 && err != nil) {
		t.Error("Incorrect execution when n == 41")
	}
	//restart
	retries = -1
	err = connection.Retry(testFun(42), 1000, 2, 3)
	if !(retries == 0) {
		if _, ok := err.(*connection.PermanentError); ok {
			t.Error("Incorrect execution when n == 42")
		}
	}
	//restart
	retries = -1
	err = connection.Retry(testFun(43), 1000, 2, 3)
	if !(retries == 0 && err == nil) {
		t.Error("Incorrect execution when n == 43")
	}
}
