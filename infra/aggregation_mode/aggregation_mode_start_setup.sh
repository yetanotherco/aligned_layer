#!/bin/bash

# This script is for setting up a server that manages the GPU Server startup process.

# Create directories
mkdir -p repos/aggregation_mode
mkdir -p config
mkdir -p .config/systemd/user

# Clone repository
cd $HOME/repos/aggregation_mode
git clone https://github.com/yetanotherco/aligned_layer.git
cd aligned_layer
git checkout staging
cd $HOME/

# Copy systemd service file
cp $HOME/repos/aggregation_mode/aligned_layer/infra/aggregation_mode/aggregation_mode_start.service $HOME/.config/systemd/user/aggregation_mode_start.service
cp $HOME/repos/aggregation_mode/aligned_layer/infra/aggregation_mode/aggregation_mode_start.timer $HOME/.config/systemd/user/aggregation_mode_start.timer

# Copy configuration file
cp $HOME/repos/aggregation_mode/aligned_layer/infra/aggregation_mode/.env.aggregation_mode_start $HOME/config/

# Enable and start the systemd timer
systemctl --user enable aggregation_mode_start.timer
systemctl --user start aggregation_mode_start.timer

# Check status
systemctl --user status aggregation_mode_start.timer


