package pkg

import (
	"fmt"
	"log"
	"os"
	"strconv"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/plonk"
	cs "github.com/consensys/gnark/constraint/bn254"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/scs"
	"github.com/consensys/gnark/test/unsafekzg"
)

// InequalityCircuit defines a simple circuit
// x != 0
type EqualityCircuit struct {
	X frontend.Variable `gnark:"x"`
	// 	Y frontend.Variable `gnark:",public"`
}

// Define declares the circuit constraints
// x == y
func (circuit *EqualityCircuit) Define(api frontend.API) error {
	//	api.AssertIsEqual(circuit.X, circuit.Y)
	api.AssertIsDifferent(circuit.X, 0)
	api.AssertIsDifferent(circuit.X, -1)
	return nil
}

func GenerateIneqProof(x int) {
	outputDir := "task_sender/test_examples/gnark_plonk_bn254_infinite_script/infinite_proofs/"
	//	y := x
	var circuit EqualityCircuit
	ccs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
	if err != nil {
		panic("circuit compilation error: " + err.Error())
	}

	scs := ccs.(*cs.SparseR1CS)

	srs, srsLagrangeInterpolation, err := unsafekzg.NewSRS(scs) //Here
	if err != nil {
		panic("KZG setup error: " + err.Error())
	}

	pk, vk, err := plonk.Setup(ccs, srs, srsLagrangeInterpolation)
	if err != nil {
		panic("PLONK setup error: " + err.Error())
	}

	assignment := EqualityCircuit{X: x}

	fullWitness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
	if err != nil {
		log.Fatal("Error creating full witness: ", err)
	}

	publicWitness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField(), frontend.PublicOnly())
	if err != nil {
		log.Fatal("Error creating public witness: ", err)
	}

	proof, err := plonk.Prove(ccs, pk, fullWitness)
	if err != nil {
		panic("PLONK proof generation error: " + err.Error())
	}

	err = plonk.Verify(proof, vk, publicWitness)
	if err != nil {
		panic("PLONK proof not verified: " + err.Error())
	}

	proofFile, err := os.Create(outputDir + "ineq_" + strconv.Itoa(x) + "_plonk.proof")
	if err != nil {
		panic("Error creating proof file: " + err.Error())
	}
	vkFile, err := os.Create(outputDir + "ineq_" + strconv.Itoa(x) + "_plonk.vk")
	if err != nil {
		panic("Error creating verification key file: " + err.Error())
	}
	witnessFile, err := os.Create(outputDir + "ineq_" + strconv.Itoa(x) + "_plonk.pub")
	if err != nil {
		panic("Error creating public witness file: " + err.Error())
	}
	defer proofFile.Close()
	defer vkFile.Close()
	defer witnessFile.Close()

	_, err = proof.WriteTo(proofFile)
	if err != nil {
		panic("Could not serialize proof into file: " + err.Error())
	}
	_, err = vk.WriteTo(vkFile)
	if err != nil {
		panic("Could not serialize verification key into file: " + err.Error())
	}
	_, err = publicWitness.WriteTo(witnessFile)
	if err != nil {
		panic("Could not serialize public witness into file: " + err.Error())
	}

	fmt.Println("Proof written into ineq_" + strconv.Itoa(x) + "_plonk.proof")
	fmt.Println("Verification key written into ineq_" + strconv.Itoa(x) + "_plonk.vk")
	fmt.Println("Public witness written into ineq_" + strconv.Itoa(x) + "_plonk.pub")
}
