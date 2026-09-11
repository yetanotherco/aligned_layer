# ============================================
# Provider Configuration
# ============================================
aws_region    = "us-east-1"
scaleway_zone = "fr-par-2"

# ============================================
# Common Configuration
# ============================================
ssh_public_key_path_aws      = "~/.ssh/aws.pub"
ssh_public_key_path_scaleway = "~/.ssh/scaleway.pem.pub"
subscription_period          = "monthly"

# ============================================
# Postgres Monitor Configuration (EC2)
# ============================================
postgres_monitor_instance_name            = "agg-mode-hoodi-postgres-monitor"
postgres_monitor_hostname                 = "agg-mode-hoodi-postgres-monitor"
postgres_monitor_instance_type            = "t2.micro"
postgres_monitor_ssh_key_name             = "agg-mode-hoodi-postgres-monitor-key"
postgres_monitor_cloud_init_template_path = "../../cloudinit/postgres-monitor-cloud-init.yaml"

# ============================================
# Postgres Primary Configuration (Elastic Metal)
# ============================================
postgres_primary_zone                     = "fr-par-2"
postgres_primary_offer_name               = "EM-A610R-NVME"
postgres_primary_server_name              = "agg-mode-hoodi-postgres-1"
postgres_primary_hostname                 = "agg-mode-hoodi-postgres-1"
postgres_primary_description              = "PostgreSQL server 1 for hoodi"
postgres_primary_ssh_key_name             = "agg-mode-hoodi-postgres-1-key"
postgres_primary_cloud_init_template_path = "../../cloudinit/scaleway-cloud-init.yaml"
postgres_primary_tags                     = ["postgres", "postgres-1", "hoodi"]

# ============================================
# Postgres Secondary Configuration (Elastic Metal)
# ============================================
postgres_secondary_zone                     = "nl-ams-1"
postgres_secondary_offer_name               = "EM-A610R-NVME"
postgres_secondary_server_name              = "agg-mode-hoodi-postgres-2"
postgres_secondary_hostname                 = "agg-mode-hoodi-postgres-2"
postgres_secondary_description              = "PostgreSQL server 2 for hoodi"
postgres_secondary_ssh_key_name             = "agg-mode-hoodi-postgres-2-key"
postgres_secondary_cloud_init_template_path = "../../cloudinit/scaleway-cloud-init.yaml"
postgres_secondary_tags                     = ["postgres", "postgres-2", "hoodi"]

# ============================================
# Gateway Primary Configuration (Elastic Metal)
# ============================================
gateway_primary_zone                     = "fr-par-2"
gateway_primary_offer_name               = "EM-A610R-NVME"
gateway_primary_server_name              = "agg-mode-hoodi-gateway-1"
gateway_primary_hostname                 = "agg-mode-hoodi-gateway-1"
gateway_primary_description              = "Gateway server 1 for hoodi"
gateway_primary_ssh_key_name             = "agg-mode-hoodi-gateway-1-key"
gateway_primary_cloud_init_template_path = "../../cloudinit/scaleway-cloud-init.yaml"
gateway_primary_tags                     = ["gateway", "gateway-1", "hoodi"]

# ============================================
# Gateway Secondary Configuration (Elastic Metal)
# ============================================
gateway_secondary_zone                     = "nl-ams-1"
gateway_secondary_offer_name               = "EM-A610R-NVME"
gateway_secondary_server_name              = "agg-mode-hoodi-gateway-2"
gateway_secondary_hostname                 = "agg-mode-hoodi-gateway-2"
gateway_secondary_description              = "Gateway server 2 for hoodi"
gateway_secondary_ssh_key_name             = "agg-mode-hoodi-gateway-2-key"
gateway_secondary_cloud_init_template_path = "../../cloudinit/scaleway-cloud-init.yaml"
gateway_secondary_tags                     = ["gateway", "gateway-2", "hoodi"]

# ============================================
# Metrics Configuration (Elastic Metal)
# ============================================
metrics_server_zone                     = "fr-par-2"
metrics_server_offer_name               = "EM-A610R-NVME"
metrics_server_name                     = "agg-mode-hoodi-metrics"
metrics_server_hostname                 = "agg-mode-hoodi-metrics"
metrics_server_description              = "Metrics server for hoodi"
metrics_server_ssh_key_name             = "agg-mode-hoodi-metrics-key"
metrics_server_cloud_init_template_path = "../../cloudinit/cloud-init.yaml"
metrics_server_tags                     = ["metrics", "hoodi"]
