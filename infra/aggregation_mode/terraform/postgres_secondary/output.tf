output public_ip {
  description = "Public IP address of the Postgres Secondary."
  value       = scaleway_baremetal_server.postgres_secondary.ips
}