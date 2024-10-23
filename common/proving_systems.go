package common

import (
	"encoding/json"
	"fmt"
	"log"

	"github.com/fxamacker/cbor/v2"
)

type ProvingSystemId uint16

const (
	GnarkPlonkBls12_381 ProvingSystemId = iota
	GnarkPlonkBn254
	Groth16Bn254
	SP1
	Risc0
	Mina
	MinaAccount
)

func (t *ProvingSystemId) String() string {
	return [...]string{"GnarkPlonkBls12_381", "GnarkPlonkBn254", "Groth16Bn254", "SP1", "Mina", "MinaAccount"}[*t]
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
	case "Risc0":
		return Risc0, nil
	case "Mina":
		return Mina, nil
	case "MinaAccount":
		return MinaAccount, nil
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
	case Risc0:
		return "Risc0", nil
	case Mina:
		return "Mina", nil
	case MinaAccount:
		return "MinaAccount", nil
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

func (t *ProvingSystemId) UnmarshalBinary(data []byte) error {
	// get string from bytes
	str := string(data[:])
	log.Printf("ProvingSystemId.UnmarshalBinary: %s\n", str)

	// get enum from string
	var err error
	*t, err = ProvingSystemIdFromString(str)

	return err
}

func (s *ProvingSystemId) UnmarshalCBOR(data []byte) error {
	var statusStr string
	if err := cbor.Unmarshal(data, &statusStr); err != nil {
		return err
	}

	switch statusStr {
	case "GnarkPlonkBls12_381":
		*s = GnarkPlonkBls12_381
	case "GnarkPlonkBn254":
		*s = GnarkPlonkBn254
	case "Groth16Bn254":
		*s = Groth16Bn254
	case "SP1":
		*s = SP1
	case "Risc0":
		*s = Risc0
	case "Mina":
		*s = Mina
	case "MinaAccount":
		*s = MinaAccount
	}

	return nil
}

func (t ProvingSystemId) MarshalBinary() ([]byte, error) {
	// needs to be defined but should never be called
	return nil, fmt.Errorf("not implemented")
}
