terraform {
  required_providers {
    scaleway = {
      source = "scaleway/scaleway"
    }
    tailscale = {
      source = "tailscale/tailscale"
    }
  }
}

# Create ephemeral Tailscale auth key
resource "tailscale_tailnet_key" "postgres_primary" {
  reusable      = false
  ephemeral     = true
  preauthorized = true
  expiry        = 3600
  description   = "Ephemeral key for postgres-primary"
  tags          = ["tag:server"]
}

# Get available bare metal offer
data "scaleway_baremetal_offer" "offer" {
  zone = var.zone
  name = var.offer_name
  subscription_period = "hourly"
}

# Get Debian 12 OS
data "scaleway_baremetal_os" "debian12" {
#   name    = "Debian"
#   version = "12"
  os_id = "83640d93-a0b8-45ad-9c9f-30cae48380a4"
}

# Upload SSH key
resource "scaleway_iam_ssh_key" "main" {
  name       = var.ssh_key_name
  public_key = file(var.ssh_public_key_path)
}

# Create bare metal server with hourly billing
resource "scaleway_baremetal_server" "postgres_primary" {
  name        = var.server_name
  offer       = data.scaleway_baremetal_offer.offer.offer_id
  zone        = var.zone
  description = var.description

  # Install OS
  os = data.scaleway_baremetal_os.debian12.os_id

  # Attach SSH key
  ssh_key_ids = [scaleway_iam_ssh_key.main.id]

  # Cloud-init configuration
#   cloud_init = templatefile("${path.module}/../cloudinit/scaleway-cloud-init.yaml", {
#     hostname           = var.hostname
#     ssh_public_key     = trimspace(file(var.ssh_public_key_path))
#     tailscale_auth_key = tailscale_tailnet_key.postgres_primary.key
#   })

  tags = var.tags
}
