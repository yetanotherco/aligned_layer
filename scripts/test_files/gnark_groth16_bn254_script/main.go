package main

import (
	"fmt"
	"github.com/consensys/gnark"
	"log"
	"os"
	"strings"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"

	//	"github.com/consensys/gnark/frontend/cs/scs"
	"github.com/consensys/gnark/frontend/cs/r1cs"

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

	outputDir := "scripts/test_files/gnark_groth16_bn254_script/"
	var gnarkVersion = strings.ReplaceAll(gnark.Version.String(), ".", "_")

	var circuit CubicCircuit
	// use r1cs.NewBuilder instead of scs.NewBuilder
	ccs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	if err != nil {
		panic("circuit compilation error")
	}

	// rics is not used in the setup
	//	r1cs := ccs.(*cs.SparseR1CS)
	// as srs is not used in the setup, we can remove it
	//	srs, err := test.NewKZGSRS(r1cs)
	if err != nil {
		panic("KZG setup error")
	}

	// no need to use srs in the setup
	pk, vk, _ := groth16.Setup(ccs)
	//	pk, vk, err := groth16.Setup(ccs, srs)

	assignment := CubicCircuit{X: 3, Y: 35}

	fullWitness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
	if err != nil {
		log.Fatal(err)
	}

	publicWitness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField(), frontend.PublicOnly())
	if err != nil {
		log.Fatal(err)
	}

	// This proof should be serialized for testing in the operator
	proof, err := groth16.Prove(ccs, pk, fullWitness)
	if err != nil {
		panic("GROTH16 proof generation error")
	}

	// The proof is verified before writing it into a file to make sure it is valid.
	err = groth16.Verify(proof, vk, publicWitness)
	if err != nil {
		panic("GROTH16 proof not verified")
	}

	// Open files for writing the proof, the verification key and the public witness
	proofFile, err := os.Create(outputDir + "groth16_" + gnarkVersion + ".proof")
	if err != nil {
		panic(err)
	}
	vkFile, err := os.Create(outputDir + "groth16_" + gnarkVersion + ".vk")
	if err != nil {
		panic(err)
	}
	witnessFile, err := os.Create(outputDir + "groth16_" + gnarkVersion + ".pub")
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

	fmt.Println("Proof written into " + outputDir + "groth16_" + gnarkVersion + ".proof")
	fmt.Println("Verification key written into " + outputDir + "groth16_" + gnarkVersion + ".vk")
	fmt.Println("Public witness written into " + outputDir + "groth16_" + gnarkVersion + ".pub")
}
