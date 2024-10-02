package actions

import (
  "regexp" 
  "errors"
)

func baseUrlOnly(url string) (string, error)  {
  // Removes the protocol and api key part from any url formated like so:
  // "<protocol>://<base_url>/<api_key>"
  regex := regexp.MustCompile(`^[a-z]+://([^/]+)`)
  match := regex.FindStringSubmatch(url)
  if len(match) > 1 {
    return match[1], nil
  }
  return "", errors.New("Url did not match the expected format <protocol>://<base_url>/<api_key>")
}
