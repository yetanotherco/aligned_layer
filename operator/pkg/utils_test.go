package operator

import (
	"testing"
)

func TestBaseUrlOnlyHappyPath(t *testing.T) {
	// Format "<protocol>://<base_url>/<api_key>"

	urls := [...][2]string{
		{"http://localhost:8545/asdfoij2a7831has89%342jddav98j2748", "localhost:8545"},
		{"ws://test.com/23r2f98hkjva0udhvi1j%342jddav98j2748", "test.com"},
		{"http://localhost:8545", "localhost:8545"},
		{"https://myservice.com/holesky/ApiKey", "myservice.com/holesky"},
	}

	for _, pair := range urls {
		url := pair[0]
		expectedBaseUrl := pair[1]

		baseUrl, err := BaseUrlOnly(url)

		if err != nil {
			t.Errorf("Unexpected error for URL %s: %v", url, err)
		}

		if baseUrl != expectedBaseUrl {
			t.Errorf("Expected base URL %s, got %s for URL %s", expectedBaseUrl, baseUrl, url)
		}
	}
}

func TestBaseUrlOnlyFailureCases(t *testing.T) {

	urls := [...]string{
		"localhost:8545/asdfoij2a7831has89%342jddav98j2748",
		"this-is-all-wrong",
	}

	for _, url := range urls {
		baseUrl, err := BaseUrlOnly(url)

		if err == nil {
			t.Errorf("An error was expected, but received %s", baseUrl)
		}
	}
}
