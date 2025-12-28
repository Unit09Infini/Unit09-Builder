output "unit09_backend_namespace" {
  description = "Namespace where Unit09 backend services are deployed"
  value       = kubernetes_namespace.unit09_backend.metadata[0].name
}

output "unit09_monitoring_namespace" {
  description = "Namespace where monitoring stack is deployed"
  value       = kubernetes_namespace.unit09_monitoring.metadata[0].name
}
