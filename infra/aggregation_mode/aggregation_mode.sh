#!/bin/bash

# This guide assumes you already clone the github repository
# You have to cd to the repository

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

# Enable linger
sudo loginctl enable-linger user

# Install other dependencies
sudo apt install -y gcc pkg-config libssl-dev build-essential apt-transport-https ca-certificates curl software-properties-common nvtop

# Install docker
sudo apt-get update
sudo apt-get install ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo groupadd docker
sudo usermod -aG docker $USER
newgrp docker

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

# If see errors
sudo apt-mark unhold cuda-drivers cuda-toolkit-12-6 nvidia-dkms-565-server nvidia-fabricmanager-565 nvidia-headless-565-server nvidia-utils-565-server
sudo apt update
sudo apt install nvidia-driver-570
sudo apt autoremove
sudo apt autoclean
sudo reboot
nvidia-smi # To check if the driver is installed correctly

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

# Install Risc0
curl -L https://risczero.com/install | bash
source $HOME/.bashrc
rzup install

# Install cast
curl -L https://foundry.paradigm.xyz | bash
source $HOME/.bashrc
foundryup

# Create directories
mkdir -p ~/config
mkdir -p ~/.config/systemd/user
mkdir -p ~/.keystores

# Create keystore
cast wallet import proof_aggregation.keystore -k $HOME/.keystores -i

# Create config file for SP1
./infra/aggregation_mode/config_file.sh ./infra/aggregation_mode/config-proof-aggregator-sp1.template.yaml $HOME/config/config-proof-aggregator-sp1.yaml
read -p "Enter a block number for SP1 (last_aggregated_block): " num && echo "{\"last_aggregated_block\":$num}" > $HOME/config/proof-aggregator-sp1.last_aggregated_block.json

# Create config file for Risc0
./infra/aggregation_mode/config_file.sh ./infra/aggregation_mode/config-proof-aggregator-risc0.template.yaml $HOME/config/config-proof-aggregator-risc0.yaml
read -p "Enter a block number for Risc0 (last_aggregated_block): " num && echo "{\"last_aggregated_block\":$num}" > $HOME/config/proof-aggregator-risc0.last_aggregated_block.json

# Build the proof_aggregator
make install_aggregation_mode

# Copy run script
cp ./infra/aggregation_mode/run.sh $HOME/run.sh
chmod 744 $HOME/run.sh

# Setup systemd service
cp ./infra/aggregation_mode/aggregation_mode.service $HOME/.config/systemd/user/aggregation_mode.service
cp ./infra/aggregation_mode/aggregation_mode.timer $HOME/.config/systemd/user/aggregation_mode.timer

#sudo systemctl enable aggregation_mode.service
systemctl --user enable aggregation_mode.timer
systemctl --user start aggregation_mode.timer

# Run the proof_aggregator manually if you want
systemctl --user start aggregation_mode.service

# Check timer status
systemctl --user status aggregation_mode.timer

# Check logs
journalctl -xfeu aggregation_mode.service --user -n10
