package actions

import "strings"

func urlRemoveProtocol(url string) string {
  // Removes the protocol part from any url formated like so:
  // "<protocol>://<base_url>"
	parts := strings.SplitN(url, "://", 2)
	if len(parts) > 1 {
		return parts[1]
	}

  return url
}
