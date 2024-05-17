package common

import "encoding/json"

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

func (t *ProvingSystemId) FromString(provingSystem string) ProvingSystemId {
	return map[string]ProvingSystemId{
		"GnarkPlonkBls12_381": GnarkPlonkBls12_381,
		"GnarkPlonkBn254":     GnarkPlonkBn254,
		"Groth16Bn254":        Groth16Bn254,
		"SP1":                 SP1,
	}[provingSystem]
}

func (t *ProvingSystemId) UnmarshalJSON(b []byte) error {
	var s string
	err := json.Unmarshal(b, &s)
	if err != nil {
		return err
	}
	*t = t.FromString(s)
	return nil
}
