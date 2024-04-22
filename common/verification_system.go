package common

type SystemVerificationId uint16

// TODO Set corrects verification systems
const (
	LambdaworksCairo SystemVerificationId = iota
	GnarkPlonkBls12_381
	Kimchi
	Sp1BabyBearBlake3
)
