# Postgres Monitor Outputs
output "monitor_hostname" {
  description = "Private DNS name of the postgres monitor EC2 instance"
  value       = module.postgres_monitor.instance_hostname
}

output "monitor_public_ip" {
  description = "Public IP address of the postgres monitor EC2 instance"
  value       = module.postgres_monitor.public_ip
}

# Postgres Primary Outputs
output "primary_server_id" {
  description = "ID of the postgres primary bare metal server"
  value       = module.postgres_primary.server_id
}

output "primary_server_ip" {
  description = "IP address of the postgres primary bare metal server"
  value       = module.postgres_primary.server_ip
}

# Postgres Secondary Outputs
output "secondary_server_id" {
  description = "ID of the postgres secondary bare metal server"
  value       = module.postgres_secondary.server_id
}

output "secondary_server_ip" {
  description = "IP address of the postgres secondary bare metal server"
  value       = module.postgres_secondary.server_ip
}
