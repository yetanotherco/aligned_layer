package compress

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/witness"
	"github.com/consensys/gnark/frontend"
)

type Circuit struct {
	Original   [100]frontend.Variable
	Compressed [200]frontend.Variable `gnark:",public"`
}

func (circuit *Circuit) Define(api frontend.API) error {

	symbol := circuit.Compressed[0]
	multiplicity := circuit.Compressed[1]
	multiplicity = api.Sub(multiplicity, frontend.Variable(48))
	y_left := circuit.Compressed

	for i := 0; i < 100; i++ {
		//ensure equality at i-th position
		api.AssertIsEqual(circuit.Original[i], symbol)

		// decrement multiplicity counter
		multiplicity = api.Sub(multiplicity, 1)

		// if counter is at zero, chomp two from compressed list
		for i := 0; i < 198; i++ {
			y_left[i] = api.Select(api.IsZero(multiplicity), y_left[i+2], y_left[i])
		}
		y_left[198] = api.Select(api.IsZero(multiplicity), frontend.Variable(0), y_left[198])
		y_left[199] = api.Select(api.IsZero(multiplicity), frontend.Variable(0), y_left[199])

		asciishift := api.Select(api.IsZero(y_left[1]), y_left[1], api.Sub(y_left[1], frontend.Variable(48)))
		multiplicity = api.Select(api.IsZero(multiplicity), asciishift, multiplicity)

		symbol = y_left[0]
	}

	return nil
}

func ReadFromInputPath(pathInput string) (map[string]interface{}, error) {

	absPath, err := filepath.Abs(pathInput)
	if err != nil {
		fmt.Println("Error constructing absolute path:", err)
		return nil, err
	}

	file, err := os.Open(absPath)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var data map[string]interface{}
	err = json.NewDecoder(file).Decode(&data)
	if err != nil {
		panic(err)
	}

	return data, nil
}

func FromJson(pathInput string) witness.Witness {

	data, err := ReadFromInputPath(pathInput)
	if err != nil {
		panic(err)
	}

	// send original to list of integers
	chars := []rune(data["original"].(string))
	original := [100]frontend.Variable{}
	compressed := [200]frontend.Variable{}
	preImage := []byte(data["original"].(string))

	prev_x := int(chars[0])
	multiplicity := 0
	y_idx := 0

	for i := 0; i < 100; i++ {
		if i < len(chars) {
			original[i] = frontend.Variable(chars[i])

			if int(chars[i]) != prev_x {
				compressed[y_idx] = frontend.Variable(prev_x)
				y_idx = y_idx + 1
				compressed[y_idx] = frontend.Variable(multiplicity + 48)
				y_idx = y_idx + 1

				multiplicity = 1
			} else {
				multiplicity = multiplicity + 1
			}
			prev_x = int(chars[i])
		} else if i == len(chars) {
			compressed[y_idx] = frontend.Variable(prev_x)
			y_idx = y_idx + 1
			compressed[y_idx] = frontend.Variable(multiplicity + 48) //add 48 for ascii encoding
			y_idx = y_idx + 1

			original[i] = frontend.Variable(0)
			preImage = append(preImage, 0)
		} else { // pad original with zeros to length 100
			original[i] = frontend.Variable(0)
			preImage = append(preImage, 0)
		}
		// pad compressed array to length 200
		for i := y_idx; i < 200; i++ {
			compressed[i] = frontend.Variable(0)
		}
	}

	assignment := Circuit{
		Original:   original,
		Compressed: compressed,
	}

	w, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
	if err != nil {
		panic(err)
	}
	return w
}
