# ============================================
# Provider Configuration
# ============================================

variable "aws_region" {
  description = "AWS region"
  type        = string
}

variable "scaleway_zone" {
  description = "Default Scaleway zone"
  type        = string
}

# ============================================
# Common Configuration
# ============================================

variable "ssh_public_key_path_aws" {
  description = "Path to SSH public key for AWS"
  type        = string
}

variable "ssh_public_key_path_scaleway" {
  description = "Path to SSH public key for Scaleway"
  type        = string
}

variable "os_id" {
  description = "Scaleway OS ID for Debian 12"
  type        = string
  default     = "83640d93-a0b8-45ad-9c9f-30cae48380a4"
}

variable "subscription_period" {
  description = "Subscription period for bare metal servers"
  type        = string
  default     = "hourly"
}

variable "tailscale_key_expiry" {
  description = "Tailscale key expiry in seconds"
  type        = number
  default     = 3600
}

variable "tailscale_tags" {
  description = "Tailscale tags"
  type        = list(string)
  default     = ["tag:server"]
}

variable "common_tags" {
  description = "Common tags for all resources"
  type        = map(string)
  default     = {}
}

# ============================================
# Postgres Monitor Configuration (EC2)
# ============================================

variable "postgres_monitor_instance_name" {
  description = "Name of the postgres monitor EC2 instance"
  type        = string
}

variable "postgres_monitor_hostname" {
  description = "Hostname for postgres monitor"
  type        = string
}

variable "postgres_monitor_instance_type" {
  description = "EC2 instance type for postgres monitor"
  type        = string
}

variable "postgres_monitor_ssh_key_name" {
  description = "SSH key name for postgres monitor"
  type        = string
}

variable "postgres_monitor_cloud_init_template_path" {
  description = "Path to cloud-init template for postgres monitor"
  type        = string
}

variable "postgres_monitor_root_volume_size" {
  description = "Root volume size for postgres monitor in GB"
  type        = number
  default     = 32
}

# ============================================
# Postgres Primary Configuration (Elastic Metal)
# ============================================

variable "postgres_primary_zone" {
  description = "Scaleway zone for postgres primary"
  type        = string
}

variable "postgres_primary_offer_name" {
  description = "Bare metal offer name for postgres primary"
  type        = string
}

variable "postgres_primary_server_name" {
  description = "Server name for postgres primary"
  type        = string
}

variable "postgres_primary_hostname" {
  description = "Hostname for postgres primary"
  type        = string
}

variable "postgres_primary_description" {
  description = "Description for postgres primary"
  type        = string
}

variable "postgres_primary_ssh_key_name" {
  description = "SSH key name for postgres primary"
  type        = string
}

variable "postgres_primary_cloud_init_template_path" {
  description = "Path to cloud-init template for postgres primary"
  type        = string
}

variable "postgres_primary_tags" {
  description = "Tags for postgres primary"
  type        = list(string)
}

# ============================================
# Postgres Secondary Configuration (Elastic Metal)
# ============================================

variable "postgres_secondary_zone" {
  description = "Scaleway zone for postgres secondary"
  type        = string
}

variable "postgres_secondary_offer_name" {
  description = "Bare metal offer name for postgres secondary"
  type        = string
}

variable "postgres_secondary_server_name" {
  description = "Server name for postgres secondary"
  type        = string
}

variable "postgres_secondary_hostname" {
  description = "Hostname for postgres secondary"
  type        = string
}

variable "postgres_secondary_description" {
  description = "Description for postgres secondary"
  type        = string
}

variable "postgres_secondary_ssh_key_name" {
  description = "SSH key name for postgres secondary"
  type        = string
}

variable "postgres_secondary_cloud_init_template_path" {
  description = "Path to cloud-init template for postgres secondary"
  type        = string
}

variable "postgres_secondary_tags" {
  description = "Tags for postgres secondary"
  type        = list(string)
}

# ============================================
# Gateway Primary Configuration (Elastic Metal)
# ============================================

variable "gateway_primary_zone" {
  description = "Scaleway zone for gateway primary"
  type        = string
}

variable "gateway_primary_offer_name" {
  description = "Bare metal offer name for gateway primary"
  type        = string
}

variable "gateway_primary_server_name" {
  description = "Server name for gateway primary"
  type        = string
}

variable "gateway_primary_hostname" {
  description = "Hostname for gateway primary"
  type        = string
}

variable "gateway_primary_description" {
  description = "Description for gateway primary"
  type        = string
}

variable "gateway_primary_ssh_key_name" {
  description = "SSH key name for gateway primary"
  type        = string
}

variable "gateway_primary_cloud_init_template_path" {
  description = "Path to cloud-init template for gateway primary"
  type        = string
}

variable "gateway_primary_tags" {
  description = "Tags for gateway primary"
  type        = list(string)
}

# ============================================
# Gateway Secondary Configuration (Elastic Metal)
# ============================================

variable "gateway_secondary_zone" {
  description = "Scaleway zone for gateway secondary"
  type        = string
}

variable "gateway_secondary_offer_name" {
  description = "Bare metal offer name for gateway secondary"
  type        = string
}

variable "gateway_secondary_server_name" {
  description = "Server name for gateway secondary"
  type        = string
}

variable "gateway_secondary_hostname" {
  description = "Hostname for gateway secondary"
  type        = string
}

variable "gateway_secondary_description" {
  description = "Description for gateway secondary"
  type        = string
}

variable "gateway_secondary_ssh_key_name" {
  description = "SSH key name for gateway secondary"
  type        = string
}

variable "gateway_secondary_cloud_init_template_path" {
  description = "Path to cloud-init template for gateway secondary"
  type        = string
}

variable "gateway_secondary_tags" {
  description = "Tags for gateway secondary"
  type        = list(string)
}

# ============================================
# Metrics Configuration (Elastic Metal)
# ============================================

variable "metrics_server_zone" {
  description = "Scaleway zone for metrics server"
  type        = string
}

variable "metrics_server_offer_name" {
  description = "Bare metal offer name for metrics server"
  type        = string
}

variable "metrics_server_name" {
  description = "Server name for metrics server"
  type        = string
}

variable "metrics_server_hostname" {
  description = "Hostname for metrics server"
  type        = string
}

variable "metrics_server_description" {
  description = "Description for metrics server"
  type        = string
}

variable "metrics_server_ssh_key_name" {
  description = "SSH key name for metrics server"
  type        = string
}

variable "metrics_server_cloud_init_template_path" {
  description = "Path to cloud-init template for metrics server"
  type        = string
}

variable "metrics_server_tags" {
  description = "Tags for metrics server"
  type        = list(string)
}

# ============================================
# Sender Configuration (Elastic Metal)
# ============================================

variable "sender_server_zone" {
  description = "Scaleway zone for sender server"
  type        = string
}

variable "sender_server_offer_name" {
  description = "Bare metal offer name for sender server"
  type        = string
}

variable "sender_server_name" {
  description = "Server name for sender server"
  type        = string
}

variable "sender_server_hostname" {
  description = "Hostname for sender server"
  type        = string
}

variable "sender_server_description" {
  description = "Description for sender server"
  type        = string
}

variable "sender_server_ssh_key_name" {
  description = "SSH key name for sender server"
  type        = string
}

variable "sender_server_cloud_init_template_path" {
  description = "Path to cloud-init template for sender server"
  type        = string
}

variable "sender_server_tags" {
  description = "Tags for sender server"
  type        = list(string)
}
