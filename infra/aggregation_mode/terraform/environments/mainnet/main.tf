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

  ec2_instance_name            = var.postgres_monitor_instance_name
  ec2_hostname                 = var.postgres_monitor_hostname
  ec2_instance_type            = var.postgres_monitor_instance_type
  ec2_ssh_key_name             = var.postgres_monitor_ssh_key_name
  ec2_ssh_public_key_path      = var.ssh_public_key_path_aws
  ec2_cloud_init_template_path = var.postgres_monitor_cloud_init_template_path
  ec2_root_volume_size         = var.postgres_monitor_root_volume_size
  ec2_tailscale_key_expiry     = var.tailscale_key_expiry
  ec2_tailscale_tags           = var.tailscale_tags

  ec2_tags = var.common_tags
}

# Postgres Primary (Elastic Metal / Scaleway Bare Metal)
module "postgres_primary" {
  source = "../../modules/elastic_metal"

  elastic_metal_zone                     = var.postgres_primary_zone
  elastic_metal_offer_name               = var.postgres_primary_offer_name
  elastic_metal_subscription_period      = var.subscription_period
  elastic_metal_server_name              = var.postgres_primary_server_name
  elastic_metal_hostname                 = var.postgres_primary_hostname
  elastic_metal_description              = var.postgres_primary_description
  elastic_metal_ssh_key_name             = var.postgres_primary_ssh_key_name
  elastic_metal_ssh_public_key_path      = var.ssh_public_key_path_scaleway
  elastic_metal_cloud_init_template_path = var.postgres_primary_cloud_init_template_path
  elastic_metal_os_id                    = var.os_id
  elastic_metal_tailscale_key_expiry     = var.tailscale_key_expiry
  elastic_metal_tailscale_tags           = var.tailscale_tags
  elastic_metal_tags                     = var.postgres_primary_tags
}

# Postgres Secondary (Elastic Metal / Scaleway Bare Metal)
module "postgres_secondary" {
  source = "../../modules/elastic_metal"

  elastic_metal_zone                     = var.postgres_secondary_zone
  elastic_metal_offer_name               = var.postgres_secondary_offer_name
  elastic_metal_subscription_period      = var.subscription_period
  elastic_metal_server_name              = var.postgres_secondary_server_name
  elastic_metal_hostname                 = var.postgres_secondary_hostname
  elastic_metal_description              = var.postgres_secondary_description
  elastic_metal_ssh_key_name             = var.postgres_secondary_ssh_key_name
  elastic_metal_ssh_public_key_path      = var.ssh_public_key_path_scaleway
  elastic_metal_cloud_init_template_path = var.postgres_secondary_cloud_init_template_path
  elastic_metal_os_id                    = var.os_id
  elastic_metal_tailscale_key_expiry     = var.tailscale_key_expiry
  elastic_metal_tailscale_tags           = var.tailscale_tags
  elastic_metal_tags                     = var.postgres_secondary_tags
}

# Gateway Primary (Elastic Metal / Scaleway Bare Metal)
module "gateway_primary" {
  source = "../../modules/elastic_metal"

  elastic_metal_zone                     = var.gateway_primary_zone
  elastic_metal_offer_name               = var.gateway_primary_offer_name
  elastic_metal_subscription_period      = var.subscription_period
  elastic_metal_server_name              = var.gateway_primary_server_name
  elastic_metal_hostname                 = var.gateway_primary_hostname
  elastic_metal_description              = var.gateway_primary_description
  elastic_metal_ssh_key_name             = var.gateway_primary_ssh_key_name
  elastic_metal_ssh_public_key_path      = var.ssh_public_key_path_scaleway
  elastic_metal_cloud_init_template_path = var.gateway_primary_cloud_init_template_path
  elastic_metal_os_id                    = var.os_id
  elastic_metal_tailscale_key_expiry     = var.tailscale_key_expiry
  elastic_metal_tailscale_tags           = var.tailscale_tags
  elastic_metal_tags                     = var.gateway_primary_tags
}

# Gateway Secondary (Elastic Metal / Scaleway Bare Metal)
module "gateway_secondary" {
  source = "../../modules/elastic_metal"

  elastic_metal_zone                     = var.gateway_secondary_zone
  elastic_metal_offer_name               = var.gateway_secondary_offer_name
  elastic_metal_subscription_period      = var.subscription_period
  elastic_metal_server_name              = var.gateway_secondary_server_name
  elastic_metal_hostname                 = var.gateway_secondary_hostname
  elastic_metal_description              = var.gateway_secondary_description
  elastic_metal_ssh_key_name             = var.gateway_secondary_ssh_key_name
  elastic_metal_ssh_public_key_path      = var.ssh_public_key_path_scaleway
  elastic_metal_cloud_init_template_path = var.gateway_secondary_cloud_init_template_path
  elastic_metal_os_id                    = var.os_id
  elastic_metal_tailscale_key_expiry     = var.tailscale_key_expiry
  elastic_metal_tailscale_tags           = var.tailscale_tags
  elastic_metal_tags                     = var.gateway_secondary_tags
}

# Metrics Server (Elastic Metal / Scaleway Bare Metal)
module "metrics" {
    source = "../../modules/elastic_metal"

    elastic_metal_zone                     = var.metrics_server_zone
    elastic_metal_offer_name               = var.metrics_server_offer_name
    elastic_metal_subscription_period      = var.subscription_period
    elastic_metal_server_name              = var.metrics_server_name
    elastic_metal_hostname                 = var.metrics_server_hostname
    elastic_metal_description              = var.metrics_server_description
    elastic_metal_ssh_key_name             = var.metrics_server_ssh_key_name
    elastic_metal_ssh_public_key_path      = var.ssh_public_key_path_scaleway
    elastic_metal_cloud_init_template_path = var.metrics_server_cloud_init_template_path
    elastic_metal_os_id                    = var.os_id
    elastic_metal_tailscale_key_expiry     = var.tailscale_key_expiry
    elastic_metal_tailscale_tags           = var.tailscale_tags
    elastic_metal_tags                     = var.metrics_server_tags
}

# Sender Server (Elastic Metal / Scaleway Bare Metal)
module "sender" {
    source = "../../modules/elastic_metal"

    elastic_metal_zone                     = var.sender_server_zone
    elastic_metal_offer_name               = var.sender_server_offer_name
    elastic_metal_subscription_period      = var.subscription_period
    elastic_metal_server_name              = var.sender_server_name
    elastic_metal_hostname                 = var.sender_server_hostname
    elastic_metal_description              = var.sender_server_description
    elastic_metal_ssh_key_name             = var.sender_server_ssh_key_name
    elastic_metal_ssh_public_key_path      = var.ssh_public_key_path_scaleway
    elastic_metal_cloud_init_template_path = var.sender_server_cloud_init_template_path
    elastic_metal_os_id                    = var.os_id
    elastic_metal_tailscale_key_expiry     = var.tailscale_key_expiry
    elastic_metal_tailscale_tags           = var.tailscale_tags
    elastic_metal_tags                     = var.sender_server_tags
}
