use actix_web::{HttpResponse, Responder, get, http::header::ContentType, web};
use std::path::Path;
use log::error;

use crate::server::content_renderer::{read_markdown_files, read_markdown_file};
use crate::server::templates::{HomeTemplate, PostTemplate};
use crate::SiteConfig;
use askama::Template;

#[get("/")]
async fn index(site_config: web::Data<SiteConfig>) -> impl Responder {
    let blog_path = format!("{}blog", site_config.content_dir);
    let mut pages = read_markdown_files(Path::new(&blog_path)).unwrap_or_default();

    // Sort posts by date (newest first)
    pages.sort_by(|a, b| b.metadata.created.cmp(&a.metadata.created));

    // Load homepage content efficiently
    let homepage_html = match read_markdown_file(Path::new(&site_config.content_dir), "homepage") {
        Some(page) => page.rendered_html,
        None => String::new(),
    };

    let tmpl = HomeTemplate {
        site: &site_config,
        homepage_html,
        posts: pages,
    };

    match tmpl.render() {
        Ok(body) => HttpResponse::Ok()
            .insert_header(ContentType::html())
            .body(body),
        Err(e) => {
            error!("Template render error: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

#[get("/about-me")]
async fn about_me(site_config: web::Data<SiteConfig>) -> impl Responder {
    // Only read the specific file we need
    if let Some(page) = read_markdown_file(Path::new(&site_config.content_dir), "About me") {
        let blog_path = format!("{}blog", site_config.content_dir);
        let mut blog_pages = read_markdown_files(Path::new(&blog_path)).unwrap_or_default();
        blog_pages.sort_by(|a, b| b.metadata.created.cmp(&a.metadata.created));

        let tmpl = PostTemplate {
            site: &site_config,
            meta_data: page.metadata,
            post: page.rendered_html,
            posts: blog_pages,
        };
        match tmpl.render() {
            Ok(body) => HttpResponse::Ok()
                .insert_header(ContentType::html())
                .body(body),
            Err(e) => {
                error!("Template render error: {}", e);
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
    } else {
        HttpResponse::NotFound().body("Error 404: About me page not found.")
    }
}

#[get("/blog/{entry}")]
async fn blog_post(path: web::Path<String>, site_config: web::Data<SiteConfig>) -> impl Responder {
    let entry = path.into_inner();
    let location = Path::new(&site_config.content_dir).join("blog");
    
    // Attempt to read the specific file first
    if let Some(page) = read_markdown_file(&location, &entry) {
        // We still need the list of posts for the sidebar/template
        let pages = read_markdown_files(&location);
        
        match pages {
            Ok(list) => {
                let mut pages_list_for_tmpl = list;
                pages_list_for_tmpl.sort_by(|a, b| b.metadata.created.cmp(&a.metadata.created));
                
                let tmpl = PostTemplate {
                    site: &site_config,
                    meta_data: page.metadata,
                    post: page.rendered_html,
                    posts: pages_list_for_tmpl,
                };
                match tmpl.render() {
                    Ok(body) => HttpResponse::Ok()
                        .insert_header(ContentType::html())
                        .body(body),
                    Err(e) => {
                        error!("Template render error: {}", e);
                        HttpResponse::InternalServerError().body("Internal Server Error")
                    }
                }
            }
            Err(e) => {
                error!("Error reading markdown files: {:?}", e);
                HttpResponse::InternalServerError().body("Failed to read content list")
            }
        }
    } else {
        HttpResponse::NotFound().body(format!("Error 404: Page '{}' not found.", entry))
    }
}
