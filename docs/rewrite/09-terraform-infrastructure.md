# Terraform Infrastructure

## Overview

Terraform configuration for quickly provisioning and destroying Hetzner Cloud infrastructure. This enables a "cattle not pets" approach - spin up the VPS when needed, destroy when not.

---

## File Structure

```
terraform/
├── main.tf                    # Provider configuration
├── variables.tf               # Input variables with validation
├── outputs.tf                 # Useful outputs (IP, SSH command, DNS)
├── firewall.tf                # Hetzner Cloud Firewall rules
├── server.tf                  # VPS server resource with cloud-init
├── ssh.tf                     # SSH key management
├── volume.tf                  # Optional persistent volume
├── cloud-init.yaml            # Server bootstrap template
├── terraform.tfvars.example   # Example values (committed)
└── terraform.tfvars           # Your values (git-ignored)
```

---

## Provider Configuration

```hcl
# terraform/main.tf

terraform {
  required_version = ">= 1.0.0"

  required_providers {
    hcloud = {
      source  = "hetznercloud/hcloud"
      version = "~> 1.45"
    }
  }

  # Local state (simple, good for personal projects)
  # For remote state, see "State Management" section below
}

provider "hcloud" {
  token = var.hcloud_token
}
```

---

## Variables

```hcl
# terraform/variables.tf

# =============================================================================
# Required Variables
# =============================================================================

variable "hcloud_token" {
  description = "Hetzner Cloud API token"
  type        = string
  sensitive   = true
}

variable "ssh_public_key" {
  description = "SSH public key content for server access"
  type        = string
  sensitive   = true
}

# =============================================================================
# Server Configuration
# =============================================================================

variable "server_name" {
  description = "Name of the Hetzner server"
  type        = string
  default     = "vr-www"
}

variable "server_type" {
  description = "Hetzner server type"
  type        = string
  default     = "cx22"

  validation {
    condition = contains([
      "cx22", "cx32", "cx42", "cx52",           # Shared vCPU Intel
      "cpx11", "cpx21", "cpx31", "cpx41", "cpx51",  # Shared vCPU AMD
      "cax11", "cax21", "cax31", "cax41"        # ARM64
    ], var.server_type)
    error_message = "Invalid server type."
  }
}

variable "location" {
  description = "Hetzner datacenter location"
  type        = string
  default     = "fsn1"  # Falkenstein, Germany

  validation {
    condition     = contains(["fsn1", "nbg1", "hel1", "ash", "hil"], var.location)
    error_message = "Invalid location. Choose: fsn1, nbg1, hel1, ash, hil."
  }
}

variable "image" {
  description = "OS image for the server"
  type        = string
  default     = "ubuntu-24.04"
}

variable "backups" {
  description = "Enable automatic backups (+20% server cost)"
  type        = bool
  default     = false
}

# =============================================================================
# Optional Volume
# =============================================================================

variable "enable_volume" {
  description = "Create and attach a persistent volume"
  type        = bool
  default     = false
}

variable "volume_size" {
  description = "Size of the persistent volume in GB"
  type        = number
  default     = 10

  validation {
    condition     = var.volume_size >= 10 && var.volume_size <= 10000
    error_message = "Volume size must be between 10 and 10000 GB."
  }
}

# =============================================================================
# Network Configuration
# =============================================================================

variable "ssh_allowed_ips" {
  description = "IP addresses/CIDRs allowed to SSH"
  type        = list(string)
  default     = ["0.0.0.0/0", "::/0"]
}

# =============================================================================
# Application Configuration
# =============================================================================

variable "app_user" {
  description = "Non-root user for running the application"
  type        = string
  default     = "www"
}

variable "app_directory" {
  description = "Directory for application deployment"
  type        = string
  default     = "/opt/www"
}

variable "domain" {
  description = "Primary domain for the website"
  type        = string
  default     = "valentinrogg.de"
}

# =============================================================================
# Labels
# =============================================================================

variable "labels" {
  description = "Labels to apply to all resources"
  type        = map(string)
  default = {
    project     = "vr-www"
    environment = "production"
    managed_by  = "terraform"
  }
}
```

---

## SSH Key

```hcl
# terraform/ssh.tf

resource "hcloud_ssh_key" "main" {
  name       = "${var.server_name}-key"
  public_key = var.ssh_public_key
  labels     = var.labels
}
```

---

## Firewall

```hcl
# terraform/firewall.tf

resource "hcloud_firewall" "main" {
  name   = "${var.server_name}-firewall"
  labels = var.labels

  # SSH
  rule {
    description = "SSH"
    direction   = "in"
    protocol    = "tcp"
    port        = "22"
    source_ips  = var.ssh_allowed_ips
  }

  # HTTP (required for Let's Encrypt)
  rule {
    description = "HTTP"
    direction   = "in"
    protocol    = "tcp"
    port        = "80"
    source_ips  = ["0.0.0.0/0", "::/0"]
  }

  # HTTPS
  rule {
    description = "HTTPS"
    direction   = "in"
    protocol    = "tcp"
    port        = "443"
    source_ips  = ["0.0.0.0/0", "::/0"]
  }

  # ICMP (ping)
  rule {
    description = "ICMP"
    direction   = "in"
    protocol    = "icmp"
    source_ips  = ["0.0.0.0/0", "::/0"]
  }

  # Outbound - allow all
  rule {
    description     = "Outbound TCP"
    direction       = "out"
    protocol        = "tcp"
    port            = "any"
    destination_ips = ["0.0.0.0/0", "::/0"]
  }

  rule {
    description     = "Outbound UDP"
    direction       = "out"
    protocol        = "udp"
    port            = "any"
    destination_ips = ["0.0.0.0/0", "::/0"]
  }

  rule {
    description     = "Outbound ICMP"
    direction       = "out"
    protocol        = "icmp"
    destination_ips = ["0.0.0.0/0", "::/0"]
  }
}
```

---

## Volume (Optional)

```hcl
# terraform/volume.tf

resource "hcloud_volume" "data" {
  count = var.enable_volume ? 1 : 0

  name     = "${var.server_name}-data"
  size     = var.volume_size
  location = var.location
  format   = "ext4"
  labels   = var.labels

  lifecycle {
    prevent_destroy = true
  }
}

resource "hcloud_volume_attachment" "data" {
  count = var.enable_volume ? 1 : 0

  volume_id = hcloud_volume.data[0].id
  server_id = hcloud_server.main.id
  automount = true
}
```

---

## Server

```hcl
# terraform/server.tf

resource "hcloud_server" "main" {
  name        = var.server_name
  server_type = var.server_type
  location    = var.location
  image       = var.image
  backups     = var.backups
  labels      = var.labels

  ssh_keys = [hcloud_ssh_key.main.id]

  firewall_ids = [hcloud_firewall.main.id]

  user_data = templatefile("${path.module}/cloud-init.yaml", {
    app_user       = var.app_user
    app_directory  = var.app_directory
    domain         = var.domain
    enable_volume  = var.enable_volume
    volume_device  = var.enable_volume ? "/dev/disk/by-id/scsi-0HC_Volume_${hcloud_volume.data[0].id}" : ""
    ssh_public_key = var.ssh_public_key
  })

  depends_on = [hcloud_firewall.main]

  lifecycle {
    ignore_changes = [user_data, image]
  }
}
```

---

## Cloud-Init

```yaml
#cloud-config
# terraform/cloud-init.yaml

package_update: true
package_upgrade: true
packages:
  - curl
  - git
  - ufw
  - fail2ban
  - sqlite3
  - gnupg
  - unattended-upgrades

users:
  - name: ${app_user}
    groups: docker, sudo
    shell: /bin/bash
    sudo: ['ALL=(ALL) NOPASSWD:ALL']
    ssh_authorized_keys:
      - ${ssh_public_key}

write_files:
  # Docker daemon config
  - path: /etc/docker/daemon.json
    content: |
      {
        "log-driver": "json-file",
        "log-opts": { "max-size": "10m", "max-file": "3" },
        "storage-driver": "overlay2"
      }

  # SSH hardening
  - path: /etc/ssh/sshd_config.d/hardening.conf
    content: |
      PermitRootLogin no
      PasswordAuthentication no
      ChallengeResponseAuthentication no
      MaxAuthTries 3
      LoginGraceTime 20
      ClientAliveInterval 300
      ClientAliveCountMax 2

  # Fail2ban config
  - path: /etc/fail2ban/jail.local
    content: |
      [DEFAULT]
      bantime = 1h
      findtime = 10m
      maxretry = 5

      [sshd]
      enabled = true
      port = ssh
      maxretry = 3
      bantime = 24h

  # Environment template
  - path: ${app_directory}/.env.template
    owner: ${app_user}:${app_user}
    content: |
      # Database
      DATABASE_URL=/data/app.db
      
      # SMTP (configure for your provider)
      SMTP_HOST=
      SMTP_PORT=465
      SMTP_USER=
      SMTP_PASS=
      
      # Friendly Captcha
      FRIENDLY_CAPTCHA_SECRET=
      FRIENDLY_CAPTCHA_SITEKEY=
      
      # Email
      EMAIL_FROM=noreply@${domain}
      EMAIL_TO=mail@${domain}
      
      # App
      BASE_URL=https://${domain}
      RUST_LOG=info,vr_www_api=debug

runcmd:
  # Install Docker
  - curl -fsSL https://get.docker.com | sh
  - systemctl enable docker
  - systemctl start docker
  - usermod -aG docker ${app_user}

  # Create app directories
  - mkdir -p ${app_directory}/{data,backups,crowdsec}
  - chown -R ${app_user}:${app_user} ${app_directory}

%{ if enable_volume }
  # Mount volume
  - mkdir -p /mnt/data
  - echo '${volume_device} /mnt/data ext4 defaults,nofail 0 2' >> /etc/fstab
  - mount -a
  - chown -R ${app_user}:${app_user} /mnt/data
  - ln -sf /mnt/data ${app_directory}/data
%{ endif }

  # Configure UFW
  - ufw default deny incoming
  - ufw default allow outgoing
  - ufw allow ssh
  - ufw allow http
  - ufw allow https
  - ufw --force enable

  # Start fail2ban
  - systemctl enable fail2ban
  - systemctl start fail2ban

  # Restart SSH
  - systemctl restart sshd

  # Set timezone
  - timedatectl set-timezone Europe/Berlin

final_message: "Cloud-init complete for ${domain}"
```

---

## Outputs

```hcl
# terraform/outputs.tf

output "ipv4_address" {
  description = "Public IPv4 address"
  value       = hcloud_server.main.ipv4_address
}

output "ipv6_address" {
  description = "Public IPv6 address"
  value       = hcloud_server.main.ipv6_address
}

output "ssh_command" {
  description = "SSH command to connect"
  value       = "ssh ${var.app_user}@${hcloud_server.main.ipv4_address}"
}

output "dns_records" {
  description = "DNS records for deSEC"
  value = {
    a_record     = "${var.domain} A ${hcloud_server.main.ipv4_address}"
    a_record_www = "www.${var.domain} A ${hcloud_server.main.ipv4_address}"
    aaaa_record  = "${var.domain} AAAA ${hcloud_server.main.ipv6_address}"
  }
}

output "deployment_steps" {
  description = "Next steps after provisioning"
  value = <<-EOT
    
    Server provisioned: ${hcloud_server.main.ipv4_address}
    
    Next steps:
    1. Wait ~2min for cloud-init: ssh ${var.app_user}@${hcloud_server.main.ipv4_address} 'tail -f /var/log/cloud-init-output.log'
    2. Configure DNS in deSEC: ${var.domain} A ${hcloud_server.main.ipv4_address}
    3. Deploy: scp docker-compose.yml Caddyfile ${var.app_user}@${hcloud_server.main.ipv4_address}:${var.app_directory}/
    4. Configure: ssh ${var.app_user}@${hcloud_server.main.ipv4_address} 'cd ${var.app_directory} && cp .env.template .env && nano .env'
    5. Start: ssh ${var.app_user}@${hcloud_server.main.ipv4_address} 'cd ${var.app_directory} && docker compose up -d'
  EOT
}

output "volume_id" {
  description = "Volume ID (if enabled)"
  value       = var.enable_volume ? hcloud_volume.data[0].id : null
}
```

---

## Example Variables

```hcl
# terraform/terraform.tfvars.example
# Copy to terraform.tfvars and fill in values

# Required
hcloud_token   = "your-hetzner-cloud-api-token"
ssh_public_key = "ssh-ed25519 AAAA... you@example.com"

# Optional - server config
server_name = "vr-www"
server_type = "cx22"       # ~4 EUR/month
location    = "fsn1"       # Falkenstein, Germany
backups     = false        # +20% cost if enabled

# Optional - volume
enable_volume = false
volume_size   = 10

# Optional - restrict SSH to your IP
# ssh_allowed_ips = ["YOUR_IP/32"]

# Optional - app settings
app_user      = "www"
app_directory = "/opt/www"
domain        = "valentinrogg.de"
```

---

## Usage

### Initial Setup

```bash
cd terraform

# Configure
cp terraform.tfvars.example terraform.tfvars
nano terraform.tfvars  # Fill in values

# Initialize
terraform init

# Preview
terraform plan

# Create
terraform apply
```

### Quick Provision/Destroy

```bash
# Provision
terraform apply -auto-approve

# Destroy (when not needed)
terraform destroy -auto-approve
```

### Get Server Info

```bash
# All outputs
terraform output

# Specific values
terraform output -raw ipv4_address
terraform output -raw ssh_command
```

---

## State Management

### Local State (Default)

Simple, no setup. State stored in `terraform.tfstate`.

**Important:** Back up state file separately. Add to `.gitignore`:
```gitignore
terraform/*.tfstate
terraform/*.tfstate.*
```

### Remote State (Optional)

For collaboration or extra safety, use Terraform Cloud (free):

```hcl
# Add to main.tf
terraform {
  cloud {
    organization = "your-org"
    workspaces {
      name = "vr-www-production"
    }
  }
}
```

Then run `terraform login`.

---

## Cost Summary

| Configuration | EUR/month |
|---------------|-----------|
| CX22 only | ~4.00 |
| + 10GB Volume | ~4.50 |
| + Backups | ~4.80 |
| + Volume + Backups | ~5.30 |

---

## Security Features

**Network:**
- Hetzner Cloud Firewall (before traffic reaches VPS)
- UFW firewall (host-level backup)
- Only SSH/HTTP/HTTPS open

**SSH:**
- Key-only authentication
- Root login disabled
- Fail2ban for brute-force protection

**Updates:**
- Automatic security updates via unattended-upgrades
