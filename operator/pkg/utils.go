package operator

import (
	"math/big"

	"github.com/yetanotherco/aligned_layer/common"
)

func IsVerifierDisabled(disabledVerifiersBitmap *big.Int, verifierId common.ProvingSystemId) bool {
	verifierIdInt := uint8(verifierId)
	bit := disabledVerifiersBitmap.Uint64() & (1 << verifierIdInt)
	return bit != 0
}
