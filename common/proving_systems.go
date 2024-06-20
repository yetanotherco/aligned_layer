package common

import (
	"encoding/json"
	"fmt"
)

type ProvingSystemId uint16

const (
	GnarkPlonkBls12_381 ProvingSystemId = iota
	GnarkPlonkBn254
	Groth16Bn254
	SP1
	Jolt
	Halo2KZG
	Halo2IPA
)

func (t *ProvingSystemId) String() string {
	return [...]string{"GnarkPlonkBls12_381", "GnarkPlonkBn254", "Groth16Bn254", "SP1", "Halo2IPA", "Jolt"}[*t]
}

func ProvingSystemIdFromString(provingSystem string) (ProvingSystemId, error) {
	switch provingSystem {
	case "GnarkPlonkBls12_381":
		return GnarkPlonkBls12_381, nil
	case "GnarkPlonkBn254":
		return GnarkPlonkBn254, nil
	case "Groth16Bn254":
		return Groth16Bn254, nil
	case "SP1":
		return SP1, nil
	case "Jolt":
		return Jolt, nil
	case "Halo2KZG":
		return Halo2KZG, nil
	case "Halo2IPA":
		return Halo2IPA, nil
	}

	return 0, fmt.Errorf("unknown proving system: %s", provingSystem)
}

func ProvingSystemIdToString(provingSystem ProvingSystemId) (string, error) {
	switch provingSystem {
	case GnarkPlonkBls12_381:
		return "GnarkPlonkBls12_381", nil
	case GnarkPlonkBn254:
		return "GnarkPlonkBn254", nil
	case Groth16Bn254:
		return "Groth16Bn254", nil
	case SP1:
		return "SP1", nil
	case Jolt:
		return "Jolt", nil
	case Halo2KZG:
		return "Halo2KZG", nil
	case Halo2IPA:
		return "Halo2IPA", nil
	}

	return "", fmt.Errorf("unknown proving system: %d", provingSystem)
}

func (t *ProvingSystemId) UnmarshalJSON(b []byte) error {
	var s string
	err := json.Unmarshal(b, &s)
	if err != nil {
		return err
	}
	*t, err = ProvingSystemIdFromString(s)
	return err
}

func (t ProvingSystemId) MarshalJSON() ([]byte, error) {
	// Check if the enum value has a corresponding string representation
	if str, ret := ProvingSystemIdToString(t); ret == nil {
		// If yes, marshal the string representation
		return json.Marshal(str)
	}
	// If not, return an error
	return nil, fmt.Errorf("invalid ProvingSystemId value: %d", t)
}
