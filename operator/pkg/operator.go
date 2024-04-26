package pkg

import (
	"crypto/ecdsa"
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	"github.com/Layr-Labs/eigensdk-go/types"
	"time"
)

type Operator struct {
	Address    string
	Socket     string
	Timeout    time.Duration
	PrivKey    *ecdsa.PrivateKey
	KeyPair    *bls.KeyPair
	OperatorId types.OperatorId
}
