package main

import (
	"fmt"
	"math/rand"
	"time"

	"github.com/avast/retry-go"
)

func main() {
	// * ---------------------------------------------------------------------------------------- *
	// *                          DEFINE THE ACTION TO BE RETRIED                                 *
	// * ---------------------------------------------------------------------------------------- *

	// In the case of Aligned, this would be whatever messaging function we would like
	// to be retried in case of failure.
	// This function will return two different type of errors: recoverable and not recoverable.
	// For the case of a recoverable error, the `Retry` function will keep trying until some of the
	// stop conditions is met.
	// For the not recoverable case, the `Retry` function will return without retrying again.
	// This behavior is simulated here with some randomness.
	// The not recoverable error is simulated by the ``PermanentError`
	action := func() error {
		fmt.Println("Doing some operation...")
		fmt.Printf("Actual time: %v\n", time.Now())

		randomNum := rand.Intn(10)
		if randomNum > 5 {
			return retry.Unrecoverable(fmt.Errorf("Non recoverable error"))
		}

		return fmt.Errorf("There was an error!")
	}

	// * ---------------------------------------------------------------------------------------- *
	// *                         EXPONENTIAL BACKOFF CONFIGURATION                                *
	// * ---------------------------------------------------------------------------------------- *
	// For the exponential backoff formula `backoff(n) = a * b^n`, we set the following config:
	// *    a = 2000ms
	// * 	b = 2
	// *    0 <= n <= 2
	// There is no randomization factor

	// marcos: This library does not implement a multiplier, but it shifts based on `n`, so it is using b = 2 under the hood
	delay := retry.Delay(time.Millisecond * 2000)
	delayFn := retry.DelayType(retry.BackOffDelay)
	attempts := retry.Attempts(3)

	// * ---------------------------------------------------------------------------------------- *
	// *                               RETRY FUNCTION CALL                                        *
	// * ---------------------------------------------------------------------------------------- *
	err := retry.Do(action, delay, delayFn, attempts)
	if err != nil {
		fmt.Println(err)
		return
	}
}
