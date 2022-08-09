terraform {
  backend "consul" {
    address   = "hashi.fluence.dev:8501"
    scheme    = "https"
    path      = "terraform/ci/marine"
    ca_file   = "../../../certs/CA.pem"
    cert_file = "../../../certs/cert.pem"
    key_file  = "../../../certs/key.pem"
  }
}

variable "branch" {
  type        = string
  description = "Branch name"
}

provider "nomad" {
  address   = "https://hashi.fluence.dev:4646"
  ca_file   = "../../../certs/CA.pem"
  cert_file = "../../../certs/cert.pem"
  key_file  = "../../../certs/key.pem"
}

resource "nomad_job" "app" {
  jobspec = templatefile("${path.module}/job.nomad.tpl.hcl", {
    job = "marine-${var.branch}",
  })

  hcl2 {
    enabled = true
    vars = {
      "image" = "redis:latest"
    }
  }
}
