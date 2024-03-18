package common

type VerifierId uint16

const (
	LambdaworksCairo VerifierId = iota
	GnarkPlonkBls12_381
	Kimchi
	Sp1BabyBearBlake3
)
