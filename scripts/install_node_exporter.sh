#!/bin/bash

echo "Installing Node Exporter..."

VERSION="1.8.2"
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [[ "$ARCH" == "x86_64" ]]; then
    ARCH="amd64"
fi

BASE_DIR="$HOME"
NODE_EXPORTER_DIR="${NODE_EXPORTER_DIR-"$BASE_DIR/.node_exporter"}"
NODE_EXPORTER_BIN_DIR="$NODE_EXPORTER_DIR/bin"
NODE_EXPORTER_BIN_PATH="$NODE_EXPORTER_BIN_DIR/node_exporter"

RELEASE_URL="https://github.com/prometheus/node_exporter/releases/download/v$VERSION/"
FILE="node_exporter-$VERSION.$OS-$ARCH.tar.gz"

mkdir -p "$NODE_EXPORTER_BIN_DIR"

echo "Downloading Node Exporter from $RELEASE_URL$FILE..."
if wget -q "$RELEASE_URL$FILE"; then
    echo "Download successful."
else
    echo "Error: Failed to download $RELEASE_URL$FILE"
    exit 1
fi

echo "Extracting Node Exporter..."
if tar xvfz $FILE; then
    mv "node_exporter-$VERSION.$OS-$ARCH/node_exporter" "$NODE_EXPORTER_BIN_PATH"
    chmod +x "$NODE_EXPORTER_BIN_PATH"
    rm -rf "node_exporter-$VERSION.$OS-$ARCH" $FILE
else
    echo "Error: Failed to extract $FILE"
    exit 1
fi

# Store the correct profile file (i.e. .profile for bash or .zshenv for ZSH).
case $SHELL in
*/zsh)
    PROFILE="${ZDOTDIR-"$HOME"}/.zshenv"
    PREF_SHELL=zsh
    ;;
*/bash)
    PROFILE=$HOME/.bashrc
    PREF_SHELL=bash
    ;;
*/fish)
    PROFILE=$HOME/.config/fish/config.fish
    PREF_SHELL=fish
    ;;
*/ash)
    PROFILE=$HOME/.profile
    PREF_SHELL=ash
    ;;
*)
    echo "could not detect shell, manually add ${NODE_EXPORTER_BIN_DIR} to your PATH."
    exit 1
esac

# Only add node-exporter if it isn't already in PATH.
if [[ ":$PATH:" != *":${NODE_EXPORTER_BIN_DIR}:"* ]]; then
    # Add the node-exporter directory to the path and ensure the old PATH variables remain.
    # If the shell is fish, echo fish_add_path instead of export.
    if [[ "$PREF_SHELL" == "fish" ]]; then
        echo >> "$PROFILE" && echo "fish_add_path -a $NODE_EXPORTER_BIN_DIR" >> "$PROFILE"
    else
        echo >> "$PROFILE" && echo "export PATH=\"\$PATH:$NODE_EXPORTER_BIN_DIR\"" >> "$PROFILE"
    fi
fi

echo "Node exporter $VERSION installed successfully in $NODE_EXPORTER_BIN_PATH."
echo "Detected your preferred shell is $PREF_SHELL and added node-exporter to PATH."
echo "Run 'source $PROFILE' or start a new terminal session to use node-exporter."
