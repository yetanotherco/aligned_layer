package common

type ProvingSystemId uint16

const (
	GnarkPlonkBls12_381 ProvingSystemId = iota
	GnarkPlonkBn254
	Groth16Bn254
	SP1
	SP1Groth16
)
