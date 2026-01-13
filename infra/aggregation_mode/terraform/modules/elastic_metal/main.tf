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

# Create Tailscale auth key
resource "tailscale_tailnet_key" "this" {
  reusable      = false
  ephemeral     = false
  preauthorized = true
  expiry        = var.elastic_metal_tailscale_key_expiry
  description   = "Auth key for ${var.elastic_metal_server_name}"
  tags          = var.elastic_metal_tailscale_tags
}

# Get available bare metal offer
data "scaleway_baremetal_offer" "offer" {
  zone                = var.elastic_metal_zone
  name                = var.elastic_metal_offer_name
  subscription_period = var.elastic_metal_subscription_period
}

# Get Debian 12 OS
data "scaleway_baremetal_os" "debian12" {
  os_id = var.elastic_metal_os_id
}

# Upload SSH key
resource "scaleway_iam_ssh_key" "main" {
  name       = var.elastic_metal_ssh_key_name
  public_key = file(var.elastic_metal_ssh_public_key_path)
}

# Create bare metal server with hourly billing
resource "scaleway_baremetal_server" "this" {
  name        = var.elastic_metal_server_name
  offer       = data.scaleway_baremetal_offer.offer.offer_id
  zone        = var.elastic_metal_zone
  description = var.elastic_metal_description

  # Install OS
  os = data.scaleway_baremetal_os.debian12.os_id

  # Attach SSH key
  ssh_key_ids = [scaleway_iam_ssh_key.main.id]

  # Cloud-init configuration
  cloud_init = templatefile(var.elastic_metal_cloud_init_template_path, {
    hostname           = var.elastic_metal_hostname
    ssh_public_key     = trimspace(file(var.elastic_metal_ssh_public_key_path))
    tailscale_auth_key = tailscale_tailnet_key.this.key
  })

  tags = var.elastic_metal_tags
}
