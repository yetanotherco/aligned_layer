package types

import (
	"fmt"
	"math/big"
)

type OperatorData struct {
	Address string // Store the address as a string instead of a gethcommon.Address to avoid encoding/decoding
	Id      string // Store the id as a string instead of a eigentypes.OperatorId to avoid encoding/decoding
	Name    string
	Stake   *big.Int
}

// Print prints the OperatorData struct
func (o *OperatorData) String() string {
	return fmt.Sprintf("OperatorData{Address: %s, Id: %s, Name: %s, Stake: %d}", o.Address, o.Id, o.Name, o.Stake)
}
