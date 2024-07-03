#!/bin/bash

# Loop through powers of 2 up to 1024 (2^10)
for i in {1..10}
do
    # Calculate power of 2
    result=$((2**i))
    echo N: $result
    array="0x0000000000000000000000000000000000000000000000000000000000000000"
    for ((j=1; j<=$result; j++))
    do
        # echo $j
        printf -v val "0x%064x" $j
        array="$array,$val"
        # echo $array
    done
    cast send 0x7969c5ed335650692bc04293b07f5bf2e7a673c0 "calculateMerkleRoot(bytes32[])(bytes32)" "[$array]" --private-key 0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba | grep "gasUsed"
done

