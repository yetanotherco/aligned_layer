package common

import (
	"fmt"

	"github.com/fxamacker/cbor"
)

type ProvingSystemId uint16

const (
	GnarkPlonkBls12_381 ProvingSystemId = iota
	GnarkPlonkBn254
	Groth16Bn254
	SP1
	Halo2KZG
	Halo2IPA
	Risc0
)

func (t *ProvingSystemId) String() string {
	return [...]string{"GnarkPlonkBls12_381", "GnarkPlonkBn254", "Groth16Bn254", "SP1", "Halo2IPA"}[*t]
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
	case "Halo2KZG":
		return Halo2KZG, nil
	case "Halo2IPA":
		return Halo2IPA, nil
	case "Risc0":
		return Risc0, nil
	}

	return 0, fmt.Errorf("unknown proving system: %s", provingSystem)
}

func (t *ProvingSystemId) UnmarshalCBOR(b []byte) error {
	var s string
	err := cbor.Unmarshal(b, &s)
	if err != nil {
		return err
	}
	*t, err = ProvingSystemIdFromString(s)
	return err
}
