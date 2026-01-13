terraform {
  required_providers {
    aws = {
      source = "hashicorp/aws"
    }
    tailscale = {
      source = "tailscale/tailscale"
    }
  }
}

# Create Tailscale auth key
resource "tailscale_tailnet_key" "this" {
  reusable      = false
  ephemeral     = false
  preauthorized = true
  expiry        = var.ec2_tailscale_key_expiry
  description   = "Auth key for ${var.ec2_instance_name}"
  tags          = var.ec2_tailscale_tags
}

# Upload existing SSH public key to AWS
resource "aws_key_pair" "ssh_key" {
  key_name   = var.ec2_ssh_key_name
  public_key = file(var.ec2_ssh_public_key_path)
}

# Debian 12 AMI
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

# Security group
resource "aws_security_group" "ssh_access" {
  name        = "${var.ec2_instance_name}-ssh-access"
  description = "Allow SSH inbound traffic for ${var.ec2_instance_name}"

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
    Name = "${var.ec2_instance_name}-ssh-access"
  }
}

# EC2 Instance
resource "aws_instance" "this" {
  ami           = data.aws_ami.debian12.id
  instance_type = var.ec2_instance_type
  key_name      = var.ec2_ssh_key_name
  vpc_security_group_ids = [aws_security_group.ssh_access.id]

  user_data = templatefile(var.ec2_cloud_init_template_path, {
    hostname           = var.ec2_hostname
    ssh_public_key     = trimspace(file(var.ec2_ssh_public_key_path))
    tailscale_auth_key = tailscale_tailnet_key.this.key
  })

  user_data_replace_on_change = true

  tags = merge(
    {
      Name = var.ec2_instance_name
    },
    var.ec2_tags
  )

  lifecycle {
    ignore_changes = [ami]
  }

  root_block_device {
    volume_size = var.ec2_root_volume_size
  }
}
