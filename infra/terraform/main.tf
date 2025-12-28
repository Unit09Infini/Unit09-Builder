terraform {
  required_version = ">= 1.5.0"

  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = ">= 2.30.0"
    }
    helm = {
      source  = "hashicorp/helm"
      version = ">= 2.13.0"
    }
  }
}

provider "kubernetes" {
  host                   = var.kubernetes_host
  cluster_ca_certificate = base64decode(var.kubernetes_cluster_ca_cert)
  token                  = var.kubernetes_token
}

provider "helm" {
  kubernetes {
    host                   = var.kubernetes_host
    cluster_ca_certificate = base64decode(var.kubernetes_cluster_ca_cert)
    token                  = var.kubernetes_token
  }
}

resource "kubernetes_namespace" "unit09_backend" {
  metadata {
    name = "unit09-backend"
  }
}

resource "kubernetes_namespace" "unit09_monitoring" {
  metadata {
    name = "unit09-monitoring"
  }
}

resource "kubernetes_secret" "unit09_config" {
  metadata {
    name      = "unit09-config"
    namespace = kubernetes_namespace.unit09_backend.metadata[0].name
  }

  data = {
    solanaRpcUrl = var.solana_rpc_url
    programId    = var.unit09_program_id
  }
}

resource "kubernetes_config_map" "unit09_engine_config" {
  metadata {
    name      = "unit09-engine-config"
    namespace = kubernetes_namespace.unit09_backend.metadata[0].name
  }

  data = {
    apiBaseUrl    = "http://unit09-api.unit09-backend.svc.cluster.local/api"
    engineBaseUrl = "http://unit09-worker.unit09-backend.svc.cluster.local"
    defaultPipelineMode = "full"
  }
}

resource "kubernetes_config_map" "unit09_worker_config" {
  metadata {
    name      = "unit09-worker-config"
    namespace = kubernetes_namespace.unit09_backend.metadata[0].name
  }

  data = {
    queueUrl                   = "redis://unit09-queue.unit09-backend.svc.cluster.local:6379"
    maxConcurrentJobs          = "4"
    jobVisibilityTimeoutSeconds = "600"
  }
}

resource "helm_release" "kube_prometheus_stack" {
  name       = "kube-prometheus-stack"
  namespace  = kubernetes_namespace.unit09_monitoring.metadata[0].name
  repository = "https://prometheus-community.github.io/helm-charts"
  chart      = "kube-prometheus-stack"
  version    = "56.6.2"

  values = [
    file("${path.module}/values/prometheus-stack-values.yaml")
  ]
}
