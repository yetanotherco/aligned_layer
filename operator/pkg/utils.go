package operator

import (
	"fmt"
	"math/big"
	"regexp"
	"strings"

	"github.com/yetanotherco/aligned_layer/common"
)

func IsVerifierDisabled(disabledVerifiersBitmap *big.Int, verifierId common.ProvingSystemId) bool {
	verifierIdInt := uint8(verifierId)
	// The cast to uint64 is necessary because we need to use the bitwise AND operator.
	// This will truncate the bitmap to 64 bits, but we are not expecting to have more than 63 verifiers.
	// If we set a number that doesn't fit in 64 bits, the bitmap will be truncated and no verifier will be disabled.
	bit := disabledVerifiersBitmap.Uint64() & (1 << verifierIdInt)
	return bit != 0
}

func BaseUrlOnly(input string) (string, error) {
	// Define a regex pattern to match the URL format
	// The pattern captures the scheme, host, and path
	pattern := `^(?P<scheme>[^:]+)://(?P<host>[^/]+)(?P<path>/.*)?$`
	re := regexp.MustCompile(pattern)

	matches := re.FindStringSubmatch(input)

	if matches == nil {
		return "", fmt.Errorf("invalid URL: %s", input)
	}

	host := matches[2]
	path := matches[3]

	// If the path is not empty, append the path without the last segment (api_key)
	if path != "" {
		pathSegments := strings.Split(path, "/")
		if len(pathSegments) > 1 {
			return host + strings.Join(pathSegments[:len(pathSegments)-1], "/"), nil
		}
	}

	return host, nil
}
