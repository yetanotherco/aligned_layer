package pkg

import (
	"fmt"
	"log"
	"os"
	"strconv"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"

	//	"github.com/consensys/gnark/frontend/cs/scs"
	"github.com/consensys/gnark/frontend/cs/r1cs"

	"github.com/consensys/gnark/frontend"
)

// CubicCircuit defines a simple circuit
// x != 0
type InequalityCircuit struct {
	// struct tags on a variable is optional
	// default uses variable name and secret visibility.
	X frontend.Variable `gnark:"x"`
}

// Define declares the circuit constraints
// x != 0
func (circuit *InequalityCircuit) Define(api frontend.API) error {
	api.AssertIsDifferent(circuit.X, 0)
	return nil
}

func GenerateIneqProof(x int) {
	outputDir := "task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/"

	var circuit InequalityCircuit
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

	assignment := InequalityCircuit{X: x}

	fullWitness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
	if err != nil {
		log.Fatal(err)
	}

	// TODO , this should be empty, right? private input is x and no pub input
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
	proofFile, err := os.Create(outputDir + "ineq_" + strconv.Itoa(x) + "_groth16.proof")
	if err != nil {
		panic(err)
	}
	vkFile, err := os.Create(outputDir + "ineq_" + strconv.Itoa(x) + "_groth16.vk")
	if err != nil {
		panic(err)
	}
	witnessFile, err := os.Create(outputDir + "ineq_" + strconv.Itoa(x) + "_groth16.pub")
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

	fmt.Println("Proof written into ineq_{x}_groth16.proof")
	fmt.Println("Verification key written into groth16_verification_key")
	fmt.Println("Public witness written into witness.pub")
}
