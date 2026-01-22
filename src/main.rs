mod config;
mod server;
use actix_web::{
    App, HttpResponse, HttpServer, middleware,
    web::{self},
};
use actix_web_prom::PrometheusMetricsBuilder;
use config::SiteConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let site_config = match SiteConfig::load_config() {
        Ok(site_config) => site_config,
        Err(e) => panic!("There was a problem with the site configuration:\n {e:?}"),
    };

    if let Some(repo_url) = &site_config.content_repo_url {
        let git_manager = server::git_content::GitContentManager::new(
            repo_url.clone(),
            site_config.content_dir.clone(),
        );
        if let Err(e) = git_manager.sync() {
            log::warn!("Failed to initialize git content: {}", e);
        }
    }

    let bind = format!("{}:{}", site_config.address, site_config.port);

    //let prometheus = PrometheusMetricsBuilder::new("api")
    //    .endpoint("/metrics")
    //    .build()
    //    .unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            //.wrap(prometheus.clone())
            // Adds SiteConfig for debugging purposes
            .app_data(web::Data::new(site_config.clone()))
            .route("/health", web::get().to(HttpResponse::Ok))
            .service(actix_files::Files::new("/static", "./static"))
            .service(server::index)
            .service(server::blog_post)
            .service(server::about_me)
            .service(server::debug_posts)
            .service(server::get_post_json)
            .service(server::api_health)
            .service(server::update_content)
    });
    server.bind(bind)?.run().await
}
