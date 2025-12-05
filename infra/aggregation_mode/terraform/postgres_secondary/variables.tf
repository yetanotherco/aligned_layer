variable "zone" {
  description = "Scaleway zone"
  type        = string
  default     = "nl-ams-1"
}

variable "offer_name" {
  description = "Name of the bare metal server offer"
  type        = string
  default     = "EM-A315X-SSD "
}

variable "server_name" {
  description = "Name of the bare metal server"
  type        = string
  default     = "postgres-secondary"
}

variable "hostname" {
  description = "Hostname for the server"
  type        = string
  default     = "postgres-secondary"
}

variable "description" {
  description = "Description of the server"
  type        = string
  default     = "PostgreSQL secondary server"
}

variable "ssh_key_name" {
  description = "Name for the SSH key in Scaleway"
  type        = string
  default     = "postgres-secondary-key"
}

variable "ssh_public_key_path" {
  description = "Path to the SSH public key file"
  type        = string
  default     = "~/.ssh/scaleway.pem.pub"
}

variable "tags" {
  description = "Tags to apply to the server"
  type        = list(string)
  default     = ["postgres", "secondary"]
}
