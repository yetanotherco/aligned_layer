package operator

import (
	"math/big"
	"testing"

	"github.com/yetanotherco/aligned_layer/common"
)

func TestIsVerifierDisabled(t *testing.T) {
	t.Run("All verifiers are enabled", func(t *testing.T) {
		disabledVerifiersBitmap := big.NewInt(0)
		proving_systems := []common.ProvingSystemId{common.GnarkPlonkBls12_381, common.GnarkPlonkBn254, common.Groth16Bn254, common.SP1, common.Risc0}
		for _, verifierId := range proving_systems {
			got := IsVerifierDisabled(disabledVerifiersBitmap, verifierId)
			want := false

			if got != want {
				t.Errorf("Verifier %s is disabled but it shouldn't be", verifierId.String())
			}
		}
	})

	t.Run("All verifiers are disabled", func(t *testing.T) {
		// This is the bitmap for all verifiers disabled since it is 11111 in binary.
		disabledVerifiersBitmap := big.NewInt(31)
		proving_systems := []common.ProvingSystemId{common.GnarkPlonkBls12_381, common.GnarkPlonkBn254, common.Groth16Bn254, common.SP1, common.Risc0}
		for _, verifierId := range proving_systems {
			got := IsVerifierDisabled(disabledVerifiersBitmap, verifierId)
			want := true

			if got != want {
				t.Errorf("Verifier %s is enabled but it shouldn't be", verifierId.String())
			}
		}
	})

	t.Run("Some verifiers are disabled", func(t *testing.T) {
		// This is the bitmap for the first and last verifiers disabled since it is 10001 in binary.
		disabledVerifiersBitmap := big.NewInt(17)
		proving_systems := []common.ProvingSystemId{common.GnarkPlonkBls12_381, common.GnarkPlonkBn254, common.Groth16Bn254, common.SP1, common.Risc0}
		for _, verifierId := range proving_systems {
			got := IsVerifierDisabled(disabledVerifiersBitmap, verifierId)
			want := verifierId == common.GnarkPlonkBls12_381 || verifierId == common.Risc0

			if got != want {
				t.Errorf("Verifier %s is enabled but it shouldn't be", verifierId.String())
			}
		}
	})
}

func TestBaseUrlOnlyHappyPath(t *testing.T) {
	// Format "<protocol>://<base_url>/<api_key>"

	urls := [...][2]string{
		{"http://localhost:8545/asdfoij2a7831has89%342jddav98j2748", "localhost:8545"},
		{"ws://test.com/23r2f98hkjva0udhvi1j%342jddav98j2748", "test.com"},
		{"http://localhost:8545", "localhost:8545"},
		{"https://myservice.com/holesky/ApiKey", "myservice.com"},
		{"https://holesky.myservice.com/holesky", "holesky.myservice.com"},
		{"https://eth-mainnet.blastapi.io/12345678-abcd-1234-abcd-123456789012", "eth-mainnet.blastapi.io"},
		{"https://eth-holesky.g.alchemy.com/v2/1234567890_abcdefghijklmnopqrstuv/", "eth-holesky.g.alchemy.com"},
		{"https://a.b.c.d/1234", "a.b.c.d"},
		{"https://a.b.c.d/1234/5678", "a.b.c.d"},
		{"https://a.b.c.d.e/1234/", "a.b.c.d.e"},
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
