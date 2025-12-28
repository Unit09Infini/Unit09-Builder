# Unit09 Monitoring

This folder contains baseline monitoring configuration for Unit09:

- `prometheus.yml` configures Prometheus scrape jobs for the API, worker,
  and scheduler services.
- `grafana-dashboards/unit09-overview.json` defines a basic Grafana
  dashboard with a small set of panels focused on error rates and job
  throughput.

In a real deployment you would typically load `prometheus.yml` into the
Prometheus server or use it as part of a larger configuration managed
through Helm or another deployment tool.
