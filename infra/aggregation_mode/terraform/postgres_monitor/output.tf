output "instance_hostname" {
  description = "Private DNS name of the EC2 instance."
  value       = aws_instance.postgres_monitor.private_dns
}

output "public_ip" {
  description = "Public IP address of the EC2 instance."
  value       = aws_instance.postgres_monitor.public_ip
}
