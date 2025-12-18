# AWS Provider Configuration
provider "aws" {
  region = var.aws_region
}

# Scaleway Provider Configuration
provider "scaleway" {
}

# Tailscale Provider Configuration
provider "tailscale" {
}

# Postgres Monitor (EC2)
module "postgres_monitor" {
  source = "../../modules/ec2"

  ec2_instance_name            = var.monitor_instance_name
  ec2_hostname                 = var.monitor_hostname
  ec2_instance_type            = var.monitor_instance_type
  ec2_ssh_key_name             = var.monitor_ssh_key_name
  ec2_ssh_public_key_path      = var.ssh_public_key_path_aws
  ec2_cloud_init_template_path = var.monitor_cloud_init_template_path
  ec2_root_volume_size         = var.monitor_root_volume_size
  ec2_tailscale_key_expiry     = var.tailscale_key_expiry
  ec2_tailscale_tags           = var.tailscale_tags

  ec2_tags = var.common_tags
}

# Postgres Primary (Elastic Metal / Scaleway Bare Metal)
module "postgres_primary" {
  source = "../../modules/elastic_metal"

  elastic_metal_zone                     = var.primary_zone
  elastic_metal_offer_name               = var.primary_offer_name
  elastic_metal_subscription_period      = var.subscription_period
  elastic_metal_server_name              = var.primary_server_name
  elastic_metal_hostname                 = var.primary_hostname
  elastic_metal_description              = var.primary_description
  elastic_metal_ssh_key_name             = var.primary_ssh_key_name
  elastic_metal_ssh_public_key_path      = var.ssh_public_key_path_scaleway
  elastic_metal_cloud_init_template_path = var.primary_cloud_init_template_path
  elastic_metal_os_id                    = var.os_id
  elastic_metal_tailscale_key_expiry     = var.tailscale_key_expiry
  elastic_metal_tailscale_tags           = var.tailscale_tags
  elastic_metal_tags                     = var.primary_tags
}

# Postgres Secondary (Elastic Metal / Scaleway Bare Metal)
module "postgres_secondary" {
  source = "../../modules/elastic_metal"

  elastic_metal_zone                     = var.secondary_zone
  elastic_metal_offer_name               = var.secondary_offer_name
  elastic_metal_subscription_period      = var.subscription_period
  elastic_metal_server_name              = var.secondary_server_name
  elastic_metal_hostname                 = var.secondary_hostname
  elastic_metal_description              = var.secondary_description
  elastic_metal_ssh_key_name             = var.secondary_ssh_key_name
  elastic_metal_ssh_public_key_path      = var.ssh_public_key_path_scaleway
  elastic_metal_cloud_init_template_path = var.secondary_cloud_init_template_path
  elastic_metal_os_id                    = var.os_id
  elastic_metal_tailscale_key_expiry     = var.tailscale_key_expiry
  elastic_metal_tailscale_tags           = var.tailscale_tags
  elastic_metal_tags                     = var.secondary_tags
}
