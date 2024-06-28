package main

import (
	"github.com/yetanotherco/aligned_layer/scripts/test_examples/gnark_groth16_bn254_infinite_script/pkg"
	"os"
	"strconv"
)

func main() {
	x, err := strconv.Atoi(os.Args[1])
	if err != nil {
		panic("error parsing input")
	}

	pkg.GenerateIneqProof(x)
}
