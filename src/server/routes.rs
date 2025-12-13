use actix_web::{HttpResponse, Responder, get, http::header::ContentType, web};

use std::path::Path;

use super::content_renderer::read_markdown_files;
use super::templates::{HomeTemplate, PostTemplate};

use crate::SiteConfig;
use askama::Template;

#[get("/")]
async fn index(site_config: web::Data<SiteConfig>) -> impl Responder {
    let blog_path = format!("{}blog", site_config.content_dir);
    let mut pages = read_markdown_files(Path::new(&blog_path)).unwrap_or_default();

    // Sort posts by date (newest first)
    pages.sort_by(|a, b| b.metadata.created.cmp(&a.metadata.created));

    // Try to load homepage content separately
    let homepage_html = match read_markdown_files(Path::new(&site_config.content_dir)) {
        Ok(root_pages) => {
            if let Some(pos) = root_pages.iter().position(|p| p.filename == "homepage") {
                root_pages[pos].rendered_html.clone()
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
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
        Err(e) => HttpResponse::InternalServerError().body(format!("Template render error: {e}")),
    }
}

#[get("/about-me")]
async fn about_me(site_config: web::Data<SiteConfig>) -> impl Responder {
    let pages = read_markdown_files(Path::new(&site_config.content_dir));

    match pages {
        Ok(list) => {
            if let Some(page) = list.into_iter().find(|p| p.filename == "About me") {
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
                    Err(e) => HttpResponse::InternalServerError()
                        .body(format!("Template render error: {e}")),
                }
            } else {
                HttpResponse::NotFound().body("Error 404: About me page not found.")
            }
        }
        Err(e) => {
            eprintln!("Error reading markdown files: {:?}", e);
            HttpResponse::InternalServerError().body(format!(
                "Failed to read or parse markdown files.\n\nError details: {:#?}",
                e
            ))
        }
    }
}

#[get("/debug")]
async fn page_debug(site_config: web::Data<SiteConfig>) -> impl Responder {
    let pages = read_markdown_files(Path::new(&site_config.content_dir));

    HttpResponse::Ok().body(format!(
        "Site configuration is: {:#?}\nAnd the markdown files config is: {:#?}",
        site_config, pages
    ))
}

#[get("/blog/{entry}")]
async fn blog(path: web::Path<String>, site_config: web::Data<SiteConfig>) -> impl Responder {
    let entry = path.into_inner();
    let location = format!("{}/blog", site_config.content_dir);
    let pages = read_markdown_files(Path::new(&location));

    match pages {
        Ok(list) => {
            let mut pages_list_for_tmpl = list.clone();
            pages_list_for_tmpl.sort_by(|a, b| b.metadata.created.cmp(&a.metadata.created));
            
            if let Some(page) = list.into_iter().find(|p| p.filename == entry) {
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
                    Err(e) => HttpResponse::InternalServerError()
                        .body(format!("Template render error: {e}")),
                }
            } else {
                HttpResponse::NotFound().body(format!("Error 404: Page '{}' not found.", entry))
            }
        }
        Err(e) => {
            eprintln!("Error reading markdown files: {:?}", e);
            HttpResponse::InternalServerError().body(format!(
                "Failed to read or parse markdown files.\n\nError details: {:#?}",
                e
            ))
        }
    }
}
