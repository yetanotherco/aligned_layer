#!/bin/bash

echo "Installing Aligned..."

BASE_DIR=$HOME
ALIGNED_DIR="${ALIGNED_DIR-"$BASE_DIR/.aligned"}"
ALIGNED_BIN_DIR="$ALIGNED_DIR/bin"
ALIGNED_BIN_PATH="$ALIGNED_BIN_DIR/aligned"
#TODO: See if v0.1.3 can be replaced with a variable of the latest release
RELEASE_URL="https://github.com/yetanotherco/aligned_layer/releases/download/v0.1.3/"

ARCH=$(uname -m)

if [ "$ARCH" == "x86_64" ]; then
    FILE="batcher-client-x86"
elif [ "$ARCH" == "arm64" ]; then
    FILE="batcher-client-arm64"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

mkdir -p "$ALIGNED_BIN_DIR"
curl -sSf -L "$RELEASE_URL$FILE" -o "$ALIGNED_BIN_PATH"
chmod +x "$ALIGNED_BIN_PATH"

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
    echo "aligned: could not detect shell, manually add ${ALIGNED_BIN_DIR} to your PATH."
    exit 1
esac

# Only add aligned if it isn't already in PATH.
if [[ ":$PATH:" != *":${ALIGNED_BIN_DIR}:"* ]]; then
    # Add the aligned directory to the path and ensure the old PATH variables remain.
    # If the shell is fish, echo fish_add_path instead of export.
    if [[ "$PREF_SHELL" == "fish" ]]; then
        echo >> "$PROFILE" && echo "fish_add_path -a $ALIGNED_BIN_DIR" >> "$PROFILE"
    else
        echo >> "$PROFILE" && echo "export PATH=\"\$PATH:$ALIGNED_BIN_DIR\"" >> "$PROFILE"
    fi
fi

echo "Aligned installed successfully."
echo "Detected your preferred shell is $PREF_SHELL and added aligned to PATH."
echo "Run 'source $PROFILE' or start a new terminal session to use aligned."
