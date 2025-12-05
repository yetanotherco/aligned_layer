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
  }

  required_version = ">= 1.2"
}