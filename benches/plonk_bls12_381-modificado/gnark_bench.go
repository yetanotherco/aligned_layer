package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"sort"
	"time"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/plonk"
	"github.com/consensys/gnark/backend/witness"
)

func verifyPlonkProof(proofBytes, vkBytes, pubInputBytes []byte) bool {
	vkReader := bytes.NewReader(vkBytes)
	vk := plonk.NewVerifyingKey(ecc.BLS12_381)
	start := time.Now()
	if _, err := vk.ReadFrom(vkReader); err != nil {
		fmt.Println("Error reading verification key:", err)
		return false
	}
	fmt.Printf("VK Read Duration: %v\n", time.Since(start))

	start = time.Now()
	proofReader := bytes.NewReader(proofBytes)
	proof := plonk.NewProof(ecc.BLS12_381)
	if _, err := proof.ReadFrom(proofReader); err != nil {
		fmt.Println("Error reading the proof:", err)
		return false
	}
	fmt.Printf("Proof Read Duration: %v\n", time.Since(start))

	start = time.Now()
	pubInputReader := bytes.NewReader(pubInputBytes)
	pubInput, err := witness.New(ecc.BLS12_381.ScalarField())
	if err != nil {
		fmt.Println("Error instantiating the witness:", err)
		return false
	}
	if _, err = pubInput.ReadFrom(pubInputReader); err != nil {
		fmt.Println("Error reading the public input PLONK:", err)
		return false
	}
	fmt.Printf("Public Input Read Duration: %v\n", time.Since(start))

	start = time.Now()
	err = plonk.Verify(proof, vk, pubInput)
	fmt.Printf("Verification Duration: %v\n", time.Since(start))

	return err == nil
}

func main() {
	proofBytes, err := ioutil.ReadFile("plonk_cubic_circuit.proof")
	if err != nil {
		fmt.Println("Error reading the proof file:", err)
		os.Exit(1)
	}
	vkBytes, err := ioutil.ReadFile("plonk_verification_key")
	if err != nil {
		fmt.Println("Error reading the verification key file:", err)
		os.Exit(1)
	}
	pubInputBytes, err := ioutil.ReadFile("witness.pub")
	if err != nil {
		fmt.Println("Error reading the public witness file:", err)
		os.Exit(1)
	}

	numCycles := 10
	numVerifications := 100
	var verificationTimes []time.Duration

	for j := 0; j < numCycles; j++ {
		for i := 0; i < numVerifications; i++ {
			startVerification := time.Now()
			if !verifyPlonkProof(proofBytes, vkBytes, pubInputBytes) {
				fmt.Println("The PLONK proof is invalid.")
				os.Exit(1)
			}
			verificationTimes = append(verificationTimes, time.Since(startVerification))
		}
	}

	// Compute statistics
	totalTime := time.Duration(0)
	for _, verificationTime := range verificationTimes {
		totalTime += verificationTime
	}
	averageTime := totalTime / time.Duration(len(verificationTimes))

	sort.Slice(verificationTimes, func(i, j int) bool { return verificationTimes[i] < verificationTimes[j] })
	medianTime := verificationTimes[len(verificationTimes)/2]

	var sumOfSquares time.Duration
	for _, verificationTime := range verificationTimes {
		diff := verificationTime - averageTime
		sumOfSquares += diff * diff
	}
	variance := sumOfSquares / time.Duration(len(verificationTimes))
	stdDev := time.Duration(math.Sqrt(float64(variance)))

	fmt.Printf("%d verifications completed across %d cycles.\n", len(verificationTimes), numCycles)
	fmt.Printf("Total time for all verifications: %s\n", totalTime)
	fmt.Printf("Average time per verification: %s\n", averageTime)
	fmt.Printf("Median time per verification: %s\n", medianTime)
	fmt.Printf("Standard deviation of verification times: %s\n", stdDev)
}
