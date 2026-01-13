# Postgres Monitor Outputs
output "postgres_monitor_name" {
  description = "Name of the postgres monitor EC2 instance"
  value       = var.postgres_monitor_instance_name
}

output "postgres_monitor_hostname" {
  description = "Private DNS name of the postgres monitor EC2 instance"
  value       = module.postgres_monitor.instance_hostname
}

output "postgres_monitor_public_ip" {
  description = "Public IP address of the postgres monitor EC2 instance"
  value       = module.postgres_monitor.public_ip
}

# Postgres Primary Outputs
output "postgres_primary_server_name" {
  description = "Name of the postgres primary bare metal server"
  value       = var.postgres_primary_server_name
}

output "postgres_primary_server_id" {
  description = "ID of the postgres primary bare metal server"
  value       = module.postgres_primary.server_id
}

output "postgres_primary_server_ip" {
  description = "IP address of the postgres primary bare metal server"
  value       = module.postgres_primary.server_ip
}

# Postgres Secondary Outputs
output "postgres_secondary_server_name" {
  description = "Name of the postgres secondary bare metal server"
  value       = var.postgres_secondary_server_name
}

output "postgres_secondary_server_id" {
  description = "ID of the postgres secondary bare metal server"
  value       = module.postgres_secondary.server_id
}

output "postgres_secondary_server_ip" {
  description = "IP address of the postgres secondary bare metal server"
  value       = module.postgres_secondary.server_ip
}

# Gateway Primary Outputs
output "gateway_primary_server_name" {
  description = "Name of the gateway primary bare metal server"
  value       = var.gateway_primary_server_name
}

output "gateway_primary_server_id" {
  description = "ID of the gateway primary bare metal server"
  value       = module.gateway_primary.server_id
}

output "gateway_primary_server_ip" {
  description = "IP address of the gateway primary bare metal server"
  value       = module.gateway_primary.server_ip
}

# Gateway Secondary Outputs
output "gateway_secondary_server_name" {
  description = "Name of the gateway secondary bare metal server"
  value       = var.gateway_secondary_server_name
}

output "gateway_secondary_server_id" {
  description = "ID of the gateway secondary bare metal server"
  value       = module.gateway_secondary.server_id
}

output "gateway_secondary_server_ip" {
  description = "IP address of the gateway secondary bare metal server"
  value       = module.gateway_secondary.server_ip
}

# Metrics Server Outputs
output "metrics_server_name" {
  description = "Name of the metrics bare metal server"
  value       = var.metrics_server_name
}

output "metrics_server_id" {
  description = "ID of the metrics bare metal server"
  value       = module.metrics.server_id
}

output "metrics_server_ip" {
  description = "IP address of the metrics bare metal server"
  value       = module.metrics.server_ip
}

# Sender Server Outputs
output "sender_server_name" {
  description = "Name of the sender bare metal server"
  value       = var.sender_server_name
}

output "sender_server_id" {
  description = "ID of the sender bare metal server"
  value       = module.sender.server_id
}

output "sender_server_ip" {
  description = "IP address of the sender bare metal server"
  value       = module.sender.server_ip
}
