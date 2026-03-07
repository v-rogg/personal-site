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
  value       = <<-EOT

    Server provisioned: ${hcloud_server.main.ipv4_address}

    Next steps:
    1. Wait ~2min for cloud-init: ssh ${var.app_user}@${hcloud_server.main.ipv4_address} 'tail -f /var/log/cloud-init-output.log'
    2. Configure DNS in deSEC: ${var.domain} A ${hcloud_server.main.ipv4_address}
    3. Deploy: scp docker-compose.yml Caddyfile ${var.app_user}@${hcloud_server.main.ipv4_address}:${var.app_directory}/
    4. Configure: ssh ${var.app_user}@${hcloud_server.main.ipv4_address} 'cd ${var.app_directory} && cp .env.template .env && nano .env'
    5. Start: ssh ${var.app_user}@${hcloud_server.main.ipv4_address} 'cd ${var.app_directory} && docker compose up -d'
  EOT
}
