output "helm_release_name" {
  value = helm_release.validator.name
}

output "gke_cluster_name" {
  value = google_container_cluster.libra2.name
}

output "gke_cluster_endpoint" {
  value = google_container_cluster.libra2.endpoint
}

output "gke_cluster_ca_certificate" {
  value = google_container_cluster.libra2.master_auth[0].cluster_ca_certificate
}

output "gke_cluster_workload_identity_config" {
  value = google_container_cluster.libra2.workload_identity_config
}
