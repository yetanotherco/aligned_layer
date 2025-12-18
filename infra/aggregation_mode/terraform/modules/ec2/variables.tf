variable "ec2_instance_name" {
  description = "Value of the EC2 instance's Name tag"
  type        = string
}

variable "ec2_hostname" {
  description = "The hostname to set for the EC2 instance"
  type        = string
}

variable "ec2_instance_type" {
  description = "The EC2 instance's type"
  type        = string
}

variable "ec2_ssh_key_name" {
  description = "The name of the SSH key pair to use for the EC2 instance"
  type        = string
}

variable "ec2_ssh_public_key_path" {
  description = "Path to the SSH public key file to upload to AWS"
  type        = string
}

variable "ec2_cloud_init_template_path" {
  description = "Path to the cloud-init template file"
  type        = string
}

variable "ec2_root_volume_size" {
  description = "Size of the root EBS volume in GB"
  type        = number
  default     = 32
}

variable "ec2_tailscale_key_expiry" {
  description = "Expiry time for Tailscale auth key in seconds"
  type        = number
  default     = 3600
}

variable "ec2_tailscale_tags" {
  description = "Tags to apply to the Tailscale key"
  type        = list(string)
  default     = ["tag:server"]
}

variable "ec2_tags" {
  description = "Additional tags to apply to the instance"
  type        = map(string)
  default     = {}
}
