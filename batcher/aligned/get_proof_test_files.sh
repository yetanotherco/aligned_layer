#!/bin/bash

SP1_ELF_URL="https://raw.githubusercontent.com/yetanotherco/aligned_layer/main/batcher/aligned/test_files/sp1/sp1_fibonacci-elf"
SP1_PROOF_URL="https://raw.githubusercontent.com/yetanotherco/aligned_layer/main/batcher/aligned/test_files/sp1/sp1_fibonacci.proof"

SP1_ELF_NAME="sp1_fibonacci-elf"
SP1_PROOF_NAME="sp1_fibonacci.proof"

BASE_DIR=$HOME
ALIGNED_DIR="${ALIGNED_DIR-"$BASE_DIR/.aligned"}"
ALIGNED_TEST_FILES_DIR="$ALIGNED_DIR/test_files"

mkdir -p "$ALIGNED_TEST_FILES_DIR"

echo "Downloading SP1 ELF file..."

if curl -sSf -L "$SP1_ELF_URL" -o "$ALIGNED_TEST_FILES_DIR/$SP1_ELF_NAME"; then
    echo "SP1 ELF download successful"
else
    echo "Error: Failed to download $SP1_ELF_URL"
    exit 1
fi

echo "Downloading SP1 proof file..."

if curl -sSf -L "$SP1_PROOF_URL" -o "$ALIGNED_TEST_FILES_DIR/$SP1_PROOF_NAME"; then
    echo "SP1 proof downloaded successful"
else
    echo "Error: Failed to downloaded $SP1_PROOF_URL"
    exit 1
fi

chmod +x "$ALIGNED_TEST_FILES_DIR/$SP1_ELF_NAME"
chmod +x "$ALIGNED_TEST_FILES_DIR/$SP1_PROOF_NAME"

echo "SP1 ELF and proof files downloaded successfully in $ALIGNED_TEST_FILES_DIR"
