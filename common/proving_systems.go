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
)

func (t *ProvingSystemId) String() string {
	return [...]string{"GnarkPlonkBls12_381", "GnarkPlonkBn254", "Groth16Bn254", "SP1"}[*t]
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
	}

	return 0, fmt.Errorf("Unknown proving system: %s", provingSystem)
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
