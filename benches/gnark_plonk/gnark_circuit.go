package main

import (
	"fmt"
	"log"
	"os"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/plonk"
	cs "github.com/consensys/gnark/constraint/bls12-381"
	"github.com/consensys/gnark/frontend/cs/scs"
	"github.com/consensys/gnark/test"

	"github.com/consensys/gnark/frontend"
)

// CubicCircuit defines a simple circuit
// x**3 + x + 5 == y
type CubicCircuit struct {
	// struct tags on a variable is optional
	// default uses variable name and secret visibility.
	X frontend.Variable `gnark:"x"`
	Y frontend.Variable `gnark:",public"`
}

// Define declares the circuit constraints
// x**3 + x + 5 == y
func (circuit *CubicCircuit) Define(api frontend.API) error {
	x3 := api.Mul(circuit.X, circuit.X, circuit.X)
	api.AssertIsEqual(circuit.Y, api.Add(x3, circuit.X, 5))
	return nil
}

func main() {
	var circuit CubicCircuit
	ccs, err := frontend.Compile(ecc.BLS12_381.ScalarField(), scs.NewBuilder, &circuit)
	if err != nil {
		panic("circuit compilation error")
	}

	r1cs := ccs.(*cs.SparseR1CS)
	srs, err := test.NewKZGSRS(r1cs)
	if err != nil {
		panic("KZG setup error")
	}

	pk, vk, err := plonk.Setup(ccs, srs)

	assignment := CubicCircuit{X: 3, Y: 35}

	fullWitness, err := frontend.NewWitness(&assignment, ecc.BLS12_381.ScalarField())
	if err != nil {
		log.Fatal(err)
	}

	publicWitness, err := frontend.NewWitness(&assignment, ecc.BLS12_381.ScalarField(), frontend.PublicOnly())
	if err != nil {
		log.Fatal(err)
	}

	// This proof should be serialized for testing in the operator
	proof, err := plonk.Prove(ccs, pk, fullWitness)
	if err != nil {
		panic("PLONK proof generation error")
	}

	// The proof is verified before writing it into a file to make sure it is valid.
	err = plonk.Verify(proof, vk, publicWitness)
	if err != nil {
		panic("PLONK proof not verified")
	}

	// Open files for writing the proof, the verification key and the public witness
	proofFile, err := os.Create("plonk_cubic_circuit.proof")
	if err != nil {
		panic(err)
	}
	vkFile, err := os.Create("plonk_verification_key")
	if err != nil {
		panic(err)
	}
	witnessFile, err := os.Create("witness.pub")
	if err != nil {
		panic(err)
	}
	defer proofFile.Close()
	defer vkFile.Close()
	defer witnessFile.Close()

	_, err = proof.WriteTo(proofFile)
	if err != nil {
		panic("could not serialize proof into file")
	}
	_, err = vk.WriteTo(vkFile)
	if err != nil {
		panic("could not serialize verification key into file")
	}
	_, err = publicWitness.WriteTo(witnessFile)
	if err != nil {
		panic("could not serialize proof into file")
	}

	fmt.Println("Proof written into plonk_cubic_circuit.proof")
	fmt.Println("Verification key written into plonk_verification_key")
	fmt.Println("Public witness written into witness.pub")
}
