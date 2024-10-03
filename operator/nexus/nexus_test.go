package nexus_test

import (
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/nexus"
)

const ProofPath = "../../scripts/test_files/nexus/nexus-proof"

// NOTE: These are generate after calling `cargo nexus prove` and stored in
// `../../scripts/test_files/nexus/target/nexus-cache/nexus-public-nova-seq-16.zst`
const ParamsPath = "../../scripts/test_files/nexus/nexus-public-nova-seq-16.zst"

func TestNexusProofVerifies(t *testing.T) {
	proofBytes, err := os.ReadFile(ProofPath)
	if err != nil {
		t.Errorf("could not open proof file: %s", err)
	}

	paramsBytes, err := os.ReadFile(ParamsPath)
	if err != nil {
		t.Errorf("could not open elf file: %s", err)
	}

	if !nexus.VerifyNexusProof(proofBytes, paramsBytes) {
		t.Errorf("proof did not verify")
	}
}
