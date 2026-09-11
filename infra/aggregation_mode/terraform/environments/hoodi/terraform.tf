terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.92"
    }
    scaleway = {
      source = "scaleway/scaleway"
      version = "2.64.0"
    }
    tailscale = {
      source = "tailscale/tailscale"
      version = "0.24.0"
    }
  }

  required_version = ">= 1.2"

  backend "s3" {
  }
}
