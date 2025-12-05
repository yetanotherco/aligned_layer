variable "zone" {
  description = "Scaleway zone"
  type        = string
  default     = "fr-par-2"
}

variable "offer_name" {
  description = "Name of the bare metal server offer"
  type        = string
  default     = "EM-A116X-SSD"
}

variable "server_name" {
  description = "Name of the bare metal server"
  type        = string
  default     = "postgres-primary"
}

variable "hostname" {
  description = "Hostname for the server"
  type        = string
  default     = "postgres-primary"
}

variable "description" {
  description = "Description of the server"
  type        = string
  default     = "PostgreSQL primary server"
}

variable "ssh_key_name" {
  description = "Name for the SSH key in Scaleway"
  type        = string
  default     = "postgres-primary-key"
}

variable "ssh_public_key_path" {
  description = "Path to the SSH public key file"
  type        = string
  default     = "~/.ssh/scaleway.pem.pub"
}

variable "tags" {
  description = "Tags to apply to the server"
  type        = list(string)
  default     = ["postgres", "primary"]
}
