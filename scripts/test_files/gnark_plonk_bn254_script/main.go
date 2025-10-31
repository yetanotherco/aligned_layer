package main

import (
	"fmt"
	"github.com/consensys/gnark"
	"log"
	"os"
	"strings"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/plonk"
	cs "github.com/consensys/gnark/constraint/bn254"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/test/unsafekzg"

	"github.com/consensys/gnark/frontend/cs/scs"
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

	outputDir := "scripts/test_files/gnark_plonk_bn254_script/"
	var gnarkVersion = strings.ReplaceAll(gnark.Version.String(), ".", "_")

	var circuit CubicCircuit
	// use scs.NewBuilder instead of r1cs.NewBuilder (groth16)
	ccs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
	if err != nil {
		panic("circuit compilation error")
	}

	// use unsafekzg.NewSRS to generate the SRS and the Lagrange interpolation of the SRS
	// Setup prepares the public data associated to a circuit + public inputs.
	// The kzg SRS must be provided in canonical and lagrange form.
	// For test purposes, see test/unsafekzg package. With an existing SRS generated through MPC in canonical form,

	r1cs := ccs.(*cs.SparseR1CS)
	srs, srsLagrangeInterpolation, err := unsafekzg.NewSRS(r1cs)
	// srs, err := test.NewKZGSRS(r1cs)
	if err != nil {
		panic("KZG setup error")
	}
	// add srsLagrangeInterpolation to the Setup function
	pk, vk, _ := plonk.Setup(ccs, srs, srsLagrangeInterpolation)

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
	proofFile, err := os.Create(outputDir + "gnark_plonk_" + gnarkVersion + ".proof")
	if err != nil {
		panic(err)
	}
	vkFile, err := os.Create(outputDir + "gnark_plonk_" + gnarkVersion + ".vk")
	if err != nil {
		panic(err)
	}
	witnessFile, err := os.Create(outputDir + "gnark_plonk_pub_input_" + gnarkVersion + ".pub")
	if err != nil {
		panic(err)
	}
	defer func(proofFile *os.File) {
		err := proofFile.Close()
		if err != nil {
			log.Fatal("could not close proof file:", err)
		}
	}(proofFile)
	defer func(vkFile *os.File) {
		err := vkFile.Close()
		if err != nil {
			log.Fatal("could not close verification key file:", err)
		}
	}(vkFile)
	defer func(witnessFile *os.File) {
		err := witnessFile.Close()
		if err != nil {
			log.Fatal("could not close witness file:", err)
		}
	}(witnessFile)

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

	fmt.Println("Proof written into " + outputDir + "gnark_plonk_" + gnarkVersion + ".proof")
	fmt.Println("Verification key written into " + outputDir + "gnark_plonk_" + gnarkVersion + ".vk")
	fmt.Println("Public witness written into " + outputDir + "gnark_plonk_pub_input_" + gnarkVersion + ".pub")
}
