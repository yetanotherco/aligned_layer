# Verify groth16 proof allow malleability

**Author(s):** Mohammed Benhelli [@Fuzzinglabs](https://github.com/FuzzingLabs/)

**Date:** 09/08/2024

## **Executive Summary**

While auditing and fuzzing some verification functions, we discovered that the groth16 verification function allows
malleability. This is an expected behavior of the groth16 proof system.

## Vulnerability Details

- **Severity:** Informational

- **Affected Components:** 
  - `batcher/aligned-batcher/gnark/verifier.go`
  - `operator/pkg/operator.go`

## Environment

- **Distro Version:** Ubuntu 22.04.4 LTS
- **Additional Environment Details:** go version go1.22.5 linux/amd64

## Steps to Reproduce

1. Create a file in your test folder with the following content:
    ```go
    package malleability_test
    
    import (
        "github.com/consensys/gnark-crypto/ecc"
        "github.com/consensys/gnark-crypto/ecc/bn254"
        "github.com/consensys/gnark/backend/groth16"
        groth16Bn254 "github.com/consensys/gnark/backend/groth16/bn254"
        "github.com/consensys/gnark/frontend"
        "github.com/consensys/gnark/frontend/cs/r1cs"
        "github.com/stretchr/testify/assert"
        "math/big"
        "math/rand"
        "testing"
    )
    
    type CubicCircuit struct {
        X frontend.Variable `gnark:"x"`
        Y frontend.Variable `gnark:",public"`
    }
    
    func (circuit *CubicCircuit) Define(api frontend.API) error {
        x3 := api.Mul(circuit.X, circuit.X, circuit.X)
        api.AssertIsEqual(circuit.Y, api.Add(x3, circuit.X, 5))
        return nil
    }
    
    func TestGroth16Bn254Randomize(t *testing.T) {
        var circuit CubicCircuit
        ccs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
        if err != nil {
            t.Fatalf("circuit compilation error: %v", err)
        }
    
        pk, vk, _ := groth16.Setup(ccs)
    
        assignment := CubicCircuit{X: 3, Y: 35}
    
        fullWitness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
        if err != nil {
            t.Fatalf("failed to create full witness: %v", err)
        }
    
        publicWitness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField(), frontend.PublicOnly())
        if err != nil {
            t.Fatalf("failed to create public witness: %v", err)
        }
    
        proof, err := groth16.Prove(ccs, pk, fullWitness)
        if err != nil {
            t.Fatalf("GROTH16 proof generation error: %v", err)
        }
    
        if err = groth16.Verify(proof, vk, publicWitness); err != nil {
            t.Fatalf("GROTH16 proof not verified: %v", err)
        }
    
        proofRand := randomizeGrothBn254(vk.(*groth16Bn254.VerifyingKey), proof.(*groth16Bn254.Proof))
        assert.NotEqualf(t, proofRand, proof, "proofs should not be equal")
    
        if err = groth16.Verify(proofRand, vk, publicWitness); err != nil {
            t.Fatalf("GROTH16 proof not verified: %v", err)
        }
    }
    
    // https://blog.sui.io/malleability-groth16-zkproof/
    // https://www.beosin.com/resources/beosin%E2%80%99s-research--transaction-malleability-attack-of-groth
    func randomizeGrothBn254(vk *groth16Bn254.VerifyingKey, proof *groth16Bn254.Proof) *groth16Bn254.Proof {
        r1 := new(big.Int)
        r2 := new(big.Int)
        for r1.Sign() == 0 || r2.Sign() == 0 {
            r1 = ecc.BN254.ScalarField().Rand(rand.New(rand.NewSource(0)), ecc.BN254.ScalarField())
            r2 = ecc.BN254.ScalarField().Rand(rand.New(rand.NewSource(1)), ecc.BN254.ScalarField())
        }
    
        newA := new(bn254.G1Affine).ScalarMultiplication(&proof.Ar, new(big.Int).ModInverse(r1, ecc.BN254.ScalarField()))
        newB := new(bn254.G2Affine).ScalarMultiplication(&proof.Bs, r1)
        newB.Add(newB, new(bn254.G2Affine).ScalarMultiplication(&vk.G2.Delta, new(big.Int).Mul(r1, r2)))
        newC := new(bn254.G1Affine).Add(&proof.Krs, new(bn254.G1Affine).ScalarMultiplication(&proof.Ar, r2))
    
        return &groth16Bn254.Proof{
            Ar:  *newA,
            Bs:  *newB,
            Krs: *newC,
        }
    }
    ```
2. Run the test:
    ```sh
    go test -v -run TestGroth16Bn254Randomize
    ```

## Recommendations

Maybe inform the user that the groth16 proof system allows malleability in case it could affect their usage.
