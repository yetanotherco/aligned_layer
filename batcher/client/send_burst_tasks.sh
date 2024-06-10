#!/bin/bash

counter=1
burst=8

# Establecer el valor de 'burst' desde el primer argumento
if [ -z "$1" ]; then
    echo "Using default burst value: 8"
elif ! [[ "$1" =~ ^[0-9]+$ ]]; then
    echo "Error: First argument must be a number."
    exit 1
else
    burst=$1
    echo "Using burst value: $burst"
fi

# Establecer el valor inicial del 'counter' desde el segundo argumento
if [ -z "$2" ]; then
    echo "Using default counter start value: 1"
elif ! [[ "$2" =~ ^[0-9]+$ ]]; then
    echo "Error: Second argument must be a number."
    exit 1
else
    counter=$2
    echo "Starting counter from: $counter"
fi

count=0  # Inicializa un contador para el número de ejecuciones

while true
do
    if [ "$count" -ge "$burst" ]; then
        break  # Salir del bucle si se han ejecutado 'burst' veces
    fi

    echo "Generating proof $counter != 0"
    ./batcher/client/generate_proof_and_send.sh $counter $burst &
    sleep 1
    counter=$((counter + 1))
    count=$((count + 1))  # Incrementar el contador de ejecuciones
done
