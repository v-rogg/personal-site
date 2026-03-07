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
      "cx22", "cx32", "cx42", "cx52",
      "cpx11", "cpx21", "cpx31", "cpx41", "cpx51",
      "cax11", "cax21", "cax31", "cax41"
    ], var.server_type)
    error_message = "Invalid server type."
  }
}

variable "location" {
  description = "Hetzner datacenter location"
  type        = string
  default     = "fsn1"

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
