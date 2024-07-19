# How to create a Gnark Plonk proof 

 ## Step 1 : Set up your enviroment

 - 1 Install Go: Make suere you have Go installed. You can download it from [here](https://go.dev/doc/install)

 - 2 Initialize a Go Module: Create a new directory for your project and initializa a Go module

 ```bash=
 mkdir gnark_plonk_circuit
 cd gnark_plonk_circuit
 go mod init gnark_plonk_circuit
 ```

 - 3 Install Gnark: Add the library to your project

 ```bash=
 go get github.com/consensys/gnark@v0.10.0
 ```



 ## Step 2: Import dependencies 

 ```bash=
 import (
 	"fmt"
 	"log"
 	"os"
 	"github.com/consensys/gnark-crypto/ecc"
 	"github.com/consensys/gnark/backend/plonk"
 	cs "github.com/consensys/gnark/constraint/bn254"
 	"github.com/consensys/gnark/frontend"
 	"github.com/consensys/gnark/test/unsafekzg"
 	"github.com/consensys/gnark/frontend/cs/scs"
 )
 ```

 Here's what each package is used for:



```fmt```: Standard Go library for formatted input/output.

```log```: Standard Go library for event logging.

 ```os```: Standard Go library for interacting with the operating system.

 ```path/filepath```: Standard Go library for portable file path manipulation.

 ```github.com/consensys/gnark-crypto/ecc```: Provides cryptographic operations over elliptic curves.

 ```github.com/consensys/gnark/backend/plonk``` Gnark backend for the PLONK proving system.

 ```github.com/consensys/gnark/constraint/bn254```: Provides types and functions to work with constraint systems specifically for the BN254 curve.

 ```github.com/consensys/gnark/frontend```: Provides the API for defining constraints and creating witness data.

 ```github.com/consensys/gnark/test/unsafekzg```: Gnark testing utilities for KZG commitments.
 
 ```github.com/consensys/gnark/frontend/cs/scs```: Gnark frontend for the SCS (Sparse Constraint System) builder.


 ## Step 3: Define the circuit

 The circuit structure is defined in this case using the equation

  $x^3 + x + 5 = y$


 ```bash=
 // CubicCircuit defines a simple circuit
 // x**3 + x + 5 == y
 type CubicCircuit struct {
 	X frontend.Variable `gnark:"x"`
 	Y frontend.Variable `gnark:",public"`
 }
 ```
 Here

 ```CubicCircuit```struct contains the variables ```X``` and ```Y```

 ```X``` is a secret input, annotated as ```'gnark:"x"'```

 ```Y``` is a public input, annotated as ```'gnark:",public"'```

 ## Step 4: Define the circuit constraints:

 Establish constraints that the circuit must satisfy. Here you define the logic that relates inputs to outputs, encapsulating the computation:

 ```bash=
 // Define declares the circuit constraints
 // x**3 + x + 5 == y
 func (circuit *CubicCircuit) Define(api frontend.API) error {
 	x3 := api.Mul(circuit.X, circuit.X, circuit.X)
 	api.AssertIsEqual(circuit.Y, api.Add(x3, circuit.X, 5))
 	return nil
 }
 ```

 The ```Define```  method specifies the constraints for the circuit.

 ```x3 := api.Mul(circuit.X, circuit.X, circuit.X)``` computes X**3


 ```api.AssertIsEqual(circuit.Y, api.Add(x3, circuit.X, 5)``` asserts that X**3 + X + 5 == Y

 There are other options that we migth use like ```ÀssertDifferent``` ```AssertIsLessOrEqual```

 ## Step 5: Compile the circuit and generate the proof

 Detail the steps to compile the circuit, generate a witness, create a proof, and verify it:


 we need to specify the directory where the proof, verification key and the public key will be saved

 ```bash
 outputDir := "gnark_plonk_circuit/"
 ```

 To compile the circuit we do

 ```bash=
 	var circuit CubicCircuit
 	// Compile the circuit using scs.NewBuilder
 	ccs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
 	if err != nil {
 		panic("circuit compilation error")
 	}
 ```
 where


 The ```frontend.Compile``` function compiles the circuit using the SCS 
  constraint system.

 ```ecc.BN254.ScalarField()``` specifies the scalar field, in this case  for the BN254 curve.

 ```scs.NewBuilder``` is used to build the sparse constraint system.

 The we generate the SRS (Structured Reference String)

 ```bash=
 	// Generate the SRS and its Lagrange interpolation
 	r1cs := ccs.(*cs.SparseR1CS)
 	srs, srsLagrangeInterpolation, err := unsafekzg.NewSRS(r1cs)
 	if err != nil {
 		panic("KZG setup error")
 	}
 ```

 ```r1cs := ccs.(*cs.SparseR1CS)``` converts the compiled circuit to a sparse R1CS(Rank-1 Constraint Systems) format required by the SRS generation.

 ```unsafekzg.NewSRS``` generates the structured reference string (SRS) and its Lagrange interpolation.


 Next we need to setup PLONK

 ```bash=
 pk, vk, _ := plonk.Setup(ccs, srs, srsLagrangeInterpolation)
 ```

 ```plonk.Setup``` initializes the PLONK proving system with the constraint system, SRS, and its Lagrange interpolation.
 This generates the proving key ```pk``` and verification key ```vk```

 Then the Witness is created

 ```bash=
 	assignment := CubicCircuit{X: 3, Y: 35}
 	fullWitness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
 	if err != nil {
 		log.Fatal(err)
 	}
 	publicWitness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField(), frontend.PublicOnly())
 	if err != nil {
 		log.Fatal(err)
 	}
 ```

 An assignment to the circuit variables is created: ```X = 3``` and ```Y = 35```.

 ```frontend.NewWitness``` creates the full witness including all variables.

 ```frontend.NewWitness``` with ```frontend.PublicOnly()``` creates the public witness including only the public variables.

 Generate the Proof:

 ```bash=
 proof, err := plonk.Prove(ccs, pk, fullWitness)
 	if err != nil {
 		panic("PLONK proof generation error")
 	}
 ```

 ```plonk.Prove``` generates a proof using the compilated circuit,proving key and full witness

 Then to Verify 

 ```bash=
 	// Verify the proof
 	err = plonk.Verify(proof, vk, publicWitness)
 	if err != nil {
 		panic("PLONK proof not verified")
 	}
 ```

 ```plonk.Verify``` verifies the proof using the compilated circuit,proving key and full witness

 Finally we have to serialize and save outputs

 ```bash=
 	// Open files for writing the proof, the verification key, and the public witness
 	proofFile, err := os.Create(outputDir + "plonk.proof")
 	if err != nil {
 		panic(err)
 	}
 	vkFile, err := os.Create( "plonk.vk")
 	if err != nil {
 		panic(err)
 	}
 	witnessFile, err := os.Create( "plonk_pub_input.pub")
 	if err != nil {
 		panic(err)
 	}
 	defer proofFile.Close()
 	defer vkFile.Close()
 	defer witnessFile.Close()
 	// Write the proof to the file
 	_, err = proof.WriteTo(proofFile)
 	if err != nil {
 		panic("could not serialize proof into file")
 	}
 	// Write the verification key to the file
 	_, err = vk.WriteTo(vkFile)
 	if err != nil {
 		panic("could not serialize verification key into file")
 	}
 	// Write the public witness to the file
 	_, err = publicWitness.WriteTo(witnessFile)
 	if err != nil {
 		panic("could not serialize proof into file")
 	}
 	fmt.Println("Proof written into plonk.proof")
 	fmt.Println("Verification key written into plonk.vk")
 	fmt.Println("Public witness written into plonk_pub_input.pub")
 }
 ```

 Files are created for the proof, verification key, and public witness.

 The proof, verification key, and public witness are written to these files.

 This ensures that the proof and related data are saved for later use or verification.

 The complete code is:

 ```bash=
 package main
 import (
 	"fmt"
 	"log"
 	"os"
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
 	proofFile, err := os.Create("plonk.proof")
 	if err != nil {
 		panic(err)
 	}
 	vkFile, err := os.Create("plonk.vk")
 	if err != nil {
 		panic(err)
 	}
 	witnessFile, err := os.Create( "plonk_pub_input.pub")
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
 	fmt.Println("Proof written into plonk.proof")
 	fmt.Println("Verification key written into plonk.vk")
 	fmt.Println("Public witness written into plonk_pub_input.pub")
 }
 ```
