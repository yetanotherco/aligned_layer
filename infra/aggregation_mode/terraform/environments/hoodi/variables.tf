# Provider Variables
variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "scaleway_zone" {
  description = "Default Scaleway zone"
  type        = string
  default     = "fr-par-2"
}

# variable "scaleway_project_id" {
#   description = "Scaleway project ID"
#   type        = string
# }

# variable "tailscale_api_key" {
#   description = "Tailscale API key"
#   type        = string
#   sensitive   = true
# }
#
# variable "tailscale_tailnet" {
#   description = "Tailscale tailnet"
#   type        = string
# }

# Common Variables
variable "ssh_public_key_path_aws" {
  description = "Path to SSH public key for AWS"
  type        = string
  default     = "~/.ssh/aws.pub"
}

variable "ssh_public_key_path_scaleway" {
  description = "Path to SSH public key for Scaleway"
  type        = string
  default     = "~/.ssh/scaleway.pem.pub"
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

# Postgres Monitor Variables (EC2)
variable "monitor_instance_name" {
  description = "Name of the postgres monitor EC2 instance"
  type        = string
  default     = "agg-mode-hoodi-postgres-monitor"
}

variable "monitor_hostname" {
  description = "Hostname for postgres monitor"
  type        = string
  default     = "agg-mode-hoodi-postgres-monitor"
}

variable "monitor_instance_type" {
  description = "EC2 instance type for postgres monitor"
  type        = string
  default     = "t2.micro"
}

variable "monitor_ssh_key_name" {
  description = "SSH key name for postgres monitor"
  type        = string
  default     = "agg-mode-hoodi-postgres-monitor-key"
}

variable "monitor_cloud_init_template_path" {
  description = "Path to cloud-init template for postgres monitor"
  type        = string
  default     = "../../cloudinit/postgres-monitor-cloud-init.yaml"
}

variable "monitor_root_volume_size" {
  description = "Root volume size for postgres monitor in GB"
  type        = number
  default     = 32
}

# Postgres Primary Variables (Elastic Metal)
variable "primary_zone" {
  description = "Scaleway zone for postgres primary"
  type        = string
  default     = "fr-par-2"
}

variable "primary_offer_name" {
  description = "Bare metal offer name for postgres primary"
  type        = string
  default     = "EM-A610R-NVME"
}

variable "primary_server_name" {
  description = "Server name for postgres primary"
  type        = string
  default     = "agg-mode-hoodi-postgres-1"
}

variable "primary_hostname" {
  description = "Hostname for postgres primary"
  type        = string
  default     = "agg-mode-hoodi-postgres-1"
}

variable "primary_description" {
  description = "Description for postgres primary"
  type        = string
  default     = "PostgreSQL server 1 for hoodi"
}

variable "primary_ssh_key_name" {
  description = "SSH key name for postgres primary"
  type        = string
  default     = "agg-mode-hoodi-postgres-1-key"
}

variable "primary_cloud_init_template_path" {
  description = "Path to cloud-init template for postgres primary"
  type        = string
  default     = "../../cloudinit/scaleway-cloud-init.yaml"
}

variable "primary_tags" {
  description = "Tags for postgres primary"
  type        = list(string)
  default     = ["postgres", "postgres-1", "hoodi"]
}

# Postgres Secondary Variables (Elastic Metal)
variable "secondary_zone" {
  description = "Scaleway zone for postgres secondary"
  type        = string
  default     = "nl-ams-1"
}

variable "secondary_offer_name" {
  description = "Bare metal offer name for postgres secondary"
  type        = string
  default     = "EM-A315X-SSD"
}

variable "secondary_server_name" {
  description = "Server name for postgres secondary"
  type        = string
  default     = "agg-mode-hoodi-postgres-2"
}

variable "secondary_hostname" {
  description = "Hostname for postgres secondary"
  type        = string
  default     = "agg-mode-hoodi-postgres-2"
}

variable "secondary_description" {
  description = "Description for postgres secondary"
  type        = string
  default     = "PostgreSQL server 2 for hoodi"
}

variable "secondary_ssh_key_name" {
  description = "SSH key name for postgres secondary"
  type        = string
  default     = "agg-mode-hoodi-postgres-2-key"
}

variable "secondary_cloud_init_template_path" {
  description = "Path to cloud-init template for postgres secondary"
  type        = string
  default     = "../../cloudinit/scaleway-cloud-init.yaml"
}

variable "secondary_tags" {
  description = "Tags for postgres secondary"
  type        = list(string)
  default     = ["postgres", "postgres-2", "hoodi"]
}
