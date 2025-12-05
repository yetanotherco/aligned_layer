output "postgres_monitor_instance_hostname" {
  description = "Private DNS name of the EC2 instance."
  value       = module.postgres_monitor.instance_hostname
}

output "postgres_monitor_public_ip" {
  description = "Public IP address of the EC2 instance."
  value       = module.postgres_monitor.public_ip
}
