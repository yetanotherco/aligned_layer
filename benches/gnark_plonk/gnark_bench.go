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
	if _, err := vk.ReadFrom(vkReader); err != nil {
		fmt.Println("Error reading verification key:", err)
		return false
	}

	proofReader := bytes.NewReader(proofBytes)
	proof := plonk.NewProof(ecc.BLS12_381)
	if _, err := proof.ReadFrom(proofReader); err != nil {
		fmt.Println("Error reading the proof:", err)
		return false
	}

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

	err = plonk.Verify(proof, vk, pubInput)
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

	var durations []time.Duration
	for j := 0; j < 2; j++ {
		start := time.Now()
		for i := 0; i < 100; i++ {
			if !verifyPlonkProof(proofBytes, vkBytes, pubInputBytes) {
				fmt.Println("The PLONK proof is invalid.")
				os.Exit(1)
			}
		}
		durations = append(durations, time.Since(start))
	}

	// Compute statistics
	totalTime := time.Duration(0)
	for _, duration := range durations {
		totalTime += duration
	}
	averageTime := totalTime / time.Duration(len(durations))

	sort.Slice(durations, func(i, j int) bool { return durations[i] < durations[j] })
	medianTime := durations[len(durations)/2]

	var sumOfSquares time.Duration
	for _, duration := range durations {
		diff := duration - averageTime
		sumOfSquares += diff * diff
	}
	variance := sumOfSquares / time.Duration(len(durations))
	stdDev := time.Duration(math.Sqrt(float64(variance)))

	fmt.Printf("100 cycles of 1000 PLONK verifications completed.\n")
	fmt.Printf("Total time: %s\n", totalTime)
	fmt.Printf("Average time per cycle: %s\n", averageTime)
	fmt.Printf("Median time per cycle: %s\n", medianTime)
	fmt.Printf("Standard deviation: %s\n", stdDev)
}
