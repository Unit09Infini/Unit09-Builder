variable "kubernetes_host" {
  description = "Kubernetes API server endpoint"
  type        = string
}

variable "kubernetes_cluster_ca_cert" {
  description = "Base64 encoded Kubernetes cluster CA certificate"
  type        = string
}

variable "kubernetes_token" {
  description = "Kubernetes API token"
  type        = string
  sensitive   = true
}

variable "solana_rpc_url" {
  description = "Solana RPC endpoint used by Unit09"
  type        = string
}

variable "unit09_program_id" {
  description = "Unit09 on-chain program ID"
  type        = string
}

variable "grafana_admin_user" {
  description = "Grafana admin user"
  type        = string
  default     = "admin"
}

variable "grafana_admin_password" {
  description = "Grafana admin password"
  type        = string
  sensitive   = true
}
