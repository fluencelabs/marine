variable "image" {
  type        = string
  description = "Docker image to use"
}

job "${job}" {
  region = "fluence"
  datacenters = ["hashistack"]
  namespace = "ci"

  group "cache" {
    network {
      port "db" {
        to = 6379
      }
    }

    task "redis" {
      driver = "docker"

      config {
        image          = "$${var.image}"
        ports          = ["db"]
        auth_soft_fail = true
      }

      resources {
        cpu    = 500
        memory = 256
      }
    }
  }
}
