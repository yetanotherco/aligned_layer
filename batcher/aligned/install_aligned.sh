#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

echo "Installing Aligned..."

BASE_DIR=$HOME
ALIGNED_DIR="${ALIGNED_DIR-"$BASE_DIR/.aligned"}"
ALIGNED_BIN_DIR="$ALIGNED_DIR/bin"
ALIGNED_BIN_PATH="$ALIGNED_BIN_DIR/aligned"
CURRENT_TAG=$(curl -s -L \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/yetanotherco/aligned_layer/releases/latest \
  | grep '"tag_name":' | awk -F'"' '{print $4}')
RELEASE_URL="https://github.com/yetanotherco/aligned_layer/releases/download/$CURRENT_TAG/"
ARCH=$(uname -m)

if [ "$ARCH" == "x86_64" ]; then
    FILE="aligned-x86"
elif [ "$ARCH" == "arm64" ]; then
    FILE="aligned-arm64"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

mkdir -p "$ALIGNED_BIN_DIR"
if curl -sSf -L "$RELEASE_URL$FILE" -o "$ALIGNED_BIN_PATH"; then
    echo "Aligned download successful, installing $CURRENT_TAG release..."
else
    echo "Error: Failed to download $RELEASE_URL$FILE"
    exit 1
fi
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

echo "Aligned $CURRENT_TAG installed successfully in $ALIGNED_BIN_PATH."
echo "Detected your preferred shell is $PREF_SHELL and added aligned to PATH."
echo "Run 'source $PROFILE' or start a new terminal session to use aligned."
