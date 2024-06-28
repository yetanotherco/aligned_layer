## Compression Circuit

This circuit is built with Gnark frontend v0.9.0 and accepts a string as input

```json
{
  "original": "rrrrrkkkkkkMMMsssssssssssssssJJJJJJJJJJJJJJJJ"
}
```

Where the compressed string collapses each symbol with multiplicity 1-9.
See the docs for a more thorough explanation of the functionality.

Within `circuit.go` we have the circuit definition

```go
func (circuit *Circuit) Define(api frontend.API) error { }
```

which iterates through each symbol in the original string and ensures that the corresponding pointer in the compressed list matches.

Note that the inputs to the circuit are not the strings represented in this JSON input, but preprocessed versions of these strings which are turned into integer lists.
From the input example above, we would actually input:

```json
"X": [97,97,97,98,98,99,99,99,99,99,99,0,0,0,...],
"Y": [97,51,98,50,99,54,0,0,...]
```

to the circuit and that is what `FromJson` does.

Refer to the [Sindri Gnark tutorial](https://sindri.app/docs/how-to-guides/frameworks/gnark/) for more information on this circuit and instructions for running the `compile.mjs` script.
