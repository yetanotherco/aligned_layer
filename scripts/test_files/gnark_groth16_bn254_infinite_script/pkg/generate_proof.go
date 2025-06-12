package pkg

import (
	"github.com/consensys/gnark"
	"log"
	"os"
	"strconv"
	"strings"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
)

// InequalityCircuit defines a simple circuit
// x != 0
type InequalityCircuit struct {
	X frontend.Variable `gnark:"x"`
}

// Define declares the circuit constraints
// x != 0
func (circuit *InequalityCircuit) Define(api frontend.API) error {
	api.AssertIsDifferent(circuit.X, 0)
	return nil
}

func GenerateIneqProof(x int, outputDir string) {
	gnarkVersion := strings.ReplaceAll(gnark.Version.String(), ".", "_")

	var circuit InequalityCircuit
	ccs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	if err != nil {
		panic("circuit compilation error")
	}

	pk, vk, _ := groth16.Setup(ccs)

	assignment := InequalityCircuit{X: x}

	fullWitness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
	if err != nil {
		log.Fatal(err)
	}

	publicWitness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField(), frontend.PublicOnly())
	if err != nil {
		log.Fatal(err)
	}

	proof, err := groth16.Prove(ccs, pk, fullWitness)
	if err != nil {
		panic("GROTH16 proof generation error")
	}

	err = groth16.Verify(proof, vk, publicWitness)
	if err != nil {
		panic("GROTH16 proof not verified")
	}

	proofFile, err := os.Create(outputDir + "ineq_" + strconv.Itoa(x) + "_groth16_" + gnarkVersion + ".proof")
	if err != nil {
		panic(err)
	}
	vkFile, err := os.Create(outputDir + "ineq_" + strconv.Itoa(x) + "_groth16_" + gnarkVersion + ".vk")
	if err != nil {
		panic(err)
	}
	witnessFile, err := os.Create(outputDir + "ineq_" + strconv.Itoa(x) + "_groth16_" + gnarkVersion + ".pub")
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
}
