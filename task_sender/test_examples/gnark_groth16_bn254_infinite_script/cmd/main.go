package main

import (
	"os"
	"strconv"
	pkg "github.com/yetanotherco/aligned_layer/task_sender/test_examples/gnark_groth16_bn254_infinite_script/pkg"
)

func main() {
	x, err := strconv.Atoi(os.Args[1])
	if err != nil {
		panic("error parsing input")
	}

	pkg.generate_ineq_proof(x)
}
