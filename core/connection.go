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

// Same as Retry only that the functionToRetry can return a value upon correct execution
func RetryWithData[T any](functionToRetry func() (*T, error), minDelay uint64, factor float64, maxTries uint64) (*T, error) {
	i := 0
	f := func() (*T, error) {
		val, err := functionToRetry()
		i++
		if perm, ok := err.(PermanentError); err != nil && ok {
			return nil, backoff.Permanent(perm.Inner)
		}
		return val, err
	}

	randomOption := backoff.WithRandomizationFactor(0)

	initialRetryOption := backoff.WithInitialInterval(time.Millisecond * time.Duration(minDelay))
	multiplierOption := backoff.WithMultiplier(factor)
	expBackoff := backoff.NewExponentialBackOff(randomOption, multiplierOption, initialRetryOption)
	var maxRetriesBackoff backoff.BackOff

	if maxTries > 0 {
		maxRetriesBackoff = backoff.WithMaxRetries(expBackoff, maxTries)
	} else {
		maxRetriesBackoff = expBackoff
	}

	return backoff.RetryWithData(f, maxRetriesBackoff)
}

// Retries a given function in an exponential backoff manner.
// It will retry calling the function while it returns an error, until the max retries.
// If maxTries == 0 then the retry function will run indefinitely until success
// from the configuration are reached, or until a `PermanentError` is returned.
// The function to be retried should return `PermanentError` when the condition for stop retrying
// is met.
func Retry(functionToRetry func() error, minDelay uint64, factor float64, maxTries uint64) error {
	i := 0
	f := func() error {
		err := functionToRetry()
		i++
		if perm, ok := err.(PermanentError); err != nil && ok {
			return backoff.Permanent(perm.Inner)
		}
		return err
	}

	randomOption := backoff.WithRandomizationFactor(0)

	initialRetryOption := backoff.WithInitialInterval(time.Millisecond * time.Duration(minDelay))
	multiplierOption := backoff.WithMultiplier(factor)
	expBackoff := backoff.NewExponentialBackOff(randomOption, multiplierOption, initialRetryOption)
	maxRetriesBackoff := backoff.WithMaxRetries(expBackoff, maxTries)

	return backoff.Retry(f, maxRetriesBackoff)
}
