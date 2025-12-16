provider "aws" {
  region = "us-east-2"
}

provider "tailscale" {
  # Configure via environment variables:
  # TAILSCALE_API_KEY
}

module "postgres_monitor" {
  source = "./postgres_monitor"
}

module "postgres_primary" {
  source = "./postgres_primary"
}

# module "postgres_secondary" {
#   source = "./postgres_secondary"
# }
