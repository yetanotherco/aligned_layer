terraform {
  required_providers {
    tailscale = {
      source = "tailscale/tailscale"
    }
  }
}

# Create ephemeral Tailscale auth key
resource "tailscale_tailnet_key" "postgres_monitor" {
  reusable      = false
  ephemeral     = true
  preauthorized = true
  expiry        = 3600
  description   = "Ephemeral key for postgres-monitor"
  tags          = ["tag:server"]
}

# Upload existing SSH public key to AWS
resource "aws_key_pair" "ssh_key" {
  key_name   = var.ssh_key_name
  public_key = file(var.ssh_public_key_path)
}

# Debian 12
data "aws_ami" "debian12" {
  most_recent = true

  filter {
    name   = "name"
    values = ["debian-12-amd64-*"]
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }

  filter {
    name   = "root-device-type"
    values = ["ebs"]
  }

  owners = ["136693071363"] # https://wiki.debian.org/Cloud/AmazonEC2Image/
}

resource "aws_security_group" "ssh_access" {
  name        = "postgres-monitor-ssh-access"
  description = "Allow SSH inbound traffic for postgres monitor"

  ingress {
    description = "SSH from anywhere"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    description = "Allow all outbound traffic"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "postgres-monitor-ssh-access"
  }
}

resource "aws_instance" "postgres_monitor" {
  ami           = data.aws_ami.debian12.id
  instance_type = var.instance_type
  key_name      = var.ssh_key_name
  vpc_security_group_ids = [aws_security_group.ssh_access.id]

  user_data = templatefile("${path.module}/../cloudinit/postgres-monitor-cloud-init.yaml", {
    hostname           = var.hostname
    ssh_public_key     = trimspace(file(var.ssh_public_key_path))
    tailscale_auth_key = tailscale_tailnet_key.postgres_monitor.key
  })

  user_data_replace_on_change = true

  tags = {
    Name = var.instance_name
  }

  root_block_device {
    volume_size = 32
  }
}
