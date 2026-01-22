# 🎨 Arcanum - Self-hosted blog made in _Rust_

## Prerequisites

Testing locally:

1. Install the [Rust toolchain](https://rustup.rs/).
2. Install Bun/nodejs/npm for tailwindcss.
3.

## Using with Kubernetes

Make sure you've created a Kubernetes Secret containing the webhook token
generated for the content directory.

In your Kubernetes deployment YAML, you can now mount your secret and point the
app to it:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: arcanum
spec:
  template:
    spec:
      containers:
        - name: arcanum
          image: arcanum:latest
          env:
            # Tell the app where to read the secret
            - name: WEBHOOK_SECRET_FILE
              value: "/etc/secrets/webhook-token"
          volumeMounts:
            - name: secret-volume
              mountPath: "/etc/secrets"
              readOnly: true
      volumes:
        - name: secret-volume
          secret:
            secretName: my-webhook-secret
```
