# 🎨 Arcanum - Self-hosted blog made in _Rust_

## Prerequisites

Testing locally:

1. Install the [Rust toolchain](https://rustup.rs/).
2. Install Bun/nodejs/npm for tailwindcss.
3.

## 🔐 Setup Webhooks

To enable automatic content updates when you push to your git repository, you
need to configure a webhook shared secret. This ensures that only your git
provider can trigger updates.

### 1. Generate a Secret Token

Generate a strong random string to use as your secret. You can use `openssl` in
your terminal:

```bash
openssl rand -hex 20
```

_Save this output. This will be your `WEBHOOK_SECRET`._

### 2. Configure GitHub (or other provider)

1. Go to your repository settings on GitHub.
2. Navigate to **Settings** > **Webhooks** > **Add webhook**.
3. **Payload URL**: `https://your-blog-domain.com/webhook/update`
4. **Content type**: `application/json`
5. **Secret**: Paste the token you generated in step 1.
6. **Events**: Select "Just the push event".
7. Click **Add webhook**.

### 3. Configure Your Application

#### Option A: Environment Variable (Simple/Local)

Set the `WEBHOOK_SECRET` environment variable directly.

```bash
export WEBHOOK_SECRET="your-generated-token-here"
```

#### Option B: Kubernetes Secrets (Recommended for Production)

Create a Kubernetes secret to store your token securely.

1. **Create the secret:**
   ```bash
   kubectl create secret generic arcanum-webhook-secret --from-literal=webhook-token='your-generated-token-here'
   ```

2. **Update your deployment:** Mount the secret as a file and tell the
   application where to find it using `WEBHOOK_SECRET_FILE`.

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
               # 1. Tell the app where to read the secret file
               - name: WEBHOOK_SECRET_FILE
                 value: "/etc/secrets/webhook-token"
             volumeMounts:
               # 2. Mount the volume to that path
               - name: secret-volume
                 mountPath: "/etc/secrets"
                 readOnly: true
         volumes:
           # 3. Define the volume from the k8s secret
           - name: secret-volume
             secret:
               secretName: arcanum-webhook-secret
               items:
                 - key: webhook-token
                   path: webhook-token
   ```

## Using with Kubernetes
