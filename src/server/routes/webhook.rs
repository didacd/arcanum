use actix_web::{post, web, HttpResponse, Responder, HttpRequest};
use crate::SiteConfig;
use crate::server::git_content::GitContentManager;
use log::{info, error, warn};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;

#[post("/webhook/update")]
pub async fn update_content(
    req: HttpRequest,
    payload: web::Bytes,
    site_config: web::Data<SiteConfig>,
) -> impl Responder {
    // 1. Validate Secret if configured
    if let Some(secret) = &site_config.webhook_secret {
        let signature_header = match req.headers().get("X-Hub-Signature-256") {
            Some(h) => h,
            None => {
                warn!("Webhook received without signature header");
                return HttpResponse::Unauthorized().body("Missing signature");
            }
        };

        let signature_str = match signature_header.to_str() {
            Ok(s) => s,
            Err(_) => return HttpResponse::BadRequest().body("Invalid signature header"),
        };

        // Expected format: sha256=...
        let signature_hex = if signature_str.starts_with("sha256=") {
            &signature_str[7..]
        } else {
            return HttpResponse::BadRequest().body("Invalid signature format");
        };

        let mut mac = match Hmac::<Sha256>::new_from_slice(secret.as_bytes()) {
            Ok(m) => m,
            Err(e) => {
                error!("Invalid HMAC secret configuration: {}", e);
                return HttpResponse::InternalServerError().body("Server configuration error");
            }
        };

        mac.update(&payload);

        let expected_signature = hex::encode(mac.finalize().into_bytes());
        
        // Constant time comparison is handled by verify_slice in a real scenario to prevent timing attacks,
        // but simple string comparison here for simplicity or use subtle crate.
        // Hmac::verify_slice uses subtle::ConstantTimeEq.
        
        // Re-calculate MAC for verification
        let mut mac_verify = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac_verify.update(&payload);
        
        if mac_verify.verify_slice(&hex::decode(signature_hex).unwrap_or_default()).is_err() {
             warn!("Webhook signature mismatch. Expected: {}, Got: {}", expected_signature, signature_hex);
             return HttpResponse::Unauthorized().body("Invalid signature");
        }
    }

    // 2. Trigger Git Pull
    if let Some(repo_url) = &site_config.content_repo_url {
         let git_manager = GitContentManager::new(
            repo_url.clone(),
            site_config.content_dir.clone(),
        );

        match web::block(move || git_manager.sync()).await {
            Ok(Ok(_)) => {
                info!("Webhook triggered content update successfully");
                HttpResponse::Ok().body("Content updated")
            },
            Ok(Err(e)) => {
                error!("Failed to update content from webhook: {}", e);
                HttpResponse::InternalServerError().body(format!("Update failed: {}", e))
            },
             Err(e) => {
                error!("Blocking task error: {}", e);
                HttpResponse::InternalServerError().body("Task execution failed")
            }
        }
    } else {
        warn!("Webhook received but CONTENT_REPO_URL is not configured");
        HttpResponse::InternalServerError().body("Content repo not configured")
    }
}
