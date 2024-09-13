package types

import (
	"fmt"
	eigentypes "github.com/Layr-Labs/eigensdk-go/types"
	gethcommon "github.com/ethereum/go-ethereum/common"
	"math/big"
)

type OperatorData struct {
	Address       gethcommon.Address
	Id            eigentypes.OperatorId
	AddressString string // Store the address as a string to avoid encoding/decoding
	IdString      string // Store the id as a string to avoid encoding/decoding
	Name          string
	Stake         *big.Int
}

// Print prints the OperatorData struct
func (o *OperatorData) String() string {
	return fmt.Sprintf("OperatorData{Address: %s, Id: %s, Name: %s, Stake: %d}", o.AddressString, o.IdString, o.Name, o.Stake)
}
