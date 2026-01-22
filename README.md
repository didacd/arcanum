# 🎨 Arcanum - Blogging made in Rust

> [!tldr] Blogging made with _Rust_

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
