package operator

// Test for the function IsVerifierDisabled

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
				t.Errorf("Verifier %s is disable but it should not", verifierId.String())
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
				t.Errorf("Verifier %s is enabled but it should not", verifierId.String())
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
				t.Errorf("Verifier %s is enabled but it should not", verifierId.String())
			}
		}
	})
}
