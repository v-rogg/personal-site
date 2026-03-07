resource "hcloud_ssh_key" "main" {
  name       = "${var.server_name}-key"
  public_key = var.ssh_public_key
  labels     = var.labels
}
