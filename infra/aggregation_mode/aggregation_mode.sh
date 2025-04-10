#!/bin/bash

# Set new server name
while :; do
	echo -e "\nEnter new server name:"
	read -p "> " new_server_name

	if [[ ! "$new_server_name" =~ ^[a-zA-Z0-9-]+$ ]]; then
		echo "Invalid characters used in the server name. Please use only alphanumeric characters and hyphens (-)."
	else
		echo -e "\nSetting new server name to '$new_server_name'..."
		echo "Old server name: $old_server_name"
		sudo hostnamectl set-hostname "$new_server_name"
		sudo sed -i "s/$old_server_name/$new_server_name/g" /etc/hosts
		echo "Please reconnect to the server to see the name change."
		break
	fi
done

# Install other dependencies
sudo apt install -y gcc pkg-config libssl-dev build-essential apt-transport-https ca-certificates curl software-properties-common nvtop

# Install tailscale
curl -fsSL https://pkgs.tailscale.com/stable/ubuntu/noble.noarmor.gpg | sudo tee /usr/share/keyrings/tailscale-archive-keyring.gpg >/dev/null
curl -fsSL https://pkgs.tailscale.com/stable/ubuntu/noble.tailscale-keyring.list | sudo tee /etc/apt/sources.list.d/tailscale.list
sudo apt-get update
sudo apt-get install tailscale
sudo tailscale up --ssh --advertise-tags=tag:server && sudo tailscale set --auto-update

# Install CUDA
sudo add-apt-repository ppa:graphics-drivers/ppa
sudo apt update
sudo apt install nvidia-driver-570

# Setup Docker and CUDA
curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg
curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list | \
  sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \
  sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list
sudo apt-get update
sudo apt-get install -y nvidia-container-toolkit
sudo nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"

# Install SP1
curl -L https://sp1.succinct.xyz | bash
source $HOME/.bashrc
sp1up

# Install cast
curl -L https://foundry.paradigm.xyz | bash
source $HOME/.bashrc
foundryup

# Create directories
mkdir -p repos/proof_aggregation
mkdir -p config
#mkdir -p .config/systemd/user
mkdir -p .keystores

# Clone Repo
cd repos/proof_aggregation
git clone https://github.com/yetanotherco/aligned_layer.git
cd aligned_layer
git checkout staging
cd ../..

# Create keystore
cast wallet import proof_aggregation.keystore -k $HOME/.keystores -i

# Copy config file
#cp repos/proof_aggregation/aligned_layer/config-files/config-proof-aggregator.yaml config/config-proof-aggregator.yaml
.$HOME/repos/proof_aggregation/aligned_layer/infra/aggregation_mode/config_file.sh $HOME/repos/proof_aggregation/aligned_layer/infra/aggregation_mode/config-proof-aggregator.template.yaml

# Build the proof_aggregator
cd repos/proof_aggregation/aligned_layer
cargo install --path aggregation_mode --features prove

# Setup systemd service
sudo cp $HOME/repos/proof_aggregation/aligned_layer/infra/aggregation_mode/aggregation_mode.service /etc/systemd/system/aggregation_mode.service
sudo systemctl enable aggregation_mode.service

# Run the proof_aggregator
sudo systemctl start aggregation_mode.service
