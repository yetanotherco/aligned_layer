variable "ssh_key_name" {
  description = "The name of the SSH key pair to use for the EC2 instance."
  type        = string
  default     = "postgres-monitor-key"
}

variable "ssh_public_key_path" {
  description = "Path to the SSH public key file to upload to AWS."
  type        = string
  default     = "~/.ssh/aws.pub"
}

variable "instance_name" {
  description = "Value of the EC2 instance's Name tag."
  type        = string
  default     = "postgres-monitor"
}

variable "instance_type" {
  description = "The EC2 instance's type."
  type        = string
  default     = "t2.micro"
}

variable "hostname" {
  description = "The hostname to set for the EC2 instance."
  type        = string
  default     = "postgres-monitor"
}
