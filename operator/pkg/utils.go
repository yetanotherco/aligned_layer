package operator

import (
	"fmt"
	"regexp"
	"strings"
)

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
