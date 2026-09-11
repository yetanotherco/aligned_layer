# ============================================
# Provider Configuration
# ============================================
aws_region    = "us-east-2"
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
postgres_monitor_instance_name            = "agg-mode-mainnet-postgres-monitor"
postgres_monitor_hostname                 = "agg-mode-mainnet-postgres-monitor"
postgres_monitor_instance_type            = "t2.micro"
postgres_monitor_ssh_key_name             = "agg-mode-mainnet-postgres-monitor-key"
postgres_monitor_cloud_init_template_path = "../../cloudinit/mainnet-cloud-init.yaml"

# ============================================
# Postgres Primary Configuration (Elastic Metal)
# ============================================
postgres_primary_zone                     = "fr-par-2"
postgres_primary_offer_name               = "EM-I120E-NVME"
postgres_primary_server_name              = "agg-mode-mainnet-postgres-1"
postgres_primary_hostname                 = "agg-mode-mainnet-postgres-1"
postgres_primary_description              = "PostgreSQL server 1 for mainnet"
postgres_primary_ssh_key_name             = "agg-mode-mainnet-postgres-1-key"
postgres_primary_cloud_init_template_path = "../../cloudinit/mainnet-cloud-init.yaml"
postgres_primary_tags                     = ["postgres", "postgres-1", "mainnet"]

# ============================================
# Postgres Secondary Configuration (Elastic Metal)
# ============================================
postgres_secondary_zone                     = "nl-ams-1"
postgres_secondary_offer_name               = "EM-A610R-NVME"
postgres_secondary_server_name              = "agg-mode-mainnet-postgres-2"
postgres_secondary_hostname                 = "agg-mode-mainnet-postgres-2"
postgres_secondary_description              = "PostgreSQL server 2 for mainnet"
postgres_secondary_ssh_key_name             = "agg-mode-mainnet-postgres-2-key"
postgres_secondary_cloud_init_template_path = "../../cloudinit/mainnet-cloud-init.yaml"
postgres_secondary_tags                     = ["postgres", "postgres-2", "mainnet"]

# ============================================
# Gateway Primary Configuration (Elastic Metal)
# ============================================
gateway_primary_zone                     = "fr-par-2"
gateway_primary_offer_name               = "EM-I120E-NVME"
gateway_primary_server_name              = "agg-mode-mainnet-gateway-1"
gateway_primary_hostname                 = "agg-mode-mainnet-gateway-1"
gateway_primary_description              = "Gateway server 1 for mainnet"
gateway_primary_ssh_key_name             = "agg-mode-mainnet-gateway-1-key"
gateway_primary_cloud_init_template_path = "../../cloudinit/mainnet-cloud-init.yaml"
gateway_primary_tags                     = ["gateway", "gateway-1", "mainnet"]

# ============================================
# Gateway Secondary Configuration (Elastic Metal)
# ============================================
gateway_secondary_zone                     = "nl-ams-1"
gateway_secondary_offer_name               = "EM-A610R-NVME"
gateway_secondary_server_name              = "agg-mode-mainnet-gateway-2"
gateway_secondary_hostname                 = "agg-mode-mainnet-gateway-2"
gateway_secondary_description              = "Gateway server 2 for mainnet"
gateway_secondary_ssh_key_name             = "agg-mode-mainnet-gateway-2-key"
gateway_secondary_cloud_init_template_path = "../../cloudinit/mainnet-cloud-init.yaml"
gateway_secondary_tags                     = ["gateway", "gateway-2", "mainnet"]

# ============================================
# Metrics Configuration (Elastic Metal)
# ============================================
metrics_server_zone                     = "nl-ams-1"
metrics_server_offer_name               = "EM-A610R-NVME"
metrics_server_name                     = "agg-mode-mainnet-metrics"
metrics_server_hostname                 = "agg-mode-mainnet-metrics"
metrics_server_description              = "Metrics server for mainnet"
metrics_server_ssh_key_name             = "agg-mode-mainnet-metrics-key"
metrics_server_cloud_init_template_path = "../../cloudinit/mainnet-cloud-init.yaml"
metrics_server_tags                     = ["metrics", "mainnet"]

# ============================================
# Sender Configuration (Elastic Metal)
# ============================================
sender_server_zone                     = "nl-ams-1"
sender_server_offer_name               = "EM-A610R-NVME"
sender_server_name                     = "agg-mode-mainnet-sender"
sender_server_hostname                 = "agg-mode-mainnet-sender"
sender_server_description              = "Sender server for mainnet"
sender_server_ssh_key_name             = "agg-mode-mainnet-sender-key"
sender_server_cloud_init_template_path = "../../cloudinit/mainnet-cloud-init.yaml"
sender_server_tags                     = ["sender", "mainnet"]
