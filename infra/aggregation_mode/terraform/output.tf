output "postgres_monitor_public_ip" {
  description = "Public IP address of the Postgres Monitor."
  value       = module.postgres_monitor.public_ip
}

# output "postgres_primary_public_ip" {
#   description = "Public IP address of the Postgres Primary."
#   value       = module.postgres_primary.public_ip
# }
#
# output "postgres_secondary_public_ip" {
#   description = "Public IP address of the Postgres Secondary."
#   value       = module.postgres_secondary.public_ip
# }
