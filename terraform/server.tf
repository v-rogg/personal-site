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
    ssh_public_key = var.ssh_public_key
  })

  depends_on = [hcloud_firewall.main]

  lifecycle {
    ignore_changes = [user_data, image]
  }
}
