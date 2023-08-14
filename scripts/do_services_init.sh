# `doctl auth init` to sign in initially
# https://docs.digitalocean.com/support/how-to-troubleshoot-apps-in-app-platform/#review-github-permissions

doctl apps create --spec spec.yaml

# To Update: 
# get ID from `doctl apps list`
# then `doctl apps update <app-id> --spec spec.yaml`