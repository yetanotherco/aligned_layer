variable "zone" {
  description = "Scaleway zone"
  type        = string
  default     = "fr-par-2"
}

variable "offer_name" {
  description = "Name of the bare metal server offer"
  type        = string
  default     = "EM-A610R-NVME" # Set correct server offer
}

variable "server_name" {
  description = "Name of the bare metal server"
  type        = string
  default     = "postgres-1"
}

variable "hostname" {
  description = "Hostname for the server"
  type        = string
  default     = "postgres-1"
}

variable "description" {
  description = "Description of the server"
  type        = string
  default     = "PostgreSQL server 1"
}

variable "ssh_key_name" {
  description = "Name for the SSH key in Scaleway"
  type        = string
  default     = "postgres-1-key"
}

variable "ssh_public_key_path" {
  description = "Path to the SSH public key file"
  type        = string
  default     = "~/.ssh/scaleway.pem.pub"
}

variable "tags" {
  description = "Tags to apply to the server"
  type        = list(string)
  default     = ["postgres"]
}
