use actix_web::{HttpResponse, Responder, get, web};
use std::path::Path;

use crate::SiteConfig;
use crate::server::content_renderer::{read_markdown_file, read_markdown_files};
use serde::Serialize;

#[derive(Serialize)]
struct DebugInfo {
    total_posts: usize,
    posts: Vec<PostInfo>,
}

#[derive(Serialize)]
struct PostInfo {
    title: String,
    filename: String,
    created: String,
}

#[derive(Serialize)]
struct PostDetail {
    title: String,
    filename: String,
    created: String,
    content: String,
}

#[get("/api/posts")]
async fn debug_posts(site_config: web::Data<SiteConfig>) -> impl Responder {
    let location = Path::new(&site_config.content_dir).join("blog");
    let pages = read_markdown_files(&location).unwrap_or_default();

    let post_infos: Vec<PostInfo> = pages
        .into_iter()
        .map(|p| PostInfo {
            title: p.metadata.title,
            filename: p.filename,
            created: p.metadata.created,
        })
        .collect();

    let debug_info = DebugInfo {
        total_posts: post_infos.len(),
        posts: post_infos,
    };

    HttpResponse::Ok().json(debug_info)
}

#[get("/api/post/{post_name}")]
async fn get_post_json(
    path: web::Path<String>,
    site_config: web::Data<SiteConfig>,
) -> impl Responder {
    let post_name = path.into_inner();
    let location = Path::new(&site_config.content_dir).join("blog");

    if let Some(page) = read_markdown_file(&location, &post_name) {
        let post_detail = PostDetail {
            title: page.metadata.title,
            filename: page.filename,
            created: page.metadata.created,
            content: page.rendered_html,
        };
        HttpResponse::Ok().json(post_detail)
    } else {
        HttpResponse::NotFound().json("Post not found")
    }
}
