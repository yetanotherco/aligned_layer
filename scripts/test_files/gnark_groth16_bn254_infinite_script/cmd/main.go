package main

import (
	"os"
	"strconv"

	"github.com/yetanotherco/aligned_layer/scripts/test_files/gnark_groth16_bn254_infinite_script/pkg"
)

func main() {
	x, err := strconv.Atoi(os.Args[1])
	outputDir := "../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/"

	if len(os.Args) > 2 {
		outputDir = os.Args[2]
	}
	if err != nil {
		panic("error parsing input")
	}

	pkg.GenerateIneqProof(x, outputDir)
}
