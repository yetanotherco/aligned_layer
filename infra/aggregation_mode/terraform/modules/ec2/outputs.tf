output "instance_hostname" {
  description = "Private DNS name of the EC2 instance"
  value       = aws_instance.this.private_dns
}

output "public_ip" {
  description = "Public IP address of the EC2 instance"
  value       = aws_instance.this.public_ip
}
