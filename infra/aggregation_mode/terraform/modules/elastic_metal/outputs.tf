output "server_id" {
  description = "ID of the bare metal server"
  value       = scaleway_baremetal_server.this.id
}

output "server_ip" {
  description = "IP address of the bare metal server"
  value       = scaleway_baremetal_server.this.ips[0].address
}
