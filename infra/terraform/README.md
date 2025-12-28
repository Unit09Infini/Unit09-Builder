# Unit09 Terraform

This module provisions the base Kubernetes resources needed for the Unit09
deployment, including:

- Namespaces for backend and monitoring
- Secrets and ConfigMaps required by the services
- A Prometheus + Grafana stack using the kube-prometheus-stack Helm chart

## Usage

```hcl
module "unit09_infra" {
  source = "./infra/terraform"

  kubernetes_host             = var.kubernetes_host
  kubernetes_cluster_ca_cert  = var.kubernetes_cluster_ca_cert
  kubernetes_token            = var.kubernetes_token
  solana_rpc_url              = var.solana_rpc_url
  unit09_program_id           = var.unit09_program_id
  grafana_admin_user          = "admin"
  grafana_admin_password      = var.grafana_admin_password
}
```

Then apply:

```bash
terraform init
terraform apply
```

You can combine this module with additional infrastructure modules to create
a complete cluster (for example, using a managed Kubernetes service).
