# Aggregation Mode Ansible Automation

This directory contains Ansible playbooks and configuration for automating the deployment and management of the Aligned Layer aggregation mode infrastructure.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Initial Setup](#initial-setup)
- [Deployment](#deployment)
- [Service Management](#service-management)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)
- [Advanced Usage](#advanced-usage)

## Overview

The Ansible automation deploys a complete aggregation mode stack consisting of:

1. **PostgreSQL Auto-Failover Cluster** (3 servers)
   - 1 Monitor node (EC2)
   - 2 Data nodes (Primary + Secondary) with automatic failover (Scaleway Elastic Metal)
   - Password authentication with scram-sha-256

2. **Gateway Service** (2 servers)
   - Rust-based gateway with TLS support
   - Runs on port 8080 (non-TLS) and port 443 (TLS)
   - Systemd service with automatic restart

3. **Poller Service** (2 servers, colocated with gateway)
   - Payment poller service
   - User-level systemd service

4. **Metrics Stack** (1 server)
   - Prometheus for metrics collection
   - Grafana for visualization
   - 90-day retention

5. **Task Sender** (1 server)
   - Automated proof submission service
   - Runs continuously in tmux session
   - Configurable interval and proof files

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Tailscale VPN                            │
│                     (100.64.0.0/10)                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐           │
│  │  PG Monitor  │  │ PG Primary   │  │ PG Secondary │           │
│  │   (EC2)      │  │  (Scaleway)  │  │  (Scaleway)  │           │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘           │
│         │                  │                  │                 │
│         └──────────────────┴──────────────────┘                 │
│                    pg_auto_failover                             │
│                                                                 │
│  ┌────────────────────────┐  ┌────────────────────────┐         │
│  │  Gateway Primary       │  │  Gateway Secondary     │         │
│  │  ├─ Gateway (8080+443)│  │  ├─ Gateway (8080+443) │         │
│  │  └─ Poller            │  │  └─ Poller             │         │
│  └────────────────────────┘  └────────────────────────┘         │
│                                                                 │
│  ┌────────────────────────┐                                     │
│  │  Metrics Server        │                                     │
│  │  ├─ Prometheus (9090)  │                                     │
│  │  └─ Grafana (3000)     │                                     │
│  └────────────────────────┘                                     │
│                                                                 │
│  ┌────────────────────────┐                                     │
│  │  Task Sender           │                                     │
│  │  (tmux session)        │                                     │
│  └────────────────────────┘                                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Prerequisites

### Local Machine

1. **Ansible** (version 2.9 or higher)
   ```bash
   pip install ansible
   ```

2. **SSH access** to all servers via Tailscale
   - Ensure you're connected to the Tailscale VPN
   - SSH keys configured for `admin` user on all servers

3. **TLS Certificates** for Gateway
   - Valid TLS certificate and key files
   - Can be Let's Encrypt, CA-issued, or self-signed

### Remote Servers

All servers are provisioned via Terraform and connected via Tailscale VPN. They should have:
- Ubuntu/Debian-based OS
- `admin` user with sudo privileges
- `app` user for application services (gateway servers)
- `postgres` user will be created automatically for PostgreSQL services
- Tailscale VPN configured

## Initial Setup

All configuration is consolidated into environment-specific files with predefined values. You only need to fill in sensitive values at the top of each config file.

### Configure Environment

Edit the config file for your environment:
- **Hoodi**: `playbooks/ini/config-hoodi.ini`
- **Mainnet**: `playbooks/ini/config-mainnet.ini`

All non-sensitive values are already pre-filled. Fill in the required values at the top of the file:

```ini
# ============================================
# REQUIRED: Sensitive Values (fill these in)
# ============================================
# Database password (used by postgres, gateway, and poller)
db_password=your_secure_password_here

# Grafana read-only database user password
grafana_postgres_password=your_secure_password_here

# TLS certificates (local paths to copy from)
tls_cert_source_path=/path/to/your/cert.pem
tls_key_source_path=/path/to/your/key.pem

# Grafana admin password
grafana_admin_password=your_grafana_admin_password

# Task sender private key (for sending proofs)
task_sender_private_key=0xYourPrivateKeyHere
```

## Deployment

### Full Stack Deployment

To deploy everything in one command:

```bash
make agg_mode_deploy_all ENV=hoodi
```

This will:
1. Deploy PostgreSQL cluster (monitor, primary, secondary)
2. Run database migrations
3. Deploy gateway and poller on both servers
4. Deploy Prometheus and Grafana
5. Deploy task sender

### Step-by-Step Deployment

For more control, deploy each component separately:

#### 1. Deploy PostgreSQL Cluster

```bash
# Deploy complete postgres cluster with password authentication
make postgres_deploy ENV=hoodi
```

This will:
- Deploy monitor with scram-sha-256 auth
- Set password for autoctl_node user
- Deploy primary and secondary nodes
- Configure replication with password auth
- Run database migrations

**Verify cluster status:**
```bash
make postgres_status ENV=hoodi
```

Expected output:
```
  Name    |  Node |     Host:Port      |       TLI: LSN |   Connection |      Reported State |      Assigned State
----------+-------+--------------------+----------------+--------------+---------------------+--------------------
monitor   |   1   | 100.x.x.x:5432     |                |              |                     |
node_1    |   2   | 100.x.x.x:5432     |      1: 0/...  | read-write   | primary             | primary
node_2    |   3   | 100.x.x.x:5432     |      1: 0/...  | read-only    | secondary           | secondary
```

#### 2. Deploy Gateway & Poller

```bash
# Deploy on both servers
make gateway_deploy ENV=hoodi

# Or deploy individually
make gateway_primary_deploy ENV=hoodi
make gateway_secondary_deploy ENV=hoodi

# Force rebuild (always rebuild binaries from latest code)
make gateway_deploy ENV=hoodi FORCE_REBUILD=true
```

**Note:** By default, the deployment is idempotent and skips building if the binary already exists. Use `FORCE_REBUILD=true` to always rebuild from the latest code in the repository.

**Verify gateway is running:**
```bash
ssh app@agg-mode-hoodi-gateway-1 "sudo systemctl status gateway"
ssh app@agg-mode-hoodi-gateway-1 "systemctl --user status poller"
```

**Test endpoint:**
```bash
curl -k https://agg-mode-hoodi-gateway-1/health
```

#### 3. Deploy Metrics Stack

```bash
# Deploy both Prometheus and Grafana
make metrics_deploy ENV=hoodi

# Or deploy individually
make prometheus_deploy ENV=hoodi
make grafana_deploy ENV=hoodi
```

**Access dashboards:**
- Prometheus: `http://<metrics-server-ip>:9090`
- Grafana: `http://<metrics-server-ip>:3000` (default credentials: admin/admin)

#### 4. Deploy Task Sender

```bash
# Deploy task sender
make task_sender_deploy ENV=hoodi
```

The task sender runs in a tmux session and continuously sends proofs to the network at the configured interval (default: 1 hour).

**Automatic Deposit Check:**

The deployment automatically:
1. Derives the wallet address from the configured private key
2. Checks if the address has an active subscription on the payment contract
3. If not subscribed or expired, automatically deposits 0.0035 ETH to the payment contract
4. Waits for transaction confirmation before starting the task sender

**Requirements:**
- The account must have sufficient ETH for:
  - Payment deposit: **0.0035 ETH**
  - Gas fees: ~**0.001 ETH** (estimated)
- Foundry (cast) will be automatically installed if not present

**Verify task sender is running:**
```bash
make task_sender_status ENV=hoodi
```

**View task sender logs:**
```bash
# Show how to view logs
make task_sender_logs ENV=hoodi

# Or directly attach to the tmux session
ssh app@agg-mode-hoodi-sender 'tmux attach -t task_sender'
# Press Ctrl+B then D to detach without stopping
```

**Restart task sender:**
```bash
make task_sender_restart ENV=hoodi
```

## Service Management

### Restart Services

**Gateway:**
```bash
make gateway_restart ENV=hoodi HOST=gateway_primary
make gateway_restart ENV=hoodi HOST=gateway_secondary
```

**Poller:**
```bash
make poller_restart ENV=hoodi HOST=gateway_primary
make poller_restart ENV=hoodi HOST=gateway_secondary
```

**Task Sender:**
```bash
make task_sender_restart ENV=hoodi
```

### Check Service Status

**PostgreSQL Cluster:**
```bash
make postgres_status ENV=hoodi
```

**Gateway:**
```bash
ssh app@agg-mode-hoodi-gateway-1 "sudo systemctl status gateway"
ssh app@agg-mode-hoodi-gateway-1 "sudo journalctl -u gateway -n 50"
```

**Poller:**
```bash
ssh app@agg-mode-hoodi-gateway-1 "systemctl --user status poller"
ssh app@agg-mode-hoodi-gateway-1 "journalctl --user -u poller -n 50"
```

**Prometheus:**
```bash
ssh admin@agg-mode-hoodi-metrics "systemctl --user status prometheus"
```

**Grafana:**
```bash
ssh admin@agg-mode-hoodi-metrics "sudo systemctl status grafana-server"
```

**Task Sender:**
```bash
make task_sender_status ENV=hoodi
# Or check tmux session directly
ssh app@agg-mode-hoodi-sender "tmux has-session -t task_sender && echo 'Running' || echo 'Not running'"
```

### View Logs

**Gateway:**
```bash
ssh app@agg-mode-hoodi-gateway-1 "sudo journalctl -u gateway -f"
```

**Poller:**
```bash
ssh app@agg-mode-hoodi-gateway-1 "journalctl --user -u poller -f"
```

**PostgreSQL:**
```bash
ssh admin@agg-mode-hoodi-postgres-1 "sudo journalctl -u pgautofailover -f"
```

**Task Sender:**
```bash
# Attach to tmux session to view live logs
ssh app@agg-mode-hoodi-sender 'tmux attach -t task_sender'
# Press Ctrl+B then D to detach

# Or capture current pane output
ssh app@agg-mode-hoodi-sender 'tmux capture-pane -t task_sender -p'
```

## Verification

### PostgreSQL Cluster Health

1. **Check cluster state:**
   ```bash
   make postgres_status ENV=hoodi
   ```

2. **Test password authentication:**
   ```bash
   ssh admin@agg-mode-hoodi-postgres-1 "PGPASSWORD='your_password' psql -U autoctl_node -h localhost -d agg_mode -c 'SELECT 1'"
   ```

3. **Verify replication:**
   ```bash
   ssh admin@agg-mode-hoodi-postgres-1 "sudo -u postgres psql -d agg_mode -c 'SELECT * FROM pg_stat_replication'"
   ```

4. **Test failover (optional):**
   ```bash
   # Stop primary
   ssh admin@agg-mode-hoodi-postgres-1 "sudo systemctl stop pgautofailover"

   # Wait 30 seconds, check status
   make postgres_status ENV=hoodi
   # Secondary should now be primary

   # Restart original primary
   ssh admin@agg-mode-hoodi-postgres-1 "sudo systemctl start pgautofailover"
   ```

### Gateway Health

1. **Check HTTP health endpoint:**
   ```bash
   curl -k https://agg-mode-hoodi-gateway-1/health
   ```

2. **Check metrics:**
   ```bash
   curl http://agg-mode-hoodi-gateway-1:9094/metrics
   ```

3. **Verify database connectivity:**
   ```bash
   ssh app@agg-mode-hoodi-gateway-1
   PGPASSWORD='your_password' psql -U autoctl_node -h agg-mode-hoodi-postgres-1 -d agg_mode -c "SELECT 1"
   ```

### Poller Health

1. **Check last processed block:**
   ```bash
   ssh app@agg-mode-hoodi-gateway-1 "cat ~/config/proof-aggregator.last_block_fetched.json"
   ```

   The block number should increase over time.

2. **Check metrics:**
   ```bash
   curl http://agg-mode-hoodi-gateway-1:9095/metrics
   ```

### Metrics Stack

1. **Prometheus targets:**
   - Navigate to `http://<metrics-ip>:9090/targets`
   - All targets should show as "UP"

2. **Grafana datasources:**
   - Navigate to `http://<metrics-ip>:3000`
   - Go to Configuration → Data Sources
   - Verify Prometheus and PostgreSQL datasources are connected

### Task Sender

1. **Check tmux session is running:**
   ```bash
   make task_sender_status ENV=hoodi
   ```

2. **View recent logs:**
   ```bash
   ssh app@agg-mode-hoodi-sender 'tmux capture-pane -t task_sender -p'
   ```

3. **Verify proof submissions:**
   - Check logs for successful proof submissions
   - Look for transaction hashes in the output
   - Verify proofs are appearing on the network

## Troubleshooting

### PostgreSQL Issues

**Problem: Node fails to join cluster**

Check monitor logs:
```bash
ssh admin@agg-mode-hoodi-postgres-monitor "sudo journalctl -u pgautofailover -n 100"
```

Check node logs:
```bash
ssh admin@agg-mode-hoodi-postgres-1 "sudo journalctl -u pgautofailover -n 100"
```

**Problem: Password authentication fails**

Verify `db_password` is set correctly in your environment config file (`config-hoodi.ini` or `config-mainnet.ini`).

Check pg_hba.conf:
```bash
ssh admin@agg-mode-hoodi-postgres-1 "sudo -u postgres cat /var/lib/postgresql/node/pg_hba.conf"
```

Should contain:
```
host    all             all             100.64.0.0/10           scram-sha-256
```

### Gateway Issues

**Problem: Gateway won't start**

Check logs for errors:
```bash
ssh app@agg-mode-hoodi-gateway-1 "sudo journalctl -u gateway -n 100"
```

Common issues:
- Missing TLS certificates → Check paths in `config-{{ env }}.ini` (tls_cert_source_path, tls_key_source_path)
- Database connection failed → Verify `db_password` in `config-{{ env }}.ini`
- Port 443 already in use → Check with `sudo lsof -i :443`

**Problem: TLS certificate errors**

Verify certificates exist:
```bash
ssh app@agg-mode-hoodi-gateway-1 "ls -la ~/.ssl/"
```

Check certificate validity:
```bash
ssh app@agg-mode-hoodi-gateway-1 "openssl x509 -in ~/.ssl/cert.pem -text -noout"
```

### Poller Issues

**Problem: Poller not syncing blocks**

Check logs:
```bash
ssh app@agg-mode-hoodi-gateway-1 "journalctl --user -u poller -n 100"
```

Verify RPC connectivity:
```bash
ssh app@agg-mode-hoodi-gateway-1 "curl -X POST -H 'Content-Type: application/json' --data '{\"jsonrpc\":\"2.0\",\"method\":\"eth_blockNumber\",\"params\":[],\"id\":1}' https://aligned-hoodi-rpc-geth.tail665ae.ts.net"
```

### Metrics Issues

**Problem: Prometheus not scraping targets**

Check Prometheus logs:
```bash
ssh admin@agg-mode-hoodi-metrics "journalctl --user -u prometheus -n 100"
```

Verify targets are reachable from metrics server:
```bash
ssh admin@agg-mode-hoodi-metrics "curl http://agg-mode-hoodi-gateway-1:9094/metrics"
```

Check Prometheus config:
```bash
ssh admin@agg-mode-hoodi-metrics "cat ~/config/prometheus.yaml"
```

### Task Sender Issues

**Problem: Task sender not running**

Check if tmux session exists:
```bash
ssh app@agg-mode-hoodi-sender "tmux list-sessions"
```

If missing, redeploy:
```bash
make task_sender_deploy ENV=hoodi
```

**Problem: Task sender crashes or exits**

Check logs for errors:
```bash
ssh app@agg-mode-hoodi-sender 'tmux capture-pane -t task_sender -p -S -100'
```

Common issues:
- Invalid private key → Check `task_sender_private_key` in `config-{{ env }}.ini`
- Missing proof/vk files → Verify files exist: `task_sender_proof_path`, `task_sender_vk_path`
- Network connectivity → Test RPC: `curl https://aligned-hoodi-rpc-geth.tail665ae.ts.net`
- Insufficient balance → Check account has ETH for gas fees

**Problem: Proofs not being submitted**

Check interval configuration:
```bash
ssh app@agg-mode-hoodi-sender "cat ~/repos/sender/aligned_layer/scripts/.agg_mode.task_sender.env"
```

Verify `INTERVAL_HOURS` is set correctly (default: 1 hour). Attach to session to see live activity:
```bash
ssh app@agg-mode-hoodi-sender 'tmux attach -t task_sender'
```

**Problem: Deployment fails with insufficient balance**

The automatic deposit check requires the account to have at least **0.0045 ETH** (0.0035 for deposit + ~0.001 for gas).

Check account balance:
```bash
ssh app@agg-mode-hoodi-sender
export PATH=$HOME/.foundry/bin:$PATH
cast balance <YOUR_WALLET_ADDRESS> --rpc-url <RPC_URL>
```

If balance is insufficient, send ETH to the account and redeploy:
```bash
make task_sender_deploy ENV=hoodi
```

**Problem: Automatic deposit fails**

If the automatic deposit fails during deployment, check the Ansible output for error messages. Common issues:
- Insufficient ETH balance in the account
- RPC connection issues
- Gas price too high

To manually deposit after fixing the issue:
```bash
ssh app@agg-mode-hoodi-sender
export PATH=$HOME/.cargo/bin:$PATH
agg_mode_cli deposit \
  --network hoodi \
  --rpc-url https://aligned-hoodi-rpc-geth.tail665ae.ts.net \
  --private-key <YOUR_PRIVATE_KEY>
```

Then restart the task sender:
```bash
make task_sender_restart ENV=hoodi
```

### General Debugging

**Check Tailscale connectivity:**
```bash
tailscale status
```

**Test SSH access to servers:**
```bash
ssh admin@agg-mode-hoodi-postgres-monitor "echo 'Connection successful'"
ssh app@agg-mode-hoodi-gateway-1 "echo 'Connection successful'"
```

**Verify Ansible inventory:**
```bash
ansible-inventory -i infra/aggregation_mode/ansible/hoodi-inventory.yaml --list
```

## Advanced Usage

### Running Individual Playbooks

You can run any playbook directly with ansible-playbook:

```bash
# Deploy only postgres monitor
ansible-playbook infra/aggregation_mode/ansible/playbooks/pg_monitor.yaml \
  -i infra/aggregation_mode/ansible/hoodi-inventory.yaml \
  -e "host=postgres_monitor" \
  -e "env=hoodi"

# Deploy only gateway (no poller)
ansible-playbook infra/aggregation_mode/ansible/playbooks/gateway.yaml \
  -i infra/aggregation_mode/ansible/hoodi-inventory.yaml \
  -e "host=gateway_primary" \
  -e "env=hoodi"

# Deploy gateway with forced rebuild
ansible-playbook infra/aggregation_mode/ansible/playbooks/gateway.yaml \
  -i infra/aggregation_mode/ansible/hoodi-inventory.yaml \
  -e "host=gateway_primary" \
  -e "env=hoodi" \
  -e "force_rebuild=true"
```

### Updating Services

**Update gateway and poller with latest code:**

The easiest way to update services is using the `FORCE_REBUILD` parameter:

```bash
# Update both primary and secondary
make gateway_deploy ENV=hoodi FORCE_REBUILD=true

# Or update individually
make gateway_primary_deploy ENV=hoodi FORCE_REBUILD=true
make gateway_secondary_deploy ENV=hoodi FORCE_REBUILD=true
```

This will:
1. Pull latest code from the configured branch (staging for hoodi, main for mainnet)
2. Delete existing binaries
3. Rebuild gateway and poller from source
4. Restart the services

**Manual update (alternative):**

If you prefer to update manually:

```bash
# Gateway
ssh app@agg-mode-hoodi-gateway-1
cd ~/repos/gateway/aligned_layer
git pull origin staging
cargo install --path aggregation_mode/gateway --bin gateway --features tls --locked
sudo systemctl restart gateway

# Poller
cd ~/repos/poller/aligned_layer
git pull origin staging
cargo install --path aggregation_mode/payments_poller --bin payments_poller --locked
systemctl --user restart poller
```

### Redeploy with Latest Code

**Idempotent deployment (skip if binary exists):**

```bash
make gateway_deploy ENV=hoodi
```

This pulls the latest code but skips building if the binary already exists. Use this when you only want to update configuration files.

**Force rebuild (always rebuild binaries):**

```bash
make gateway_deploy ENV=hoodi FORCE_REBUILD=true
```

This always rebuilds binaries from the latest code, even if they already exist. Use this when you want to deploy code changes.

### Changing Configuration

1. Update INI files in `playbooks/ini/`
2. Redeploy the affected service:
   ```bash
   make gateway_deploy ENV=hoodi
   # or
   make postgres_deploy ENV=hoodi
   ```

### Rotating Passwords

1. Update password fields in your environment config file (`config-hoodi.ini` or `config-mainnet.ini`):
   - `db_password` (used by postgres, gateway, and poller)
   - `grafana_postgres_password` (separate read-only user)
2. Run password update on PostgreSQL:
   ```bash
   ssh admin@agg-mode-hoodi-postgres-monitor "sudo -u postgres psql -d pg_auto_failover -c \"ALTER USER autoctl_node PASSWORD 'new_password'\""
   ```
3. Redeploy gateway and metrics:
   ```bash
   make gateway_deploy ENV=hoodi
   make metrics_deploy ENV=hoodi
   ```

## File Structure

```
infra/aggregation_mode/ansible/
├── README.md                           # This file
├── hoodi-inventory.yaml                # Hoodi environment inventory
├── mainnet-inventory.yaml              # Mainnet environment inventory
└── playbooks/
    ├── ini/                            # Configuration files
    │   ├── config-hoodi.ini            # Hoodi config (tracked, fill in passwords)
    │   └── config-mainnet.ini          # Mainnet config (tracked, fill in passwords)
    ├── templates/                      # Jinja2 templates
    │   ├── config-files/               # Service config templates
    │   ├── services/                   # Systemd service templates
    │   ├── sudoers/                    # Sudoers templates
    │   ├── prometheus/                 # Prometheus config templates
    │   └── grafana/                    # Grafana config templates
    ├── rust.yaml                       # Rust installation
    ├── pg_autofailover_common.yaml     # PostgreSQL + pg_auto_failover setup
    ├── pg_monitor.yaml                 # PostgreSQL monitor deployment
    ├── pg_node.yaml                    # PostgreSQL node deployment
    ├── postgres_migrations.yaml        # Database migrations
    ├── gateway.yaml                    # Gateway deployment
    ├── poller.yaml                     # Poller deployment
    ├── prometheus_agg_mode.yaml        # Prometheus deployment
    ├── grafana_agg_mode.yaml           # Grafana deployment
    ├── task_sender.yaml                # Task sender deployment
    ├── postgres_cluster.yaml           # Postgres orchestration
    ├── gateway_stack.yaml              # Gateway + poller orchestration
    ├── metrics_stack.yaml              # Metrics orchestration
    └── deploy_all.yaml                 # Full stack orchestration
```

## Security Notes

1. **Passwords**: Config files are tracked in git with empty password fields. Fill in passwords locally. Use `git update-index --assume-unchanged config-*.ini` after filling passwords to prevent accidentally committing them.

2. **Private Keys**: The `task_sender_private_key` field must be filled with a valid Ethereum private key. Never commit this value to git. The playbook sets appropriate permissions (0600) on the environment file.

3. **TLS Certificates**: Keep private keys secure. The playbooks set appropriate permissions (0600).

4. **SSH Access**: All servers are only accessible via Tailscale VPN (100.64.0.0/10).

5. **PostgreSQL**: Uses scram-sha-256 password authentication, not trust mode.

6. **Firewall**: UFW is configured on all servers with deny-by-default policy.

## Support

For issues or questions:
- Check the [Troubleshooting](#troubleshooting) section
- Review logs on the affected server
- Contact the infrastructure team

## References

- [PostgreSQL Auto-Failover Documentation](https://pg-auto-failover.readthedocs.io/)
- [Ansible Documentation](https://docs.ansible.com/)
- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Documentation](https://grafana.com/docs/)
