package common

type ProvingSystemId uint16

const (
	GnarkPlonkBls12_381 ProvingSystemId = iota
	GnarkPlonkBn254
	SP1
	GnarkPlonkBls12_381Batch
)
