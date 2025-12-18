variable "elastic_metal_zone" {
  description = "Scaleway zone"
  type        = string
}

variable "elastic_metal_offer_name" {
  description = "Name of the bare metal server offer"
  type        = string
}

variable "elastic_metal_subscription_period" {
  description = "Subscription period for the bare metal server"
  type        = string
  default     = "hourly"
}

variable "elastic_metal_server_name" {
  description = "Name of the bare metal server"
  type        = string
}

variable "elastic_metal_hostname" {
  description = "Hostname for the server"
  type        = string
}

variable "elastic_metal_description" {
  description = "Description of the server"
  type        = string
}

variable "elastic_metal_ssh_key_name" {
  description = "Name for the SSH key in Scaleway"
  type        = string
}

variable "elastic_metal_ssh_public_key_path" {
  description = "Path to the SSH public key file"
  type        = string
}

variable "elastic_metal_cloud_init_template_path" {
  description = "Path to the cloud-init template file"
  type        = string
}

variable "elastic_metal_os_id" {
  description = "Scaleway OS ID"
  type        = string
  default     = "83640d93-a0b8-45ad-9c9f-30cae48380a4"
}

variable "elastic_metal_tailscale_key_expiry" {
  description = "Expiry time for Tailscale auth key in seconds"
  type        = number
  default     = 3600
}

variable "elastic_metal_tailscale_tags" {
  description = "Tags to apply to the Tailscale key"
  type        = list(string)
  default     = ["tag:server"]
}

variable "elastic_metal_tags" {
  description = "Tags to apply to the server"
  type        = list(string)
  default     = []
}
