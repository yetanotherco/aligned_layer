output public_ip {
  description = "Public IP address of the Postgres Primary."
  value       = scaleway_baremetal_server.postgres_primary.ips
}