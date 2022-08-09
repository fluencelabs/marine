{{ with secret "pki/issue/internal" "ttl=5m" "common_name=github-actions.service.consul" }}
{{ .Data.issuing_ca  | writeToFile (env "GITHUB_WORKSPACE" | printf "%s/certs/CA.pem")   "" "" "0644" }}
{{ .Data.certificate | writeToFile (env "GITHUB_WORKSPACE" | printf "%s/certs/cert.pem") "" "" "0644" }}
{{ .Data.private_key | writeToFile (env "GITHUB_WORKSPACE" | printf "%s/certs/key.pem")  "" "" "0644" }}
{{ end }}
