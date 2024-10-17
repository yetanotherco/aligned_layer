package connection

import (
	"time"

	"github.com/cenkalti/backoff/v4"
)

type PermanentError struct {
	Inner error
}

func (e PermanentError) Error() string { return e.Inner.Error() }
func (e PermanentError) Unwrap() error {
	return e.Inner
}
func (e PermanentError) Is(err error) bool {
	_, ok := err.(PermanentError)
	return ok
}

// Retries a given function in an exponential backoff manner.
// It will retry calling the function while it returns an error, until the max retries
// from the configuration are reached, or until a `PermanentError` is returned.
// The function to be retried should return `PermanentError` when the condition for stop retrying
// is met.
func Retry(functionToRetry func() (interface{}, error), minDelay uint64, factor float64, maxTries uint64) (interface{}, error) {
	f := func() (interface{}, error) {
		val, err := functionToRetry()
		if perm, ok := err.(PermanentError); err != nil && ok {
			return nil, backoff.Permanent(perm.Inner)
		}
		return val, err
	}

	randomOption := backoff.WithRandomizationFactor(0)

	initialRetryOption := backoff.WithInitialInterval(time.Millisecond * time.Duration(minDelay))
	multiplierOption := backoff.WithMultiplier(factor)
	expBackoff := backoff.NewExponentialBackOff(randomOption, multiplierOption, initialRetryOption)
	maxRetriesBackoff := backoff.WithMaxRetries(expBackoff, maxTries)

	return backoff.RetryWithData(f, maxRetriesBackoff)
}
